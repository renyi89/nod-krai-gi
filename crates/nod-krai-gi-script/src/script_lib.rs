use bevy_ecs::prelude::*;
use crossbeam_queue::SegQueue;
use mlua::{Function, Table};
use nod_krai_gi_data::scene::{LuaContext, LuaEvt};
use std::sync::Arc;

pub enum ScriptCommand {
    AddExtraGroupSuite {
        ctx: Table,
        group_id: u32,
        suite_id: u32,
    },
    RemoveExtraGroupSuite {
        ctx: Table,
        group_id: u32,
        suite_id: u32,
    },

    LoadGroup {
        scene_id: u32,
        block_id: u32,
        group_id: u32,
    },

    UnloadGroup {
        group_id: u32,
    },
}

pub trait ScriptLib: Send + Sync + 'static {
    fn get_group_monster_count(&self, ctx: Table) -> i32;

    fn add_extra_group_suite(&self, ctx: Table, group_id: u32, suite_id: u32);
    fn remove_extra_group_suite(&self, ctx: Table, group_id: u32, suite_id: u32);

    // no lua
    fn load_group(&self, scene_id: u32, block_id: u32, group_id: u32);

    fn unload_group(&self, group_id: u32);
}

#[derive(Resource, Clone)]
pub struct BevyScriptLib {
    pub queue: Arc<SegQueue<ScriptCommand>>,
}

impl ScriptLib for BevyScriptLib {
    fn get_group_monster_count(&self, _ctx: Table) -> i32 {
        1
    }

    fn add_extra_group_suite(&self, ctx: Table, group_id: u32, suite_id: u32) {
        self.queue.push(ScriptCommand::AddExtraGroupSuite {
            ctx,
            group_id,
            suite_id,
        });
    }

    fn remove_extra_group_suite(&self, ctx: Table, group_id: u32, suite_id: u32) {
        self.queue.push(ScriptCommand::RemoveExtraGroupSuite {
            ctx,
            group_id,
            suite_id,
        });
    }

    // no lua
    fn load_group(&self, scene_id: u32, block_id: u32, group_id: u32) {
        self.queue.push(ScriptCommand::LoadGroup {
            scene_id,
            block_id,
            group_id,
        });
    }

    fn unload_group(&self, group_id: u32) {
        self.queue.push(ScriptCommand::UnloadGroup { group_id });
    }
}

pub fn call_lua_trigger_condition(
    func: &Function,
    context: LuaContext,
    evt: LuaEvt,
) -> mlua::Result<bool> {
    match func.call((context, evt)) {
        Ok(ret) => Ok(ret),
        Err(e) => {
            tracing::debug!("Lua trigger function returned error: {:?}", e);
            Err(e)
        }
    }
}

pub fn call_lua_trigger_action(
    func: &Function,
    context: LuaContext,
    evt: LuaEvt,
) -> mlua::Result<i32> {
    match func.call((context, evt)) {
        Ok(ret) => Ok(ret),
        Err(e) => {
            tracing::debug!("Lua trigger function returned error: {:?}", e);
            Err(e)
        }
    }
}
