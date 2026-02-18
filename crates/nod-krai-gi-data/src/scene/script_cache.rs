use crate::scene::scene_block_template::*;
use crate::scene::scene_config_template::*;
use crate::scene::scene_group_template::*;
use dashmap::DashMap;
use mlua::{Lua, LuaSerdeExt, Value};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    fs,
    path::Path,
    sync::{Arc, OnceLock},
};

pub static SCENE_CONFIG_COLLECTION: OnceLock<Arc<HashMap<u32, SceneConfigTemplate>>> =
    OnceLock::new();
pub static SCENE_BLOCK_COLLECTION: OnceLock<Arc<HashMap<(u32, u32), SceneBlockTemplate>>> =
    OnceLock::new();

pub static SCENE_GROUP_COLLECTION: OnceLock<Arc<DashMap<u32, Option<SceneGroupTemplate>>>> =
    OnceLock::new();


pub fn init_scene_static_templates(root: &str) {
    let root_path = Path::new(root);

    let entries = match fs::read_dir(root_path) {
        Ok(v) => v,
        Err(_) => return,
    };

    let results: Vec<_> = entries
        .par_bridge()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if !entry.file_type().ok()?.is_dir() {
                return None;
            }

            let scene_id: u32 = entry.file_name().to_string_lossy().parse().ok()?;
            let scene_dir = entry.path();

            let config_path = scene_dir.join(format!("scene{}.lua", scene_id));
            let config = load_scene_config_file(&config_path);

            let mut config_map = HashMap::<u32, SceneConfigTemplate>::new();
            let mut block_map = HashMap::<(u32, u32), SceneBlockTemplate>::new();

            let mut block_id_list = vec![];

            if let Some(config) = config {
                block_id_list.extend(config.blocks.clone());
                config_map.insert(scene_id, config);
            }

            for block_id in block_id_list {
                let block_path = scene_dir.join(format!("scene{}_block{}.lua", scene_id, block_id));
                if let Some(block) = load_scene_block_file(&block_path) {
                    block_map.insert((scene_id, block_id), block);
                }
            }

            Some((config_map, block_map))
        })
        .collect();

    let mut config_map = HashMap::<u32, SceneConfigTemplate>::new();
    let mut block_map = HashMap::<(u32, u32), SceneBlockTemplate>::new();

    for (scene_configs, scene_blocks) in results {
        config_map.extend(scene_configs);
        block_map.extend(scene_blocks);
    }

    SCENE_CONFIG_COLLECTION.set(Arc::new(config_map)).ok();
    SCENE_BLOCK_COLLECTION.set(Arc::new(block_map)).ok();
    SCENE_GROUP_COLLECTION.set(Arc::new(DashMap::new())).ok();
}

fn load_scene_config_file(path: &Path) -> Option<SceneConfigTemplate> {
    let lua = Lua::new();

    let code = fs::read_to_string(path).ok()?;
    lua.load(&code).exec().ok()?;

    let globals = lua.globals();

    Some(SceneConfigTemplate {
        scene_config: lua
            .from_value(globals.get::<Value>("scene_config").ok()?)
            .ok()?,
        blocks: lua.from_value(globals.get::<Value>("blocks").ok()?).ok()?,
        block_rects: lua
            .from_value(globals.get::<Value>("block_rects").ok()?)
            .ok()?,
    })
}

fn load_scene_block_file(path: &Path) -> Option<SceneBlockTemplate> {
    let lua = Lua::new();

    let code = fs::read_to_string(path).ok()?;
    lua.load(&code).exec().ok()?;

    let globals = lua.globals();

    Some(SceneBlockTemplate {
        groups: lua.from_value(globals.get::<Value>("groups").ok()?).ok()?,
    })
}

pub fn scene_group_is_bad(group_id: u32) -> bool {
    let scene_group_collection_clone = Arc::clone(SCENE_GROUP_COLLECTION.get().unwrap());

    let Some(cache) = scene_group_collection_clone.get(&group_id) else {
        return false;
    };
    if cache.value().is_none() {
        true
    } else {
        false
    }
}

pub fn load_scene_group_from_cache(
    lua: &Lua,
    scene_id: u32,
    block_id: u32,
    group_id: u32,
) -> Option<SceneGroupTemplate> {
    let scene_group_collection_clone = Arc::clone(SCENE_GROUP_COLLECTION.get().unwrap());

    let Some(cache) = scene_group_collection_clone.get(&group_id) else {
        let result = load_scene_group(lua, scene_id, block_id, group_id);
        scene_group_collection_clone.insert(group_id, result.clone());
        return result;
    };
    cache.value().clone()
}

pub fn load_scene_group(
    lua: &Lua,
    scene_id: u32,
    block_id: u32,
    group_id: u32,
) -> Option<SceneGroupTemplate> {
    let globals = lua.globals();

    // monsters
    let monsters = match lua.from_value(globals.get::<Value>("monsters").ok()?) {
        Ok(v) => v,
        Err(err) => {
            println!(
                "load_scene_group {} parse monsters error: {}",
                group_id, err
            );
            return None;
        }
    };

    // gadgets
    let gadgets = match lua.from_value(globals.get::<Value>("gadgets").ok()?) {
        Ok(v) => v,
        Err(err) => {
            println!("load_scene_group {} parse gadgets error: {}", group_id, err);
            return None;
        }
    };

    // regions
    let regions = match lua.from_value(globals.get::<Value>("regions").ok()?) {
        Ok(v) => v,
        Err(err) => {
            println!("load_scene_group {} parse regions error: {}", group_id, err);
            return None;
        }
    };

    // triggers
    let triggers = match lua.from_value(globals.get::<Value>("triggers").ok()?) {
        Ok(v) => v,
        Err(err) => {
            println!(
                "load_scene_group {} parse triggers error: {}",
                group_id, err
            );
            return None;
        }
    };

    // variables
    let variables = match lua.from_value(globals.get::<Value>("variables").ok()?) {
        Ok(v) => v,
        Err(err) => {
            println!(
                "load_scene_group {} parse variables error: {}",
                group_id, err
            );
            return None;
        }
    };

    // init_config
    let init_config = match lua.from_value(globals.get::<Value>("init_config").ok()?) {
        Ok(v) => v,
        Err(err) => {
            println!(
                "load_scene_group {} parse init_config error: {}",
                group_id, err
            );
            return None;
        }
    };

    // suites
    let suites = match lua.from_value(globals.get::<Value>("suites").ok()?) {
        Ok(v) => v,
        Err(err) => {
            println!("load_scene_group {} parse suites error: {}", group_id, err);
            return None;
        }
    };

    Some(SceneGroupTemplate {
        monsters,
        gadgets,
        regions,
        triggers,
        variables,
        init_config,
        suites,
        base_info: BaseInfo {
            scene_id,
            block_id,
            group_id,
        },
    })
}