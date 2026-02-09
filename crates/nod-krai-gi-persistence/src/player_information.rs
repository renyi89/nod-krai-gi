// player persistent 'Data' definitions

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlayerDataBin {
    pub uid: u32,
    pub nick_name: String,
    pub guid_counter: u32,
    pub basic_bin: PlayerBasicCompBin,
    pub avatar_bin: PlayerAvatarCompBin,
    pub quest_bin: PlayerQuestCompBin,
    pub item_bin: PlayerItemCompBin,
    pub scene_bin: PlayerSceneCompBin,
}

impl PlayerDataBin {
    pub fn next_guid(&mut self) -> u64 {
        self.guid_counter += 1;
        ((self.uid as u64) << 32) | self.guid_counter as u64
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct PlayerBasicCompBin {
    pub level: u32,
    pub exp: u32,
    pub is_game_time_locked: bool,
}

#[derive(Serialize, Deserialize, Default)]
pub struct PlayerAvatarCompBin {
    pub avatar_map: HashMap<u64, AvatarBin>,
    pub cur_avatar_guid: u64,
    pub team_map: HashMap<u32, AvatarTeamBin>,
    pub cur_team_id: u32,
    pub choose_avatar_guid: u64,
    pub owned_flycloak_list: Vec<u32>,
    pub owned_costume_list: Vec<u32>,
    pub owned_trace_effect_list: Vec<u32>,
    pub cur_avatar_guid_list: Vec<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct AvatarTeamBin {
    pub avatar_guid_list: Vec<u64>,
    pub team_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct AvatarBin {
    pub avatar_id: u32,
    pub guid: u64,
    pub level: u32,
    pub cur_hp: f32,
    pub promote_level: u32,
    pub skill_depot_id: u32,
    pub skill_map: HashMap<u32, AvatarSkillBin>,
    pub depot_map: HashMap<u32, AvatarSkillDepotBin>,
    pub born_time: u32,
    pub weapon_guid: u64,
    pub wearing_flycloak_id: u32,
    pub costume_id: u32,
    pub trace_effect_id: u32,
    pub weapon_skin_id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct AvatarSkillBin {
    pub max_charge_count: u32,
}

#[derive(Serialize, Deserialize)]
pub struct AvatarSkillDepotBin {
    pub talent_id_list: Vec<u32>,
    pub core_proud_skill_level: u32,
    pub inherent_proud_skill_list: Vec<u32>,
    pub skill_level_map: HashMap<u32, u32>,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerItemCompBin {
    pub pack_store: ItemStoreBin,
}

#[derive(Serialize, Deserialize)]
pub struct ItemStoreBin {
    pub item_map: HashMap<u64, ItemBin>,
}
#[derive(Serialize, Deserialize)]
pub enum ItemBin {
    Weapon {
        weapon_id: u32,
        level: u32,
        exp: u32,
        promote_level: u32,
        affix_map: HashMap<u32, u32>,
        is_locked: bool,
    },
}

#[derive(Serialize, Deserialize)]
pub struct PlayerSceneCompBin {
    pub my_cur_scene_id: u32,
    pub my_prev_pos: nod_krai_gi_proto::server_only::Vector,
    pub my_prev_rot: nod_krai_gi_proto::server_only::Vector,
}

#[derive(Serialize, Deserialize, Default)]
pub struct PlayerQuestCompBin {
    pub enable: bool,
    pub parent_quest_map: HashMap<u32, ParentQuestItem>,
    pub quest_map: HashMap<u32, QuestItem>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ParentQuestItem {
    pub accept_time: u32,
    pub quest_vars: Vec<u32>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct QuestItem {
    pub parent_quest_id: u32,
    pub state: u32,
    pub start_time: u32,
    pub accept_time: u32,
    pub finish_time: u32,
    pub finish_progress_list: Vec<u32>,
    pub fail_progress_list: Vec<u32>,
}

impl PlayerItemCompBin {
    pub fn add_item(&mut self, guid: u64, item: ItemBin) {
        let _ = &self.pack_store.item_map.insert(guid, item);
    }

    pub fn get_item(&self, guid: &u64) -> Option<&ItemBin> {
        self.pack_store.item_map.get(guid)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, u64, ItemBin> {
        self.pack_store.item_map.iter()
    }
}
