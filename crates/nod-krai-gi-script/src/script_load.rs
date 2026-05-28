use crate::script_lua_vm::LuaRuntime;
use bevy_ecs::change_detection::ResMut;
use mlua::Function;
use nod_krai_gi_data::scene::scene_group_template::SceneGroupTemplate;
use nod_krai_gi_data::scene::script_cache::{load_scene_group_from_cache, scene_group_is_bad};
use nod_krai_gi_data::scene::{EventType, LuaContext};
use std::collections::HashMap;

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
    pub variables: HashMap<String, i32>,
}

impl SceneGroupRuntime {
    pub fn new(
        scene_id: u32,
        block_id: u32,
        group_id: u32,
        lua_vm: &mut ResMut<LuaRuntime>,
    ) -> Option<Self> {
        if scene_group_is_bad(group_id) {
            return None;
        }

        let lua = &lua_vm.lua;
        let globals = lua.globals();

        let lua_root = "./assets/lua";

        let script_name = format!("scene{}_group{}", scene_id, group_id);
        let script_path = format!("{}/scene/{}/{}.lua", lua_root, scene_id, script_name);

        let code = common::string_util::read_utf8_no_bom(&script_path).ok()?;
        let code = code.replace("ScriptLib.", "ScriptLib:");

        let env = lua.create_table().ok()?;
        let mt = lua.create_table().ok()?;
        mt.set("__index", globals.clone()).ok()?;
        env.set_metatable(Some(mt)).ok()?;

        let chunk = lua.load(&code).set_name(&script_name);
        chunk.set_environment(env.clone()).exec().ok()?;

        globals.set(script_name.clone(), env.clone()).ok()?;

        let data = load_scene_group_from_cache(lua_root, &lua, scene_id, block_id, group_id)?;

        if data.triggers.is_empty() {
            let variables = data
                .variables
                .iter()
                .map(|v| (v.name.clone(), v.value))
                .collect();

            return Some(Self {
                data,
                context: LuaContext {
                    scene_id,
                    group_id,
                    config_id: 0,
                    source_entity_id: 0,
                    target_entity_id: 0,
                    uid: 1234,
                },
                triggers_by_event: HashMap::new(),
                active_suites: vec![1],
                active_trigger_names: Vec::new(),
                variables,
            });
        }

        let mut triggers_by_event: HashMap<EventType, Vec<CompiledTrigger>> = HashMap::new();

        for trig in &data.triggers {
            if trig.trigger_count == Some(0) {
                continue;
            }
            let cond = (!trig.condition.is_empty())
                .then_some(trig.condition.as_str())
                .and_then(|name| env.get::<Function>(name).ok());

            let action = (!trig.action.is_empty())
                .then_some(trig.action.as_str())
                .and_then(|name| env.get::<Function>(name).ok());

            if let Some(action) = action {
                triggers_by_event
                    .entry(trig.event.clone())
                    .or_default()
                    .push(CompiledTrigger {
                        name: trig.name.clone(),
                        condition: cond,
                        action: Some(action),
                    });
            }
        }

        if triggers_by_event.is_empty() {
            tracing::debug!("SceneGroupRuntime {} has no valid triggers", group_id);
            return None;
        }

        let variables = data
            .variables
            .iter()
            .map(|v| (v.name.clone(), v.value))
            .collect();

        let mut rt = Self {
            data,
            context: LuaContext {
                scene_id,
                group_id,
                config_id: 0,
                source_entity_id: 0,
                target_entity_id: 0,
                uid: 1234,
            },
            triggers_by_event,
            active_suites: vec![1],
            active_trigger_names: Vec::new(),
            variables,
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
