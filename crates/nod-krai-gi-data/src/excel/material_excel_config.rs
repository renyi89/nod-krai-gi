use super::common::{ItemType, MaterialType};
use common::string_util::InternString;
use std::collections::HashMap;

#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum ItemUseTarget {
    #[serde(alias = "ITEM_USE_TARGET_NONE")]
    #[default]
    None,
    #[serde(alias = "ITEM_USE_TARGET_CUR_AVATAR")]
    CurAvatar,
    #[serde(alias = "ITEM_USE_TARGET_CUR_TEAM")]
    CurTeam,
    #[serde(alias = "ITEM_USE_TARGET_SPECIFY_AVATAR")]
    SpecifyAvatar,
    #[serde(alias = "ITEM_USE_TARGET_SPECIFY_ALIVE_AVATAR")]
    SpecifyAliveAvatar,
    #[serde(alias = "ITEM_USE_TARGET_SPECIFY_DEAD_AVATAR")]
    SpecifyDeadAvatar,
    #[serde(alias = "ITEM_USE_TARGET_PLAYER_AVATAR")]
    PlayerAvatar,
}

#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum ItemUseOp {
    #[serde(alias = "ITEM_USE_ACCEPT_QUEST")]
    AcceptQuest,

    #[serde(alias = "ITEM_USE_TRIGGER_ABILITY")]
    TriggerAbility,

    #[serde(alias = "ITEM_USE_GAIN_AVATAR")]
    GainAvatar,

    #[serde(alias = "ITEM_USE_ADD_EXP")]
    AddExp,

    #[serde(alias = "ITEM_USE_RELIVE_AVATAR")]
    ReliveAvatar,

    #[serde(alias = "ITEM_USE_ADD_BIG_TALENT_POINT")]
    AddBigTalentPoint,

    #[serde(alias = "ITEM_USE_ADD_PERSIST_STAMINA")]
    AddPersistStamina,

    #[serde(alias = "ITEM_USE_ADD_TEMPORARY_STAMINA")]
    AddTemporaryStamina,

    #[serde(alias = "ITEM_USE_ADD_CUR_STAMINA")]
    AddCurStamina,

    #[serde(alias = "ITEM_USE_ADD_CUR_HP")]
    AddCurHp,

    #[serde(alias = "ITEM_USE_ADD_ELEM_ENERGY")]
    AddElemEnergy,

    #[serde(alias = "ITEM_USE_ADD_ALL_ENERGY")]
    AddAllEnergy,

    #[serde(alias = "ITEM_USE_ADD_DUNGEON_COND_TIME")]
    AddDungeonCondTime,

    #[serde(alias = "ITEM_USE_ADD_WEAPON_EXP")]
    AddWeaponExp,

    #[serde(alias = "ITEM_USE_ADD_SERVER_BUFF")]
    AddServerBuff,

    #[serde(alias = "ITEM_USE_DEL_SERVER_BUFF")]
    DelServerBuff,

    #[serde(alias = "ITEM_USE_UNLOCK_COOK_RECIPE")]
    UnlockCookRecipe,

    #[serde(alias = "ITEM_USE_OPEN_RANDOM_CHEST")]
    OpenRandomChest,

    #[serde(alias = "ITEM_USE_MAKE_GADGET")]
    MakeGadget,

    #[serde(alias = "ITEM_USE_ADD_ITEM")]
    AddItem,

    #[serde(alias = "ITEM_USE_GRANT_SELECT_REWARD")]
    GrantSelectReward,

    #[serde(alias = "ITEM_USE_ADD_SELECT_ITEM")]
    AddSelectItem,

    #[serde(alias = "ITEM_USE_GAIN_FLYCLOAK")]
    GainFlycloak,

    #[serde(alias = "ITEM_USE_GAIN_NAME_CARD")]
    GainNameCard,

    #[serde(alias = "ITEM_USE_UNLOCK_PAID_BATTLE_PASS_NORMAL")]
    UnlockPaidBattlePassNormal,

    #[serde(alias = "ITEM_USE_GAIN_CARD_PRODUCT")]
    GainCardProduct,

    #[serde(alias = "ITEM_USE_UNLOCK_FORGE")]
    UnlockForge,

    #[serde(alias = "ITEM_USE_UNLOCK_COMBINE")]
    UnlockCombine,

    #[serde(alias = "ITEM_USE_UNLOCK_CODEX")]
    UnlockCodex,

    #[serde(alias = "ITEM_USE_CHEST_SELECT_ITEM")]
    ChestSelectItem,

    #[serde(alias = "ITEM_USE_GAIN_RESIN_CARD_PRODUCT")]
    GainResinCardProduct,

    #[serde(alias = "ITEM_USE_ADD_RELIQUARY_EXP")]
    AddReliquaryExp,

    #[serde(alias = "ITEM_USE_UNLOCK_FURNITURE_FORMULA")]
    UnlockFurnitureFormula,

    #[serde(alias = "ITEM_USE_UNLOCK_FURNITURE_SUITE")]
    UnlockFurnitureSuite,

    #[serde(alias = "ITEM_USE_ADD_CHANNELLER_SLAB_BUFF")]
    AddChannellerSlabBuff,

    #[serde(alias = "ITEM_USE_GAIN_COSTUME")]
    GainCostume,

    #[serde(alias = "ITEM_USE_ADD_TREASURE_MAP_BONUS_REGION_FRAGMENT")]
    AddTreasureMapBonusRegionFragment,

    #[serde(alias = "ITEM_USE_COMBINE_ITEM")]
    CombineItem,

    #[serde(alias = "ITEM_USE_UNLOCK_HOME_MODULE")]
    UnlockHomeModule,

    #[serde(alias = "ITEM_USE_UNLOCK_HOME_BGM")]
    UnlockHomeBgm,

    #[serde(alias = "ITEM_USE_ADD_REGIONAL_PLAY_VAR")]
    AddRegionalPlayVar,

    #[serde(alias = "ITEM_USE_UNLOCK_AVATAR_TRACE")]
    UnlockAvatarTrace,

    #[serde(alias = "ITEM_USE_UNLOCK_NORMAL_BEYOND_BATTLE_PASS")]
    UnlockNormalBeyondBattlePass,

    #[serde(alias = "ITEM_USE_OPEN_RENAME_DIALOG")]
    OpenRenameDialog,

    #[serde(alias = "ITEM_USE_ADD_PHLOGISTON")]
    AddPhlogiston,

    #[serde(alias = "ITEM_USE_ADD_MAGNET_POWER")]
    AddMagnetPower,

    #[serde(alias = "ITEM_USE_GAIN_AVATAR_TALENT_MATERIAL")]
    GainAvatarTalentMaterial,

    #[serde(alias = "ITEM_USE_ADD_AVATAR_EXTRA_PROPERTY")]
    AddAvatarExtraProperty,

    #[serde(alias = "ITEM_USE_ADD_ALCHEMY_SIM_ITEM")]
    AddAlchemySimItem,

    #[serde(alias = "ITEM_USE_SET_OPEN_STATE")]
    SetOpenState,

    #[serde(alias = "ITEM_USE_CHECK_FORMAL_AVATAR")]
    CheckFormalAvatar,

    #[serde(alias = "ITEM_USE_OPEN_DROP_EXTRA")]
    OpenDropExtra,

    #[serde(alias = "ITEM_USE_UNLOCK_PROFILE_FRAME")]
    UnlockProfileFrame,

    #[serde(alias = "ITEM_USE_UNLOCK_PROFILE_PICTURE")]
    UnlockProfilePicture,

    #[serde(alias = "ITEM_USE_UNLOCK_PHOTOGRAPH_POSE")]
    UnlockPhotographPose,

    #[serde(alias = "ITEM_USE_MUSIC_GAME_BOOK_UNLOCK_THEME")]
    MusicGameBookUnlockTheme,

    #[serde(alias = "ITEM_USE_NONE")]
    #[serde(other)]
    #[default]
    None,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemUse {
    pub use_op: ItemUseOp,
    pub use_param: Vec<InternString>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialExcelConfig {
    pub id: u32,
    pub item_type: ItemType,
    pub material_type: MaterialType,

    pub item_use: Vec<ItemUse>,
    pub max_use_count: u32,
    pub use_on_gain: bool,
    pub use_target: ItemUseTarget,

    pub rank: u32,
    pub rank_level: u32,
    pub gadget_id: u32,
    pub desc_text_map_hash: u64,
    pub name_text_map_hash: u64,
}

pub trait MaterialExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, MaterialExcelConfig>;
}

impl MaterialExcelConfigKeyed<u32> for MaterialExcelConfig {
    fn key(&self) -> u32 {
        self.id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, MaterialExcelConfig> {
        let json = std::fs::read(&format!(
            "{excel_bin_output_path}/MaterialExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<MaterialExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();
        data
    }
}
