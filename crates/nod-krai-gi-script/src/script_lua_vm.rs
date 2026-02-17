use crate::script_lib::BevyScriptLib;
use crate::script_lib_handle::LuaScriptLibHandle;
use bevy_ecs::prelude::*;
use mlua::{Lua, Table};
use nod_krai_gi_data::excel::common::{EntityType, QuestState, VisionLevelType};
use nod_krai_gi_data::scene::{
    inject_enum, ChallengeEventMarkType, EventType, FatherChallengeProperty, GadgetState,
    GroupKillPolicy, RegionShape, SealBattleType,
};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;

#[derive(Resource, Clone)]
pub struct LuaRuntime {
    pub lua: Lua,
}

impl LuaRuntime {
    pub fn new(script_lib: Arc<BevyScriptLib>) -> Self {
        let lua = get_lua(script_lib);

        Self { lua: lua }
    }
}

pub fn get_lua(script_lib: Arc<BevyScriptLib>) -> Lua {
    let lua = Lua::new();

    // all enum
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

    let globals = lua.globals();

    let lib_handle = LuaScriptLibHandle { script_lib };
    let result = globals.set("ScriptLib", lib_handle);

    match result {
        Ok(_) => {}
        Err(err) => {
            tracing::debug!("SceneGroupRuntime init_group_lua_vm  fail {}", err);
            std::process::exit(0);
        }
    }

    let modules = load_lua_directory("./assets/lua/common");

    install_memory_require(&lua, modules);

    lua
}

fn load_lua_directory(root: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    scan_dir(Path::new(root), Path::new(root), &mut map);
    map
}

fn scan_dir(root: &Path, path: &Path, map: &mut HashMap<String, String>) {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            scan_dir(root, &path, map);
        } else if path.extension().and_then(|s| s.to_str()) == Some("lua") {
            let content = fs::read_to_string(&path).unwrap();

            let rel = path.strip_prefix(root).unwrap();
            let module_name = rel
                .to_str()
                .unwrap()
                .trim_end_matches(".lua")
                .replace("\\", "/");

            map.insert(module_name, content);
        }
    }
}

fn install_memory_require(lua: &Lua, modules: HashMap<String, String>) {
    let package: Table = lua.globals().get("package").unwrap();
    let preload: Table = package.get("preload").unwrap();

    modules.into_iter().for_each(|(module_name, code)| {
        let code = code.replace("ScriptLib.", "ScriptLib:");
        let loader = lua
            .create_function(move |lua, _: ()| lua.load(&code).into_function())
            .unwrap();

        preload.set(module_name, loader).unwrap();
    });
}
