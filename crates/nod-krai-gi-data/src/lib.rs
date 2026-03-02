pub mod ability;
pub mod config;
pub mod dynamic_float;
pub mod excel;
pub mod prop_type;
pub mod quest;
pub mod scene;

pub use dynamic_float::DynamicFloat;

use common::game_server_config::GameServerConfig;
use common::language::Language;
use common::TomlConfig;
use std::sync::LazyLock;

pub static GAME_SERVER_CONFIG: LazyLock<GameServerConfig> = LazyLock::new(|| {
    let mut config = GameServerConfig::load_or_create("game-server.toml");
    config.language = Language::from_locale() as u32;
    config
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::group_spatial_cache::{
        build_scene_spatial_cache, get_or_init_spatial_cache, has_spatial_cache, GroupSpatialCache,
    };
    use crate::scene::script_cache::{
        init_scene_static_templates, load_lua_vm, load_scene_group, SCENE_LUA_VM,
    };
    use std::fs;
    use std::path::Path;
    use std::sync::Arc;

    #[test]
    fn test_load() {
        let _ = excel::load_all("../../assets/ExcelBinOutput");
    }

    #[test]
    fn test_load_ability() {
        let _ = ability::load_ability_configs_from_bin("../../assets/BinOutput");
    }

    #[test]
    fn test_load_scene_block() {
        init_scene_static_templates("../../assets/lua/scene/");
        let scene_block_collection_clone = Arc::clone(
            scene::script_cache::SCENE_BLOCK_COLLECTION
                .get()
                .unwrap(),
        );

        for key in scene_block_collection_clone.keys() {
            if key.0 == 3 {
                println!("Loading scene groups... {:?}", key);
            }
        }
    }

    #[test]
    fn test_load_scene_group() {
        load_lua_vm("../../assets/lua/common");
        let lua = SCENE_LUA_VM.get().unwrap().clone();

        let scene_id = 3;
        let block_id = 1061;
        let group_id = 131061695;

        let path = format!(
            "../../assets/lua/scene/{}/scene{}_group{}.lua",
            scene_id, scene_id, group_id
        );

        let Ok(code) = fs::read_to_string(&path) else {
            println!("load_scene_group failed read scene {}", path);
            return;
        };

        if let Err(err) = lua
            .load(&code)
            .set_name(&format!("scene{}_group{}", scene_id, group_id))
            .exec()
        {
            println!("failed lua load {}: {}", path, err);
            return;
        }

        let a = load_scene_group(&lua, scene_id, block_id, group_id);
        println!("{:?}", a);
    }

    #[test]
    fn test_build_scene_spatial_cache() {
        load_lua_vm("../../assets/lua/common");
        let lua = SCENE_LUA_VM.get().unwrap().clone();

        init_scene_static_templates("../../assets/lua/scene/");

        let cache_dir = Path::new("../../assets/cache");
        if !cache_dir.exists() {
            fs::create_dir_all(cache_dir).expect("Failed to create cache directory");
        }

        for scene_id in [3u32, 5u32, 6u32, 7u32, 11u32, 101u32] {
            let cache =
                build_scene_spatial_cache(&lua, scene_id, "../../assets/lua", "../../assets/cache");

            if let Some(cache) = cache {
                println!(
                    "Scene {} built successfully with {} groups",
                    scene_id,
                    cache.scene_groups.len()
                );
            } else {
                println!("Scene {} failed to build", scene_id);
            }
        }
    }

    #[test]
    fn test_load_cache_and_query_rtree() {
        let scene_id = 40501;

        load_lua_vm("../../assets/lua/common");
        init_scene_static_templates("../../assets/lua/scene/");

        let cache = get_or_init_spatial_cache(scene_id, "../../assets/lua", "../../assets/cache/")
            .expect("Failed to load cache from file");

        println!(
            "Using cache for scene {} with {} groups (cached: {})",
            scene_id,
            cache.scene_groups.len(),
            has_spatial_cache(scene_id)
        );

        let test_position = [0.0f32, 0.0, 0.0];
        let nearby_group_ids = cache.query_groups_at_position(test_position);
        println!(
            "Found {} groups near position {:?}",
            nearby_group_ids.len(),
            test_position
        );

        for group_id in &nearby_group_ids[..nearby_group_ids.len().min(5)] {
            if let Some(group) = cache.scene_groups.get(group_id) { 
                println!(
                    "  Group {}: center={:?}, range={}",
                    group.group_id, group.center, group.vision_range
                );
            }
        }

        let cache2 = get_or_init_spatial_cache(scene_id, "../../assets/lua", "../../assets/cache/")
            .expect("Failed to load cache from file");
        assert_eq!(
            cache.scene_groups.len(),
            cache2.scene_groups.len()
        );
        println!(
            "Cache reuse verified: {} groups",
            cache2.scene_groups.len()
        );
    }

    #[test]
    fn test_rtree_query_performance() {
        let scene_id = 3;
        let cache_path = format!("../../assets/cache/scene_cache_{}.json", scene_id);

        let cache_data = fs::read_to_string(&cache_path).expect("Failed to read cache file");
        let cache: GroupSpatialCache =
            serde_json::from_str(&cache_data).expect("Failed to parse cache");

        let test_positions: Vec<[f32; 3]> = vec![
            [0.0, 0.0, 0.0],
            [100.0, 0.0, 100.0],
            [-100.0, 50.0, -100.0],
            [500.0, 100.0, 500.0],
            [-500.0, -50.0, -500.0],
        ];

        let start = std::time::Instant::now();
        for _ in 0..100 {
            for pos in &test_positions {
                let _ = cache.query_nearby_groups_rtree( *pos, 1000.0f32 * 1000.0f32);
            }
        }
        let rtree_time = start.elapsed();
        println!("R-tree query: {:?}", rtree_time);
    }
}
