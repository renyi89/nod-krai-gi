use crate::script_lib::BevyScriptLib;
use crate::script_lib_handle::LuaScriptLibHandle;
use bevy_ecs::prelude::*;
use mlua::Lua;
use nod_krai_gi_data::scene::script_cache::SCENE_LUA_VM;
use std::sync::Arc;

#[derive(Resource, Clone)]
pub struct LuaRuntime {
    pub lua: Lua,
}

impl LuaRuntime {
    pub fn new(script_lib: Arc<BevyScriptLib>) -> Self {
        let lua = get_lua(script_lib);

        Self { lua }
    }
}

pub fn get_lua(script_lib: Arc<BevyScriptLib>) -> Lua {
    let lua = SCENE_LUA_VM.get().unwrap().clone();

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

    lua
}
