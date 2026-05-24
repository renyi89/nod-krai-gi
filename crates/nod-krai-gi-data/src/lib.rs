pub mod ability;
pub mod config;
pub mod custom;
pub mod dynamic_float;
pub mod excel;
pub mod prop_type;
pub mod quest;
pub mod scene;

pub use dynamic_float::DynamicFloat;

use common::data::RegionConfig;
use common::game_server_config::GameServerConfig;
use common::language::Language;
use common::TomlConfig;
use std::sync::{LazyLock, OnceLock};

pub static GAME_SERVER_CONFIG: LazyLock<GameServerConfig> = LazyLock::new(|| {
    let mut config = GameServerConfig::load_or_create("game-server.toml");
    config.language = Language::from_locale() as u32;
    config
});

pub static REGION_LIST: OnceLock<Vec<RegionConfig>> = OnceLock::new();

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
    fn test_gen_handbook() {
        let _ = excel::load_all("../../assets/ExcelBinOutput");

        let text_map_content = std::fs::read_to_string("../../assets/TextMap/TextMap.json")
            .expect("Failed to read TextMap.json");
        let text_map: std::collections::HashMap<String, String> =
            serde_json::from_str(&text_map_content).expect("Failed to parse TextMap.json");

        let mut output = String::new();
        output.push_str("# Excel Data TextMap Handbook\n\n");

        fn lookup_text(
            textmap: &std::collections::HashMap<String, String>,
            hash: u64,
        ) -> Option<String> {
            textmap.get(&hash.to_string()).cloned()
        }

        output.push_str("## AvatarExcelConfig\n\n");
        output.push_str("| id | name | desc |\n");
        output.push_str("|----|------|------|\n");
        let avatar_data = excel::avatar_excel_config_collection::get();
        let mut avatar_list: Vec<_> = avatar_data.iter().collect();
        avatar_list.sort_by_key(|(id, _)| *id);
        for (id, config) in avatar_list {
            let name = lookup_text(&text_map, config.name_text_map_hash);
            let desc = lookup_text(&text_map, config.desc_text_map_hash);
            if name.is_some() || desc.is_some() {
                output.push_str(&format!(
                    "| {} | {} | {} |\n",
                    id,
                    name.unwrap_or_default(),
                    desc.unwrap_or_default()
                ));
            }
        }

        output.push_str("\n## AvatarFlycloakExcelConfig\n\n");
        output.push_str("| id | name | desc |\n");
        output.push_str("|----|------|------|\n");
        let flycloak_data = excel::avatar_flycloak_excel_config_collection::get();
        let mut flycloak_list: Vec<_> = flycloak_data.iter().collect();
        flycloak_list.sort_by_key(|(id, _)| *id);
        for (id, config) in flycloak_list {
            let name = lookup_text(&text_map, config.name_text_map_hash);
            let desc = lookup_text(&text_map, config.desc_text_map_hash);
            if name.is_some() || desc.is_some() {
                output.push_str(&format!(
                    "| {} | {} | {} |\n",
                    id,
                    name.unwrap_or_default(),
                    desc.unwrap_or_default()
                ));
            }
        }

        output.push_str("\n## AvatarCostumeExcelConfig\n\n");
        output.push_str("| id | name | desc |\n");
        output.push_str("|----|------|------|\n");
        let costume_data = excel::avatar_costume_excel_config_collection::get();
        let mut costume_list: Vec<_> = costume_data.iter().collect();
        costume_list.sort_by_key(|(id, _)| *id);
        for (id, config) in costume_list {
            let name = lookup_text(&text_map, config.name_text_map_hash);
            let desc = lookup_text(&text_map, config.desc_text_map_hash);
            if name.is_some() || desc.is_some() {
                output.push_str(&format!(
                    "| {} | {} | {} |\n",
                    id,
                    name.unwrap_or_default(),
                    desc.unwrap_or_default()
                ));
            }
        }

        output.push_str("\n## AvatarTraceEffectExcelConfig\n\n");
        output.push_str("| id | name | desc |\n");
        output.push_str("|----|------|------|\n");
        let trace_data = excel::avatar_trace_effect_excel_config_collection::get();
        let mut trace_list: Vec<_> = trace_data.iter().collect();
        trace_list.sort_by_key(|(id, _)| *id);
        for (id, config) in trace_list {
            let name = lookup_text(&text_map, config.name_text_map_hash);
            let desc = lookup_text(&text_map, config.desc_text_map_hash);
            if name.is_some() || desc.is_some() {
                output.push_str(&format!(
                    "| {} | {} | {} |\n",
                    id,
                    name.unwrap_or_default(),
                    desc.unwrap_or_default()
                ));
            }
        }

        output.push_str("\n## AvatarSkillExcelConfig\n\n");
        output.push_str("| id | name | desc |\n");
        output.push_str("|----|------|------|\n");
        let skill_data = excel::avatar_skill_excel_config_collection::get();
        let mut skill_list: Vec<_> = skill_data.iter().collect();
        skill_list.sort_by_key(|(id, _)| *id);
        for (id, config) in skill_list {
            let name = lookup_text(&text_map, config.name_text_map_hash);
            let desc = lookup_text(&text_map, config.desc_text_map_hash);
            if name.is_some() || desc.is_some() {
                output.push_str(&format!(
                    "| {} | {} | {} |\n",
                    id,
                    name.unwrap_or_default(),
                    desc.unwrap_or_default()
                ));
            }
        }

        output.push_str("\n## AvatarTalentExcelConfig\n\n");
        output.push_str("| id | name | desc |\n");
        output.push_str("|----|------|------|\n");
        let talent_data = excel::avatar_talent_excel_config_collection::get();
        let mut talent_list: Vec<_> = talent_data.iter().collect();
        talent_list.sort_by_key(|(id, _)| *id);
        for (id, config) in talent_list {
            let name = lookup_text(&text_map, config.name_text_map_hash);
            let desc = lookup_text(&text_map, config.desc_text_map_hash);
            if name.is_some() || desc.is_some() {
                output.push_str(&format!(
                    "| {} | {} | {} |\n",
                    id,
                    name.unwrap_or_default(),
                    desc.unwrap_or_default()
                ));
            }
        }

        output.push_str("\n## ProudSkillExcelConfig\n\n");
        output.push_str("| id | name | desc |\n");
        output.push_str("|----|------|------|\n");
        let proud_skill_data = excel::proud_skill_excel_config_collection::get();
        let mut proud_skill_list: Vec<_> = proud_skill_data.iter().collect();
        proud_skill_list.sort_by_key(|(id, _)| *id);
        for (id, config) in proud_skill_list {
            let name = lookup_text(&text_map, config.name_text_map_hash);
            let desc = lookup_text(&text_map, config.desc_text_map_hash);
            if name.is_some() || desc.is_some() {
                output.push_str(&format!(
                    "| {} | {} | {} |\n",
                    id,
                    name.unwrap_or_default(),
                    desc.unwrap_or_default()
                ));
            }
        }

        output.push_str("\n## MaterialExcelConfig\n\n");
        output.push_str("| id | name | desc |\n");
        output.push_str("|----|------|------|\n");
        let material_data = excel::material_excel_config_collection::get();
        let mut material_list: Vec<_> = material_data.iter().collect();
        material_list.sort_by_key(|(id, _)| *id);
        for (id, config) in material_list {
            let name = lookup_text(&text_map, config.name_text_map_hash);
            let desc = lookup_text(&text_map, config.desc_text_map_hash);
            if name.is_some() || desc.is_some() {
                output.push_str(&format!(
                    "| {} | {} | {} |\n",
                    id,
                    name.unwrap_or_default(),
                    desc.unwrap_or_default()
                ));
            }
        }

        output.push_str("\n## WeaponExcelConfig\n\n");
        output.push_str("| id | name | desc |\n");
        output.push_str("|----|------|------|\n");
        let weapon_data = excel::weapon_excel_config_collection::get();
        let mut weapon_list: Vec<_> = weapon_data.iter().collect();
        weapon_list.sort_by_key(|(id, _)| *id);
        for (id, config) in weapon_list {
            let name = lookup_text(&text_map, config.name_text_map_hash);
            let desc = lookup_text(&text_map, config.desc_text_map_hash);
            if name.is_some() || desc.is_some() {
                output.push_str(&format!(
                    "| {} | {} | {} |\n",
                    id,
                    name.unwrap_or_default(),
                    desc.unwrap_or_default()
                ));
            }
        }

        output.push_str("\n## ReliquaryExcelConfig\n\n");
        output.push_str("| id | name | desc |\n");
        output.push_str("|----|------|------|\n");
        let reliquary_data = excel::reliquary_excel_config_collection::get();
        let mut reliquary_list: Vec<_> = reliquary_data.iter().collect();
        reliquary_list.sort_by_key(|(id, _)| *id);
        for (id, config) in reliquary_list {
            let name = lookup_text(&text_map, config.name_text_map_hash);
            let desc = lookup_text(&text_map, config.desc_text_map_hash);
            if name.is_some() || desc.is_some() {
                output.push_str(&format!(
                    "| {} | {} | {} |\n",
                    id,
                    name.unwrap_or_default(),
                    desc.unwrap_or_default()
                ));
            }
        }

        output.push_str("\n## GadgetExcelConfig\n\n");
        output.push_str("| id | name |\n");
        output.push_str("|----|------|\n");
        let gadget_data = excel::gadget_excel_config_collection::get();
        let mut gadget_list: Vec<_> = gadget_data.iter().collect();
        gadget_list.sort_by_key(|(id, _)| *id);
        for (id, config) in gadget_list {
            let name = lookup_text(&text_map, config.interact_name_text_map_hash);
            if let Some(name) = name {
                output.push_str(&format!("| {} | {} |\n", id, name));
            }
        }

        output.push_str("\n## MonsterExcelConfig\n\n");
        output.push_str("| id | name |\n");
        output.push_str("|----|------|\n");
        let monster_data = excel::monster_excel_config_collection::get();
        let mut monster_list: Vec<_> = monster_data.iter().collect();
        monster_list.sort_by_key(|(id, _)| *id);
        for (id, config) in monster_list {
            if let Some(ref describe) = config.describe {
                let name = lookup_text(&text_map, describe.name_text_map_hash);
                if let Some(name) = name {
                    output.push_str(&format!("| {} | {} |\n", id, name));
                }
            }
        }

        output.push_str("\n## QuestConfig\n\n");
        if quest::quest_config::QUEST_CONFIG_COLLECTION.get().is_none() {
            quest::quest_config::load_quest_configs_from_bin("../../assets/BinOutput");
        }
        if let Some(quest_data) = quest::quest_config::QUEST_CONFIG_COLLECTION.get() {
            let mut quest_list: Vec<_> = quest_data.iter().collect();
            quest_list.sort_by_key(|(main_id, _)| *main_id);
            for (main_id, config) in quest_list {
                let title = lookup_text(&text_map, config.title_text_map_hash);
                if let Some(ref sub_quests) = config.sub_quests {
                    let mut sub_quest_list: Vec<_> = sub_quests.iter().collect();
                    sub_quest_list.sort_by_key(|sq| sq.sub_id);
                    let has_content = sub_quest_list
                        .iter()
                        .any(|sq| lookup_text(&text_map, sq.desc_text_map_hash).is_some());
                    if title.is_some() || has_content {
                        output.push_str(&format!(
                            "### {} - {}\n\n| sub_id | desc |\n|--------|------|\n",
                            main_id,
                            title.as_deref().unwrap_or("")
                        ));
                        for sub_quest in sub_quest_list {
                            let desc = lookup_text(&text_map, sub_quest.desc_text_map_hash);
                            output.push_str(&format!(
                                "| {} | {} |\n",
                                sub_quest.sub_id,
                                desc.unwrap_or_else(|| "null".to_string())
                            ));
                        }
                        output.push_str("\n");
                    }
                }
            }
        }

        std::fs::write("../../handbook/handbook_id.md", &output)
            .expect("Failed to write handbook_id.md");
        println!("Handbook generated: handbook/handbook_id.md");
    }

    #[test]
    fn test_load_ability() {
        let _ = ability::load_ability_configs_from_bin("../../assets/BinOutput");
    }

    #[test]
    fn test_load_scene_block() {
        let lua_root = "../../assets/lua";

        init_scene_static_templates(format!("{}/scene", lua_root).as_str());
        let scene_block_collection_clone =
            Arc::clone(scene::script_cache::SCENE_BLOCK_COLLECTION.get().unwrap());

        for key in scene_block_collection_clone.keys() {
            if key.0 == 3 {
                println!("Loading scene groups... {:?}", key);
            }
        }
    }

    #[test]
    fn test_load_scene_group() {
        let lua_root = "../../assets/lua";

        load_lua_vm(format!("{}/common", lua_root).as_str());
        let lua = SCENE_LUA_VM.get().unwrap().clone();

        let scene_id = 40501;
        let block_id = 40501;
        let group_id = 240501002;

        let a = load_scene_group(lua_root, &lua, scene_id, block_id, group_id);
        println!("{:?}", a);
    }

    #[test]
    fn test_build_scene_spatial_cache() {
        let lua_root = "../../assets/lua";

        load_lua_vm(format!("{}/common", lua_root).as_str());
        let lua = SCENE_LUA_VM.get().unwrap().clone();

        init_scene_static_templates(format!("{}/scene", lua_root).as_str());

        let cache_dir = Path::new("../../assets/cache");
        if !cache_dir.exists() {
            fs::create_dir_all(cache_dir).expect("Failed to create cache directory");
        }

        for scene_id in [3u32, 5u32, 6u32, 7u32, 11u32] {
            let cache = build_scene_spatial_cache(&lua, scene_id, lua_root, "../../assets/cache");

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
        let lua_root = "../../assets/lua";

        load_lua_vm(format!("{}/common", lua_root).as_str());
        init_scene_static_templates(format!("{}/scene", lua_root).as_str());

        let cache = get_or_init_spatial_cache(scene_id, lua_root, "../../assets/cache/")
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

        let cache2 = get_or_init_spatial_cache(scene_id, lua_root, "../../assets/cache/")
            .expect("Failed to load cache from file");
        assert_eq!(cache.scene_groups.len(), cache2.scene_groups.len());
        println!("Cache reuse verified: {} groups", cache2.scene_groups.len());
    }

    #[test]
    fn test_rtree_query_performance() {
        let scene_id = 40501;
        let cache_path = format!("../../assets/cache/scene_cache_{}.json", scene_id);

        let cache_data =
            common::string_util::read_utf8_no_bom(&cache_path).expect("Failed to read cache file");
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
                let _ = cache.query_nearby_groups_rtree(*pos, 1000.0f32 * 1000.0f32);
            }
        }
        let rtree_time = start.elapsed();
        println!("R-tree query: {:?}", rtree_time);
    }
}
