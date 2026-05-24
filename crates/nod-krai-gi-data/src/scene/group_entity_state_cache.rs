use dashmap::DashMap;
use std::sync::{Arc, OnceLock};

pub static GROUP_ENTITY_STATE_CACHE: OnceLock<Arc<GroupEntityStateCache>> = OnceLock::new();

pub fn get_group_entity_state_cache() -> Arc<GroupEntityStateCache> {
    Arc::clone(GROUP_ENTITY_STATE_CACHE.get_or_init(|| Arc::new(GroupEntityStateCache::new())))
}

pub struct GroupEntityStateCache {
    user_caches: DashMap<u32, UserGroupStateCache>,
}

#[derive(Clone, Debug, Default)]
pub struct UserGroupStateCache {
    pub group_states: DashMap<u32, GroupState>,
}

#[derive(Clone, Debug, Default)]
pub struct GroupState {
    pub monsters: DashMap<u32, MonsterEntityState>,
    pub gadgets: DashMap<u32, GadgetEntityState>,
}

#[derive(Clone, Debug)]
pub struct MonsterEntityState {
    pub config_id: u32,
    pub entity_id: u32,
    pub life_state: u32,
    pub cur_hp: f32,
    pub max_hp: f32,
}

#[derive(Clone, Debug)]
pub struct GadgetEntityState {
    pub config_id: u32,
    pub entity_id: u32,
    pub life_state: u32,
    pub cur_hp: f32,
    pub max_hp: f32,
    pub gadget_state: u32,
}

impl GroupEntityStateCache {
    pub fn new() -> Self {
        Self {
            user_caches: DashMap::new(),
        }
    }

    fn get_or_create_user_cache(
        &self,
        uid: u32,
    ) -> dashmap::mapref::one::Ref<'_, u32, UserGroupStateCache> {
        self.user_caches.entry(uid).or_default();
        self.user_caches.get(&uid).unwrap()
    }

    pub fn clear_user_cache(&self, uid: u32) {
        self.user_caches.remove(&uid);
    }

    pub fn on_monster_spawn(
        &self,
        uid: u32,
        group_id: u32,
        config_id: u32,
        entity_id: u32,
        cur_hp: f32,
        max_hp: f32,
    ) {
        let user_cache = self.get_or_create_user_cache(uid);
        let group_state = user_cache.group_states.entry(group_id).or_default();
        group_state.monsters.insert(
            config_id,
            MonsterEntityState {
                config_id,
                entity_id,
                life_state: 1,
                cur_hp,
                max_hp,
            },
        );
    }

    pub fn on_gadget_spawn(
        &self,
        uid: u32,
        group_id: u32,
        config_id: u32,
        entity_id: u32,
        cur_hp: f32,
        max_hp: f32,
        gadget_state: u32,
    ) {
        let user_cache = self.get_or_create_user_cache(uid);
        let group_state = user_cache.group_states.entry(group_id).or_default();
        group_state.gadgets.insert(
            config_id,
            GadgetEntityState {
                config_id,
                entity_id,
                life_state: 1,
                cur_hp,
                max_hp,
                gadget_state,
            },
        );
    }

    pub fn on_monster_life_state_update(
        &self,
        uid: u32,
        group_id: u32,
        config_id: u32,
        life_state: u32,
        cur_hp: f32,
        max_hp: f32,
    ) {
        if let Some(user_cache) = self.user_caches.get(&uid) {
            if let Some(group_state) = user_cache.group_states.get(&group_id) {
                if let Some(mut monster_state) = group_state.monsters.get_mut(&config_id) {
                    monster_state.life_state = life_state;
                    monster_state.cur_hp = cur_hp;
                    monster_state.max_hp = max_hp;
                }
            }
        }
    }

    pub fn on_gadget_life_state_update(
        &self,
        uid: u32,
        group_id: u32,
        config_id: u32,
        life_state: u32,
        cur_hp: f32,
        max_hp: f32,
    ) {
        if let Some(user_cache) = self.user_caches.get(&uid) {
            if let Some(group_state) = user_cache.group_states.get(&group_id) {
                if let Some(mut gadget_state) = group_state.gadgets.get_mut(&config_id) {
                    gadget_state.life_state = life_state;
                    gadget_state.cur_hp = cur_hp;
                    gadget_state.max_hp = max_hp;
                }
            }
        }
    }

    pub fn on_gadget_state_update(
        &self,
        uid: u32,
        group_id: u32,
        config_id: u32,
        gadget_state: u32,
    ) {
        if let Some(user_cache) = self.user_caches.get(&uid) {
            if let Some(group_state) = user_cache.group_states.get(&group_id) {
                if let Some(mut state) = group_state.gadgets.get_mut(&config_id) {
                    state.gadget_state = gadget_state;
                }
            }
        }
    }

    pub fn get_monster_state(
        &self,
        uid: u32,
        group_id: u32,
        config_id: u32,
    ) -> Option<MonsterEntityState> {
        let user_cache = self.user_caches.get(&uid)?;
        let group_state = user_cache.group_states.get(&group_id)?;
        group_state.monsters.get(&config_id).map(|m| m.clone())
    }

    pub fn get_gadget_state(
        &self,
        uid: u32,
        group_id: u32,
        config_id: u32,
    ) -> Option<GadgetEntityState> {
        let user_cache = self.user_caches.get(&uid)?;
        let group_state = user_cache.group_states.get(&group_id)?;
        group_state.gadgets.get(&config_id).map(|g| g.clone())
    }

    pub fn get_alive_monster_count(&self, uid: u32, group_id: u32) -> u32 {
        if let Some(user_cache) = self.user_caches.get(&uid) {
            if let Some(group_state) = user_cache.group_states.get(&group_id) {
                return group_state
                    .monsters
                    .iter()
                    .filter(|m| m.life_state == 1)
                    .count() as u32;
            }
        }
        0
    }

    pub fn is_all_monsters_dead(&self, uid: u32, group_id: u32) -> bool {
        self.get_alive_monster_count(uid, group_id) == 0
    }

    pub fn remove_group(&self, uid: u32, group_id: u32) {
        if let Some(user_cache) = self.user_caches.get(&uid) {
            user_cache.group_states.remove(&group_id);
        }
    }

    pub fn reset_group(&self, uid: u32, group_id: u32) {
        if let Some(user_cache) = self.user_caches.get(&uid) {
            user_cache.group_states.remove(&group_id);
        }
    }
}

impl Default for GroupEntityStateCache {
    fn default() -> Self {
        Self::new()
    }
}
