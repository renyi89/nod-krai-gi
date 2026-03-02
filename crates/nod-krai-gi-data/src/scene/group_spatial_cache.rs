use crate::excel::common::VisionLevelType;
use crate::scene::scene_block_template::BlockGroup;
use crate::scene::scene_group_template::SceneGroupTemplate;
use crate::scene::script_cache::{load_scene_group, SCENE_BLOCK_COLLECTION, SCENE_LUA_VM};
use dashmap::DashMap;
use mlua::Lua;
use rstar::{Envelope, PointDistance, RTree, RTreeObject, AABB};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, OnceLock};

pub static GROUP_SPATIAL_CACHE_COLLECTION: OnceLock<Arc<DashMap<u32, GroupSpatialCache>>> =
    OnceLock::new();

pub fn get_or_init_spatial_cache(
    scene_id: u32,
    lua_root: &str,
    cache_root: &str,
) -> Option<Arc<GroupSpatialCache>> {
    let cache_collection = GROUP_SPATIAL_CACHE_COLLECTION.get_or_init(|| Arc::new(DashMap::new()));

    if let Some(entry) = cache_collection.get(&scene_id) {
        return Some(Arc::new(entry.value().clone()));
    }

    let cache_path = format!("{}/scene_cache_{}.json", cache_root, scene_id);

    if Path::new(&cache_path).exists() {
        let cache_data = fs::read_to_string(&cache_path).ok()?;
        let cache: GroupSpatialCache = serde_json::from_str(&cache_data).ok()?;

        let cache_clone = cache.clone();
        cache_collection.insert(scene_id, cache);

        return Some(Arc::new(cache_clone));
    }

    let lua = SCENE_LUA_VM.get()?.clone();
    let cache = build_scene_spatial_cache(&lua, scene_id, lua_root, cache_root)?;

    let cache_clone = cache.clone();
    cache_collection.insert(scene_id, cache);

    Some(Arc::new(cache_clone))
}

pub fn store_spatial_cache(scene_id: u32, cache: GroupSpatialCache) {
    let cache_collection = GROUP_SPATIAL_CACHE_COLLECTION.get_or_init(|| Arc::new(DashMap::new()));
    cache_collection.insert(scene_id, cache);
}

pub fn has_spatial_cache(scene_id: u32) -> bool {
    let cache_collection = GROUP_SPATIAL_CACHE_COLLECTION.get_or_init(|| Arc::new(DashMap::new()));
    cache_collection.contains_key(&scene_id)
}

pub fn build_scene_spatial_cache(
    lua: &Lua,
    scene_id: u32,
    lua_root: &str,
    cache_root: &str,
) -> Option<GroupSpatialCache> {
    let block_collection = Arc::clone(SCENE_BLOCK_COLLECTION.get()?);

    let mut cache = GroupSpatialCache::new();
    let mut processed_groups = 0;
    let mut failed_groups = 0;

    let scene_blocks: Vec<_> = block_collection
        .iter()
        .filter(|entry| entry.0 .0 == scene_id)
        .collect();

    for entry in scene_blocks {
        let (scene_id_key, block_id) = entry.0;
        let block = entry.1;

        for block_group in &block.groups {
            let group_id = block_group.id;
            let path = format!(
                "{}/scene/{}/scene{}_group{}.lua",
                lua_root, scene_id_key, scene_id_key, group_id
            );

            let Ok(code) = fs::read_to_string(&path) else {
                failed_groups += 1;
                println!("Failed to read group {}", group_id);
                continue;
            };

            if let Err(err) = lua
                .load(&code)
                .set_name(&format!("scene{}_group{}", scene_id_key, group_id))
                .exec()
            {
                println!("Failed to load group {}: {}", group_id, err);
                failed_groups += 1;
                continue;
            }

            if let Some(group) = load_scene_group(&lua, *scene_id_key, *block_id, group_id) {
                if let Some(info) = GroupSpatialInfo::from_scene_group(&group) {
                    cache.add_group(scene_id, info);
                    processed_groups += 1;
                }
            } else {
                failed_groups += 1;
            }
        }
    }

    if !cache.scene_groups.is_empty() {
        let cache_dir = Path::new(cache_root);
        if !cache_dir.exists() {
            fs::create_dir_all(cache_dir).ok();
        }

        let cache_path = format!("{}/scene_cache_{}.json", cache_root, scene_id);
        let json = serde_json::to_string_pretty(&cache).ok()?;
        fs::write(&cache_path, json).ok()?;

        println!(
            "Scene {}: processed {} groups, failed {}, written to {}",
            scene_id, processed_groups, failed_groups, cache_path
        );
    }

    Some(cache)
}

const VISION_LEVEL_DISTANCES: [f32; 6] = [
    80.0,   // Normal = 0
    16.0,   // LittleRemote = 1
    1000.0, // Remote = 2
    4000.0, // Super = 3
    40.0,   // Nearby = 4
    20.0,   // SuperNearby = 5
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupSpatialInfo {
    pub group_id: u32,
    pub block_id: u32,
    pub center: [f32; 3],
    pub vision_range: f32,
}

impl GroupSpatialInfo {
    pub fn from_scene_group(group: &SceneGroupTemplate) -> Option<Self> {
        let mut all_positions = Vec::new();
        let mut max_vision_range = VISION_LEVEL_DISTANCES[VisionLevelType::Normal as usize];

        for monster in &group.monsters {
            all_positions.push([monster.pos.x, monster.pos.y, monster.pos.z]);
        }

        for gadget in &group.gadgets {
            all_positions.push([gadget.pos.x, gadget.pos.y, gadget.pos.z]);
            if let Some(vision_level) = gadget.vision_level {
                let vision_range = VISION_LEVEL_DISTANCES[vision_level as usize];
                if vision_range > max_vision_range {
                    max_vision_range = vision_range;
                }
            }
        }

        for region in &group.regions {
            all_positions.push([region.pos.x, region.pos.y, region.pos.z]);
        }

        if all_positions.is_empty() {
            return None;
        }

        let mut sum = [0.0f32; 3];
        for pos in &all_positions {
            for i in 0..3 {
                sum[i] += pos[i];
            }
        }

        let count = all_positions.len() as f32;
        let center = [sum[0] / count, sum[1] / count, sum[2] / count];

        let mut max_distance_from_center = 0.0f32;
        for pos in &all_positions {
            let dx = pos[0] - center[0];
            let dy = pos[1] - center[1];
            let dz = pos[2] - center[2];
            let distance = (dx * dx + dy * dy + dz * dz).sqrt();
            if distance > max_distance_from_center {
                max_distance_from_center = distance;
            }
        }

        Some(Self {
            group_id: group.base_info.group_id,
            block_id: group.base_info.block_id,
            center,
            vision_range: f32::max(max_vision_range, max_distance_from_center),
        })
    }

    pub fn expanded_bounds(&self) -> ([f32; 3], [f32; 3]) {
        let expanded_min = [
            self.center[0] - self.vision_range,
            self.center[1] - self.vision_range,
            self.center[2] - self.vision_range,
        ];
        let expanded_max = [
            self.center[0] + self.vision_range,
            self.center[1] + self.vision_range,
            self.center[2] + self.vision_range,
        ];
        (expanded_min, expanded_max)
    }
}

#[derive(Debug, Clone)]
pub struct GroupSpatialObject {
    pub info: GroupSpatialInfo,
}

impl RTreeObject for GroupSpatialObject {
    type Envelope = AABB<[f32; 3]>;

    fn envelope(&self) -> Self::Envelope {
        let (min, max) = self.info.expanded_bounds();
        AABB::from_corners(min, max)
    }
}

impl PointDistance for GroupSpatialObject {
    fn distance_2(&self, point: &[f32; 3]) -> f32 {
        let envelope = self.envelope();
        envelope.distance_2(point)
    }

    fn contains_point(&self, point: &[f32; 3]) -> bool {
        let envelope = self.envelope();
        envelope.contains_point(point)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupSpatialCache {
    pub scene_groups: HashMap<u32, GroupSpatialInfo>,
    #[serde(skip)]
    pub rtree_cache: Arc<OnceLock<RTree<GroupSpatialObject>>>,
}

impl Default for GroupSpatialCache {
    fn default() -> Self {
        Self {
            scene_groups: HashMap::new(),
            rtree_cache: Arc::new(OnceLock::new()),
        }
    }
}

impl GroupSpatialCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_group(&mut self, _scene_id: u32, info: GroupSpatialInfo) {
        self.scene_groups.insert(info.group_id, info);
    }

    pub fn get_or_build_rtree(&self) -> &RTree<GroupSpatialObject> {
        self.rtree_cache.get_or_init(|| {
            let objects: Vec<GroupSpatialObject> = self
                .scene_groups
                .values()
                .map(|info| GroupSpatialObject { info: info.clone() })
                .collect();
            RTree::bulk_load(objects)
        })
    }

    pub fn build_rtree(&self) -> RTree<GroupSpatialObject> {
        let objects: Vec<GroupSpatialObject> = self
            .scene_groups
            .values()
            .map(|info| GroupSpatialObject { info: info.clone() })
            .collect();
        RTree::bulk_load(objects)
    }

    pub fn query_nearby_groups_rtree(
        &self,
        position: [f32; 3],
        max_squared_radius: f32,
    ) -> Vec<u32> {
        let rtree = self.get_or_build_rtree();

        rtree
            .locate_within_distance(position, max_squared_radius)
            .filter(|obj| {
                let info = &obj.info;
                let dx = info.center[0] - position[0];
                let dy = info.center[1] - position[1];
                let dz = info.center[2] - position[2];
                let dist_squared = dx * dx + dy * dy + dz * dz;
                let vision_range_squared = info.vision_range * info.vision_range;
                dist_squared <= vision_range_squared
            })
            .map(|obj| obj.info.group_id)
            .collect()
    }

    pub fn query_groups_at_position(&self, position: [f32; 3]) -> Vec<u32> {
        let rtree = self.get_or_build_rtree();

        rtree
            .locate_all_at_point(&position)
            .map(|obj| obj.info.group_id)
            .collect()
    }

    pub fn get_group_info(&self, group_id: u32) -> Option<&GroupSpatialInfo> {
        self.scene_groups.get(&group_id)
    }

    pub fn get_group_count(&self) -> usize {
        self.scene_groups.len()
    }
}

pub fn build_spatial_cache_from_groups(
    _scene_id: u32,
    block_groups: &[BlockGroup],
    group_templates: &HashMap<u32, SceneGroupTemplate>,
) -> Vec<GroupSpatialInfo> {
    let mut infos = Vec::new();

    for block_group in block_groups {
        if let Some(template) = group_templates.get(&block_group.id) {
            if let Some(info) = GroupSpatialInfo::from_scene_group(template) {
                infos.push(info);
            }
        }
    }

    infos
}
