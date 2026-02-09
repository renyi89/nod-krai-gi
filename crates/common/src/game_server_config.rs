use crate::database_config::DatabaseSettings;
use crate::TomlConfig;
use serde::Deserialize;
use std::cmp::PartialEq;

#[derive(Deserialize)]
pub struct GameServerConfig {
    #[serde(skip)]
    pub language: u32,
    pub network: NetworkSettings,
    pub plugin: PluginSettings,
    pub database: DatabaseSettings,
    pub cur_region_name: String,
    pub region_list_path: String,
    pub encryption_config_path: String,
}

#[derive(Deserialize)]
pub struct NetworkSettings {
    pub udp_host: String,
}

#[derive(Deserialize)]
pub struct PluginSettings {
    pub packet_log: bool,
    pub ability: bool,
    pub ability_log: bool,
    pub social: bool,
    pub quest: bool,
}

impl TomlConfig for GameServerConfig {
    const DEFAULT_TOML: &str = include_str!("../game-server.default.toml");
}

use crate::language::Language;
use dashmap::DashMap;
use std::sync::{Arc, OnceLock};

pub static PLAYER_CACHE: OnceLock<Arc<DashMap<u32, PlayerCache>>> = OnceLock::new();

pub fn init_player_cache() {
    PLAYER_CACHE.get_or_init(|| Arc::new(DashMap::new()));
}

#[derive(Clone, Debug)]
pub struct PlayerCache {
    pub is_tp: bool,
    pub is_mp: bool,
    pub is_pause: bool,
    pub client_time: u32,
    pub nick_name: String,
    pub player_level: u32,
    pub language: Language,
    pub name_card_id: u32,
    pub profile_picture_id: u32,
    pub profile_frame_id: u32,
    pub cur_player_num_in_world: u32,
    pub online_status: PlayerStatusType,
}

impl Default for PlayerCache {
    fn default() -> Self {
        Self {
            is_tp: false,
            is_mp: false,
            is_pause: false,
            client_time: 0,
            nick_name: String::new(),
            player_level: 0,
            language: Language::Chs,
            name_card_id: 0,
            profile_picture_id: 0,
            profile_frame_id: 0,
            cur_player_num_in_world: 0,
            online_status: PlayerStatusType::PlayerStatusOffline,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerStatusType {
    PlayerStatusOffline = 0,
    PlayerStatusOnline = 1,
}

pub fn is_player_online(uid: u32) -> bool {
    if let Some(map) = PLAYER_CACHE.get() {
        if let Some(cache) = map.get(&uid) {
            return cache.online_status == PlayerStatusType::PlayerStatusOnline;
        }
    }
    false
}

pub fn update_player_cache<F>(uid: u32, f: F)
where
    F: FnOnce(&mut PlayerCache),
{
    if let Some(map) = PLAYER_CACHE.get() {
        use dashmap::mapref::entry::Entry;

        match map.entry(uid) {
            Entry::Occupied(mut entry) => {
                let v = entry.get_mut();
                f(v);
            }
            Entry::Vacant(entry) => {
                let mut c = PlayerCache::default();
                f(&mut c);
                entry.insert(c);
            }
        }
    }
}

pub fn cache_set_is_tp(uid: u32, value: bool) {
    update_player_cache(uid, |v| v.is_tp = value);
}

pub fn cache_get_is_tp(uid: u32) -> Option<bool> {
    get_player_cache(uid).map(|v| v.is_tp)
}

pub fn cache_set_is_mp(uid: u32, value: bool) {
    update_player_cache(uid, |v| v.is_mp = value);
}

pub fn cache_get_is_mp(uid: u32) -> Option<bool> {
    get_player_cache(uid).map(|v| v.is_mp)
}

pub fn cache_set_is_pause(uid: u32, value: bool) {
    update_player_cache(uid, |v| v.is_pause = value);
}

pub fn cache_get_is_pause(uid: u32) -> Option<bool> {
    get_player_cache(uid).map(|v| v.is_pause)
}

pub fn cache_set_client_time(uid: u32, value: u32) {
    update_player_cache(uid, |v| v.client_time = value);
}

pub fn cache_get_client_time(uid: u32) -> Option<u32> {
    get_player_cache(uid).map(|v| v.client_time)
}

pub fn cache_set_player_nick_name(uid: u32, nick: String) {
    update_player_cache(uid, |v| v.nick_name = nick);
}

pub fn cache_set_player_level(uid: u32, level: u32) {
    update_player_cache(uid, |v| v.player_level = level);
}

pub fn cache_set_language(uid: u32, language: Language) {
    update_player_cache(uid, |v| v.language = language);
}

pub fn cache_set_name_card_id(uid: u32, id: u32) {
    update_player_cache(uid, |v| v.name_card_id = id);
}

pub fn cache_set_profile_picture_id(uid: u32, id: u32) {
    update_player_cache(uid, |v| v.profile_picture_id = id);
}

pub fn cache_set_profile_frame_id(uid: u32, id: u32) {
    update_player_cache(uid, |v| v.profile_frame_id = id);
}

pub fn cache_set_cur_player_num_in_world(uid: u32, num: u32) {
    update_player_cache(uid, |v| v.cur_player_num_in_world = num);
}

pub fn cache_set_online_status(uid: u32, status: PlayerStatusType) {
    update_player_cache(uid, |v| v.online_status = status);
}

pub fn get_player_cache(uid: u32) -> Option<PlayerCache> {
    PLAYER_CACHE.get()?.get(&uid).map(|v| v.clone())
}

pub fn cache_get_player_nick_name(uid: u32) -> Option<String> {
    get_player_cache(uid).map(|v| v.nick_name)
}

pub fn cache_get_player_level(uid: u32) -> Option<u32> {
    get_player_cache(uid).map(|v| v.player_level)
}

pub fn cache_get_language(uid: u32) -> Option<Language> {
    get_player_cache(uid).map(|v| v.language)
}

pub fn cache_get_name_card_id(uid: u32) -> Option<u32> {
    get_player_cache(uid).map(|v| v.name_card_id)
}

pub fn cache_get_profile_picture_id(uid: u32) -> Option<u32> {
    get_player_cache(uid).map(|v| v.profile_picture_id)
}

pub fn cache_get_profile_frame_id(uid: u32) -> Option<u32> {
    get_player_cache(uid).map(|v| v.profile_frame_id)
}

pub fn cache_get_cur_player_num_in_world(uid: u32) -> Option<u32> {
    get_player_cache(uid).map(|v| v.cur_player_num_in_world)
}

pub fn cache_get_online_status(uid: u32) -> Option<PlayerStatusType> {
    get_player_cache(uid).map(|v| v.online_status)
}
