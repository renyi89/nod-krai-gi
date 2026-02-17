use crate::script_lua_vm::LuaRuntime;
use mlua::Function;
use nod_krai_gi_data::scene::scene_group_template::SceneGroupTemplate;
use nod_krai_gi_data::scene::script_cache::{load_scene_group_from_cache, scene_group_is_bad};
use nod_krai_gi_data::scene::{EventType, LuaContext};
use std::collections::HashMap;
use std::fs;

pub struct CompiledTrigger {
    pub name: String,
    pub condition: Option<Function>,
    pub action: Option<Function>,
}

pub enum GroupLoadState {
    Unloaded,
    Loading,
    Loaded(SceneGroupRuntime),
    Failed,
}

pub struct SceneGroupRuntime {
    pub data: SceneGroupTemplate,
    pub context: LuaContext,
    pub triggers_by_event: HashMap<EventType, Vec<CompiledTrigger>>,
    pub active_suites: Vec<u32>,
    pub active_trigger_names: Vec<String>,
}

impl SceneGroupRuntime {
    pub fn new(scene_id: u32, block_id: u32, group_id: u32, lua_vm: LuaRuntime) -> Option<Self> {
        if scene_group_is_bad(group_id) {
            return None;
        }

        let path = format!(
            "./assets/lua/scene/{}/scene{}_group{}.lua",
            scene_id, scene_id, group_id
        );

        let Ok(code) = fs::read_to_string(&path) else {
            println!("load_scene_group failed read scene {}", path);
            return None;
        };

        let code = code.replace("ScriptLib.", "ScriptLib:");

        if let Err(err) = lua_vm
            .lua
            .load(&code)
            .set_name(&format!("scene{}_group{}", scene_id, group_id))
            .exec()
        {
            tracing::debug!("SceneGroupRuntime failed lua load {}: {}", path, err);
            return None;
        }

        let Some(data) = load_scene_group_from_cache(&lua_vm.lua, scene_id, block_id, group_id)
        else {
            tracing::debug!(
                "SceneGroupRuntime load_scene_group_from_cache {} fail",
                group_id
            );
            return None;
        };

        if data.triggers.is_empty() {
            let rt = Self {
                data,
                context: LuaContext {
                    scene_id,
                    group_id,
                    uid: 1234,
                },
                triggers_by_event: HashMap::new(),
                active_suites: vec![1],
                active_trigger_names: Vec::new(),
            };
            return Some(rt);
        };

        let mut triggers_by_event: HashMap<EventType, Vec<CompiledTrigger>> = HashMap::new();

        let globals = lua_vm.lua.globals();

        for trig in &data.triggers {
            let cond = if !trig.condition.is_empty() {
                globals.get::<Function>(trig.condition.as_str()).ok()
            } else {
                None
            };

            let action = if !trig.action.is_empty() {
                globals.get::<Function>(trig.action.as_str()).ok()
            } else {
                None
            };

            if action.is_none() {
                continue;
            }

            tracing::debug!(
                "event {:#?} trig {} ",
                trig.event.clone(),
                trig.name.clone()
            );

            triggers_by_event
                .entry(trig.event.clone())
                .or_default()
                .push(CompiledTrigger {
                    name: trig.name.clone(),
                    condition: cond,
                    action,
                });
        }

        if triggers_by_event.is_empty() {
            tracing::debug!("SceneGroupRuntime {} has no valid triggers", group_id);
            return None;
        }

        let mut rt = Self {
            data,
            context: LuaContext {
                scene_id,
                group_id,
                uid: 1234,
            },
            triggers_by_event,
            active_suites: vec![1],
            active_trigger_names: Vec::new(),
        };

        rt.recompute_active_triggers();
        Some(rt)
    }

    pub fn recompute_active_triggers(&mut self) {
        let mut names = Vec::new();

        for suite_id in &self.active_suites {
            let idx = (*suite_id as usize).saturating_sub(1);
            if let Some(suite) = self.data.suites.get(idx) {
                for name in &suite.triggers {
                    if !names.contains(name) {
                        names.push(name.clone());
                    }
                }
            }
        }

        self.active_trigger_names = names;
    }
}
