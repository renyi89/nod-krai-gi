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
    use crate::excel::common::*;
    use crate::scene::script_cache::{init_scene_static_templates, load_scene_group};
    use crate::scene::*;
    use mlua::Lua;
    use std::fs;
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
            crate::scene::script_cache::SCENE_BLOCK_COLLECTION
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

        let lua = Lua::new();

        // 注入所有 enum
        inject_enum::<EventType>(&lua, "EventType").ok().unwrap();
        inject_enum::<GadgetState>(&lua, "GadgetState")
            .ok()
            .unwrap();
        inject_enum::<RegionShape>(&lua, "RegionShape")
            .ok()
            .unwrap();
        inject_enum::<GroupKillPolicy>(&lua, "GroupKillPolicy")
            .ok()
            .unwrap();
        inject_enum::<SealBattleType>(&lua, "SealBattleType")
            .ok()
            .unwrap();
        inject_enum::<FatherChallengeProperty>(&lua, "FatherChallengeProperty")
            .ok()
            .unwrap();
        inject_enum::<ChallengeEventMarkType>(&lua, "ChallengeEventMarkType")
            .ok()
            .unwrap();
        inject_enum::<EntityType>(&lua, "EntityType").ok().unwrap();
        inject_enum::<QuestState>(&lua, "QuestState").ok().unwrap();
        inject_enum::<VisionLevelType>(&lua, "VisionLevelType")
            .ok()
            .unwrap();

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
}
