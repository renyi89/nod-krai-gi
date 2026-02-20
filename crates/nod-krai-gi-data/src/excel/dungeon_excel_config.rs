use std::collections::HashMap;

#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum DungeonSubType {
    #[default]
    #[serde(alias = "DUNGEON_SUB_NONE")]
    DungeonSubNone,
    #[serde(alias = "DUNGEON_SUB_BOSS")]
    DungeonSubBoss,
    #[serde(alias = "DUNGEON_SUB_TALENT")]
    DungeonSubTalent,
    #[serde(alias = "DUNGEON_SUB_WEAPON")]
    DungeonSubWeapon,
    #[serde(alias = "DUNGEON_SUB_RELIQUARY")]
    DungeonSubReliquary,
    #[serde(alias = "DUNGEON_SUB_LEY_LINE_CHALLENGE")]
    DungeonSubLeyLineChallenge,
}

#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum DungeonPlayType {
    #[default]
    #[serde(alias = "DUNGEON_PLAY_TYPE_NONE")]
    DungeonPlayTypeNone,
    #[serde(alias = "DUNGEON_PLAY_TYPE_FOGGY_MAZE")]
    DungeonPlayTypeFoggyMaze,
    #[serde(alias = "DUNGEON_PLAY_TYPE_MIST_TRIAL")]
    DungeonPlayTypeMistTrial,
    #[serde(alias = "DUNGEON_PLAY_TYPE_TRIAL_AVATAR")]
    DungeonPlayTypeTrialAvatar,
}

#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum DungeonInvolveType {
    #[default]
    #[serde(alias = "INVOLVE_NONE")]
    InvolveNone,
    #[serde(alias = "INVOLVE_ONLY_SINGLE")]
    InvolveOnlySingle,
    #[serde(alias = "INVOLVE_SINGLE_MULTIPLE")]
    InvolveSingleMultiple,
}

#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum SettleShowType {
    #[default]
    #[serde(alias = "SETTLE_SHOW_NONE")]
    SettleShowNone,
    #[serde(alias = "SETTLE_SHOW_TIME_COST")]
    SettleShowTimeCost,
    #[serde(alias = "SETTLE_SHOW_OPEN_CHEST_COUNT")]
    SettleShowOpenChestCount,
    #[serde(alias = "SETTLE_SHOW_KILL_MONSTER_COUNT")]
    SettleShowKillMonsterCount,
    #[serde(alias = "SETTLE_SHOW_BLACKSCREEN")]
    SettleShowBlackscreen,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DungeonExcelConfig {
    pub id: u32,
    pub scene_id: u32,
    pub show_level: u32,
    pub sub_type: DungeonSubType,
    pub play_type: DungeonPlayType,
    pub involve_type: DungeonInvolveType,
    pub limit_level: u32,
    pub pass_cond: u32,
    pub pass_jump_dungeon: u32,
    pub revive_max_count: u32,
    pub settle_countdown_time: u32,
    pub fail_settle_countdown_time: u32,
    pub quit_settle_countdown_time: u32,
    pub settle_shows: Vec<SettleShowType>,
    #[serde(alias = "passRewardPreviewID")]
    pub pass_reward_preview_id: u32,
    #[serde(alias = "statueCostID")]
    pub statue_cost_id: u32,
    pub statue_cost_count: u32,
    pub statue_drop: u32,
}

pub trait DungeonExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, DungeonExcelConfig>;
}

impl DungeonExcelConfigKeyed<u32> for DungeonExcelConfig {
    fn key(&self) -> u32 {
        self.id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, DungeonExcelConfig> {
        let json =  std::fs::read(&format!(
            "{excel_bin_output_path}/DungeonExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<DungeonExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();
        data
    }
}
