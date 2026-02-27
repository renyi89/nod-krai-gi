use crate::script_lib::{BevyScriptLib, ScriptLib};
use bevy_ecs::prelude::*;
use nod_krai_gi_data::scene::scene_block_template::BlockGroup;
use nod_krai_gi_data::scene::scene_config_template::{BlockRect, SceneConfigTemplate};
use nod_krai_gi_data::scene::script_cache::SCENE_GROUP_COLLECTION;
use nod_krai_gi_data::scene::Position;
use nod_krai_gi_event::combat::PlayerMoveEvent;
use std::{
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
};

struct DistanceSq {
    load_distance_sq: f32,
    unload_distance_sq: f32,
}

impl Default for DistanceSq {
    fn default() -> Self {
        Self {
            load_distance_sq: 80.0 * 80.0,
            unload_distance_sq: 120.0 * 120.0,
        }
    }
}

static DISTANCE_SQ_MAP: std::sync::LazyLock<HashMap<u32, DistanceSq>> =
    std::sync::LazyLock::new(|| {
        let mut m = HashMap::new();

        m.insert(0, DistanceSq::default());
        m.insert(
            999999,
            DistanceSq {
                load_distance_sq: 720.0 * 720.0,
                unload_distance_sq: 1080.0 * 1080.0,
            },
        );

        m
    });

#[derive(Resource)]
pub struct GroupLoadManager {
    pub groups: HashSet<u32>,
    pub player_groups: HashMap<u32, HashSet<u32>>,
    pub last_load: HashMap<u32, Instant>,
    pub cd: Duration,
}

impl Default for GroupLoadManager {
    fn default() -> Self {
        Self {
            groups: HashSet::new(),
            player_groups: HashMap::new(),
            last_load: HashMap::new(),
            cd: Duration::from_millis(3000),
        }
    }
}

fn is_in_block(pos: &Position, rect: &BlockRect) -> bool {
    pos.x >= rect.min.x && pos.x <= rect.max.x && pos.z >= rect.min.z && pos.z <= rect.max.z
}

fn get_player_blocks(cfg: &SceneConfigTemplate, pos: &Position) -> Vec<u32> {
    cfg.blocks
        .iter()
        .zip(cfg.block_rects.iter())
        .filter(|(_, rect)| is_in_block(pos, rect))
        .map(|(block_id, _)| *block_id)
        .collect()
}

fn get_groups_in_blocks(
    scene_id: u32,
    blocks: &[u32],
    group_block_map: &mut HashMap<u32, u32>,
) -> Vec<BlockGroup> {
    let scene_block_collection_clone = std::sync::Arc::clone(
        nod_krai_gi_data::scene::script_cache::SCENE_BLOCK_COLLECTION
            .get()
            .unwrap(),
    );

    let mut groups = Vec::new();

    for block_id in blocks {
        let Some(block) = scene_block_collection_clone.get(&(scene_id, *block_id)) else {
            continue;
        };

        let block = block.clone();

        block.groups.iter().for_each(|group| {
            group_block_map.insert(group.id, *block_id);
        });
        groups.extend(block.groups);
    }
    groups
}

fn get_group_position(group_id: u32) -> (u32, u32, Position) {
    let none_pos = Position {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let scene_group_collection_clone = std::sync::Arc::clone(SCENE_GROUP_COLLECTION.get().unwrap());

    let scene_block_collection_clone = std::sync::Arc::clone(
        nod_krai_gi_data::scene::script_cache::SCENE_BLOCK_COLLECTION
            .get()
            .unwrap(),
    );

    match scene_group_collection_clone.get(&group_id) {
        None => (0, 0, none_pos),
        Some(scene_group_template) => match scene_group_template.value() {
            None => (0, 0, none_pos),
            Some(scene_group_template) => {
                let scene_id = scene_group_template.base_info.scene_id;
                let block_id = scene_group_template.base_info.block_id;
                let Some(block) = scene_block_collection_clone.get(&(scene_id, block_id)) else {
                    return (0, 0, none_pos);
                };
                let Some(block_group) = block.groups.iter().find(|g| g.id == group_id) else {
                    return (0, 0, none_pos);
                };
                (
                    scene_id,
                    block_group.refresh_id.unwrap_or_default(),
                    block_group.pos.clone(),
                )
            }
        },
    }
}

pub fn on_player_move(
    mut events: MessageReader<PlayerMoveEvent>,
    mut group_load_manager: ResMut<GroupLoadManager>,
    script_lib: Res<BevyScriptLib>,
) {
    for ev in events.read() {
        tracing::trace!(
            "PlayerMoveEvent: uid={}, scene={}, pos={:?}",
            ev.0,
            ev.1,
            ev.2
        );

        let uid = ev.0;
        let scene_id = ev.1;
        let pos = Position::from(ev.2);
        let allow_load = ev.3;

        let now = Instant::now();

        let allow = allow_load
            || group_load_manager
                .last_load
                .get(&uid)
                .map(|t| now.duration_since(*t) >= group_load_manager.cd)
                .unwrap_or(true);

        if !allow {
            continue;
        }

        group_load_manager.last_load.insert(uid, now);

        let scene_config_collection_clone = std::sync::Arc::clone(
            nod_krai_gi_data::scene::script_cache::SCENE_CONFIG_COLLECTION
                .get()
                .unwrap(),
        );

        let blocks = {
            match scene_config_collection_clone.get(&scene_id) {
                None => vec![],
                Some(scene_cfg) => get_player_blocks(scene_cfg, &pos),
            }
        };

        let mut group_block_map = HashMap::new();
        let now_block_groups = get_groups_in_blocks(scene_id, &blocks, &mut group_block_map);

        let load_target: HashSet<u32> = now_block_groups
            .iter()
            .filter(|g| {
                !group_load_manager.groups.contains(&g.id)
                    && g.pos.distance_squared(&pos)
                        < DISTANCE_SQ_MAP
                            .get(&g.refresh_id.unwrap_or_default())
                            .unwrap_or(&DistanceSq::default())
                            .load_distance_sq
            })
            .map(|g| g.id)
            .collect();

        let mut other_player_groups: HashSet<u32> = HashSet::new();

        for (player_uid, player_group_set) in group_load_manager.player_groups.iter() {
            if *player_uid != uid {
                other_player_groups.extend(player_group_set);
            }
        }

        if !group_load_manager.player_groups.contains_key(&uid) {
            group_load_manager.player_groups.insert(uid, HashSet::new());
        }

        let unload_target: HashSet<u32> = group_load_manager
            .groups
            .iter()
            .filter(|group_id| {
                !other_player_groups.contains(*group_id) && {
                    let (this_scene_id, refresh_id, this_position) = get_group_position(**group_id);
                    this_scene_id != scene_id
                        || this_position.distance_squared(&pos)
                            > DISTANCE_SQ_MAP
                                .get(&refresh_id)
                                .unwrap_or(&DistanceSq::default())
                                .unload_distance_sq
                }
            })
            .map(|id| *id)
            .collect();

        for group_id in load_target {
            match group_block_map.get(&group_id) {
                None => {}
                Some(block_id) => {
                    tracing::debug!(
                        ">>> Loading scene {} block {} group {} ",
                        scene_id,
                        block_id,
                        group_id,
                    );
                    group_load_manager.groups.insert(group_id);
                    group_load_manager
                        .player_groups
                        .get_mut(&uid)
                        .unwrap()
                        .insert(group_id);
                    script_lib.load_group(scene_id, *block_id, group_id);
                }
            }
        }

        for group_id in unload_target {
            tracing::debug!(">>> Unloading group {}", group_id);
            group_load_manager.groups.remove(&group_id);
            group_load_manager
                .player_groups
                .get_mut(&uid)
                .unwrap()
                .remove(&group_id);
            script_lib.unload_group(group_id);
        }

        tracing::debug!("now groups {:#?}", group_load_manager.groups)
    }
}
