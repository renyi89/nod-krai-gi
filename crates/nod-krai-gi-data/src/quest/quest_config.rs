use common::string_util::InternString;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::sync::Arc;

pub static QUEST_CONFIG_COLLECTION: std::sync::OnceLock<Arc<HashMap<u32, QuestConfig>>> =
    std::sync::OnceLock::new();

pub static SUB_QUEST_CONFIG_COLLECTION: std::sync::OnceLock<Arc<HashMap<u32, SubQuestData>>> =
    std::sync::OnceLock::new();

/// 安全获取任务配置集合
/// 如果配置未初始化，返回空 Arc
pub fn get_quest_config_collection() -> Arc<HashMap<u32, QuestConfig>> {
    QUEST_CONFIG_COLLECTION
        .get()
        .cloned()
        .unwrap_or_else(|| Arc::new(HashMap::new()))
}

/// 安全获取子任务配置集合
/// 如果配置未初始化，返回空 Arc
pub fn get_sub_quest_config_collection() -> Arc<HashMap<u32, SubQuestData>> {
    SUB_QUEST_CONFIG_COLLECTION
        .get()
        .cloned()
        .unwrap_or_else(|| Arc::new(HashMap::new()))
}

#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum QuestType {
    #[default]
    #[serde(alias = "AQ")]
    AQ,
    #[serde(alias = "FQ")]
    FQ,
    #[serde(alias = "LQ")]
    LQ,
    #[serde(alias = "EQ")]
    EQ,
    #[serde(alias = "DQ")]
    DQ,
    #[serde(alias = "IQ")]
    IQ,
    #[serde(alias = "VQ")]
    VQ,
    #[serde(alias = "WQ")]
    WQ,
}

#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum LogicType {
    #[serde(alias = "LOGIC_NONE")]
    #[default]
    LogicNone,
    #[serde(alias = "LOGIC_AND")]
    LogicAnd,
    #[serde(alias = "LOGIC_OR")]
    LogicOr,
    #[serde(alias = "LOGIC_NOT")]
    LogicNot,
    #[serde(alias = "LOGIC_A_AND_ETCOR")]
    LogicAAndEtcor,
    #[serde(alias = "LOGIC_A_AND_B_AND_ETCOR")]
    LogicAAndBAndEtcor,
    #[serde(alias = "LOGIC_A_OR_ETCAND")]
    LogicAOrEtcand,
    #[serde(alias = "LOGIC_A_OR_B_OR_ETCAND")]
    LogicAOrBOrEtcand,
    #[serde(alias = "LOGIC_A_AND_B_OR_ETCAND")]
    LogicAAndBOrEtcand,
    #[serde(alias = "QUEST_HIDDEN")]
    QuestHidden,
}

#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum QuestExec {
    #[serde(alias = "QUEST_EXEC_DEL_PACK_ITEM")]
    DelPackItem,

    #[serde(alias = "QUEST_EXEC_UNLOCK_POINT")]
    UnlockPoint,

    #[serde(alias = "QUEST_EXEC_UNLOCK_AREA")]
    UnlockArea,

    #[serde(alias = "QUEST_EXEC_UNLOCK_FORCE")]
    UnlockForce,

    #[serde(alias = "QUEST_EXEC_LOCK_FORCE")]
    LockForce,

    #[serde(alias = "QUEST_EXEC_CHANGE_AVATAR_ELEMET")]
    ChangeAvatarElement,

    #[serde(alias = "QUEST_EXEC_REFRESH_GROUP_MONSTER")]
    RefreshGroupMonster,

    #[serde(alias = "QUEST_EXEC_SET_IS_FLYABLE")]
    SetIsFlyable,

    #[serde(alias = "QUEST_EXEC_SET_IS_WEATHER_LOCKED")]
    SetIsWeatherLocked,

    #[serde(alias = "QUEST_EXEC_SET_IS_GAME_TIME_LOCKED")]
    SetIsGameTimeLocked,

    #[serde(alias = "QUEST_EXEC_SET_IS_TRANSFERABLE")]
    SetIsTransferable,

    #[serde(alias = "QUEST_EXEC_GRANT_TRIAL_AVATAR")]
    GrantTrialAvatar,

    #[serde(alias = "QUEST_EXEC_OPEN_BORED")]
    OpenBored,

    #[serde(alias = "QUEST_EXEC_ROLLBACK_QUEST")]
    RollbackQuest,

    #[serde(alias = "QUEST_EXEC_NOTIFY_GROUP_LUA")]
    NotifyGroupLua,

    #[serde(alias = "QUEST_EXEC_SET_OPEN_STATE")]
    SetOpenState,

    #[serde(alias = "QUEST_EXEC_LOCK_POINT")]
    LockPoint,

    #[serde(alias = "QUEST_EXEC_DEL_PACK_ITEM_BATCH")]
    DelPackItemBatch,

    #[serde(alias = "QUEST_EXEC_REFRESH_GROUP_SUITE")]
    RefreshGroupSuite,

    #[serde(alias = "QUEST_EXEC_REMOVE_TRIAL_AVATAR")]
    RemoveTrialAvatar,

    #[serde(alias = "QUEST_EXEC_SET_GAME_TIME")]
    SetGameTime,

    #[serde(alias = "QUEST_EXEC_SET_WEATHER_GADGET")]
    SetWeatherGadget,

    #[serde(alias = "QUEST_EXEC_ADD_QUEST_PROGRESS")]
    AddQuestProgress,

    #[serde(alias = "QUEST_EXEC_NOTIFY_DAILY_TASK")]
    NotifyDailyTask,

    #[serde(alias = "QUEST_EXEC_CREATE_PATTERN_GROUP")]
    CreatePatternGroup,

    #[serde(alias = "QUEST_EXEC_REMOVE_PATTERN_GROUP")]
    RemovePatternGroup,

    #[serde(alias = "QUEST_EXEC_REFRESH_GROUP_SUITE_RANDOM")]
    RefreshGroupSuiteRandom,

    #[serde(alias = "QUEST_EXEC_ACTIVE_ITEM_GIVING")]
    ActiveItemGiving,

    #[serde(alias = "QUEST_EXEC_DEL_ALL_SPECIFIC_PACK_ITEM")]
    DelAllSpecificPackItem,

    #[serde(alias = "QUEST_EXEC_ROLLBACK_PARENT_QUEST")]
    RollbackParentQuest,

    #[serde(alias = "QUEST_EXEC_LOCK_AVATAR_TEAM")]
    LockAvatarTeam,

    #[serde(alias = "QUEST_EXEC_UNLOCK_AVATAR_TEAM")]
    UnlockAvatarTeam,

    #[serde(alias = "QUEST_EXEC_UPDATE_PARENT_QUEST_REWARD_INDEX")]
    UpdateParentQuestRewardIndex,

    #[serde(alias = "QUEST_EXEC_SET_DAILY_TASK_VAR")]
    SetDailyTaskVar,

    #[serde(alias = "QUEST_EXEC_INC_DAILY_TASK_VAR")]
    IncDailyTaskVar,

    #[serde(alias = "QUEST_EXEC_DEC_DAILY_TASK_VAR")]
    DecDailyTaskVar,

    #[serde(alias = "QUEST_EXEC_ACTIVE_ACTIVITY_COND_STATE")]
    ActiveActivityCondState,

    #[serde(alias = "QUEST_EXEC_INACTIVE_ACTIVITY_COND_STATE")]
    InactiveActivityCondState,

    #[serde(alias = "QUEST_EXEC_ADD_CUR_AVATAR_ENERGY")]
    AddCurAvatarEnergy,

    #[serde(alias = "QUEST_EXEC_START_BARGAIN")]
    StartBargain,

    #[serde(alias = "QUEST_EXEC_STOP_BARGAIN")]
    StopBargain,

    #[serde(alias = "QUEST_EXEC_SET_QUEST_GLOBAL_VAR")]
    SetQuestGlobalVar,

    #[serde(alias = "QUEST_EXEC_INC_QUEST_GLOBAL_VAR")]
    IncQuestGlobalVar,

    #[serde(alias = "QUEST_EXEC_DEC_QUEST_GLOBAL_VAR")]
    DecQuestGlobalVar,

    #[serde(alias = "QUEST_EXEC_REGISTER_DYNAMIC_GROUP")]
    RegisterDynamicGroup,

    #[serde(alias = "QUEST_EXEC_UNREGISTER_DYNAMIC_GROUP")]
    UnregisterDynamicGroup,

    #[serde(alias = "QUEST_EXEC_SET_QUEST_VAR")]
    SetQuestVar,

    #[serde(alias = "QUEST_EXEC_INC_QUEST_VAR")]
    IncQuestVar,

    #[serde(alias = "QUEST_EXEC_DEC_QUEST_VAR")]
    DecQuestVar,

    #[serde(alias = "QUEST_EXEC_RANDOM_QUEST_VAR")]
    RandomQuestVar,

    #[serde(alias = "QUEST_EXEC_ACTIVATE_SCANNING_PIC")]
    ActivateScanningPic,

    #[serde(alias = "QUEST_EXEC_RELOAD_SCENE_TAG")]
    ReloadSceneTag,

    #[serde(alias = "QUEST_EXEC_REGISTER_DYNAMIC_GROUP_ONLY")]
    RegisterDynamicGroupOnly,

    #[serde(alias = "QUEST_EXEC_CHANGE_SKILL_DEPOT")]
    ChangeSkillDepot,

    #[serde(alias = "QUEST_EXEC_ADD_SCENE_TAG")]
    AddSceneTag,

    #[serde(alias = "QUEST_EXEC_DEL_SCENE_TAG")]
    DelSceneTag,

    #[serde(alias = "QUEST_EXEC_INIT_TIME_VAR")]
    InitTimeVar,

    #[serde(alias = "QUEST_EXEC_CLEAR_TIME_VAR")]
    ClearTimeVar,

    #[serde(alias = "QUEST_EXEC_MODIFY_CLIMATE_AREA")]
    ModifyClimateArea,

    #[serde(alias = "QUEST_EXEC_GRANT_TRIAL_AVATAR_AND_LOCK_TEAM")]
    GrantTrialAvatarAndLockTeam,

    #[serde(alias = "QUEST_EXEC_CHANGE_MAP_AREA_STATE")]
    ChangeMapAreaState,

    #[serde(alias = "QUEST_EXEC_DEACTIVE_ITEM_GIVING")]
    DeactiveItemGiving,

    #[serde(alias = "QUEST_EXEC_CHANGE_SCENE_LEVEL_TAG")]
    ChangeSceneLevelTag,

    #[serde(alias = "QUEST_EXEC_UNLOCK_PLAYER_WORLD_SCENE")]
    UnlockPlayerWorldScene,

    #[serde(alias = "QUEST_EXEC_LOCK_PLAYER_WORLD_SCENE")]
    LockPlayerWorldScene,

    #[serde(alias = "QUEST_EXEC_FAIL_MAINCOOP")]
    FailMainCoop,

    #[serde(alias = "QUEST_EXEC_MODIFY_WEATHER_AREA")]
    ModifyWeatherArea,

    #[serde(alias = "QUEST_EXEC_MODIFY_ARANARA_COLLECTION_STATE")]
    ModifyAranaraCollectionState,

    #[serde(alias = "QUEST_EXEC_GRANT_TRIAL_AVATAR_BATCH_AND_LOCK_TEAM")]
    GrantTrialAvatarBatchAndLockTeam,

    #[serde(alias = "QUEST_EXEC_NONE")]
    #[serde(other)]
    #[default]
    None,
}
#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum QuestCond {
    #[serde(alias = "QUEST_COND_STATE_EQUAL")]
    StateEqual = 1,
    #[serde(alias = "QUEST_COND_STATE_NOT_EQUAL")]
    StateNotEqual = 2,
    #[serde(alias = "QUEST_COND_PACK_HAVE_ITEM")]
    PackHaveItem = 3,
    #[serde(alias = "QUEST_COND_AVATAR_ELEMENT_EQUAL")]
    AvatarElementEqual = 4,
    #[serde(alias = "QUEST_COND_AVATAR_ELEMENT_NOT_EQUAL")]
    AvatarElementNotEqual = 5,
    #[serde(alias = "QUEST_COND_AVATAR_CAN_CHANGE_ELEMENT")]
    AvatarCanChangeElement = 6,
    #[serde(alias = "QUEST_COND_CITY_LEVEL_EQUAL_GREATER")]
    CityLevelEqualGreater = 7,
    #[serde(alias = "QUEST_COND_ITEM_NUM_LESS_THAN")]
    ItemNumLessThan = 8,
    #[serde(alias = "QUEST_COND_DAILY_TASK_START")]
    DailyTaskStart = 9,
    #[serde(alias = "QUEST_COND_OPEN_STATE_EQUAL")]
    OpenStateEqual = 10,
    #[serde(alias = "QUEST_COND_DAILY_TASK_OPEN")]
    DailyTaskOpen = 11,
    #[serde(alias = "QUEST_COND_DAILY_TASK_REWARD_CAN_GET")]
    DailyTaskRewardCanGet = 12,
    #[serde(alias = "QUEST_COND_DAILY_TASK_REWARD_RECEIVED")]
    DailyTaskRewardReceived = 13,
    #[serde(alias = "QUEST_COND_PLAYER_LEVEL_REWARD_CAN_GET")]
    PlayerLevelRewardCanGet = 14,
    #[serde(alias = "QUEST_COND_EXPLORATION_REWARD_CAN_GET")]
    ExplorationRewardCanGet = 15,
    #[serde(alias = "QUEST_COND_IS_WORLD_OWNER")]
    IsWorldOwner = 16,
    #[serde(alias = "QUEST_COND_PLAYER_LEVEL_EQUAL_GREATER")]
    PlayerLevelEqualGreater = 17,
    #[serde(alias = "QUEST_COND_SCENE_AREA_UNLOCKED")]
    SceneAreaUnlocked = 18,
    #[serde(alias = "QUEST_COND_ITEM_GIVING_ACTIVED")]
    ItemGivingActived = 19,
    #[serde(alias = "QUEST_COND_ITEM_GIVING_FINISHED")]
    ItemGivingFinished = 20,
    #[serde(alias = "QUEST_COND_IS_DAYTIME")]
    IsDaytime = 21,
    #[serde(alias = "QUEST_COND_CURRENT_AVATAR")]
    CurrentAvatar = 22,
    #[serde(alias = "QUEST_COND_CURRENT_AREA")]
    CurrentArea = 23,
    #[serde(alias = "QUEST_COND_QUEST_VAR_EQUAL")]
    QuestVarEqual = 24,
    #[serde(alias = "QUEST_COND_QUEST_VAR_GREATER")]
    QuestVarGreater = 25,
    #[serde(alias = "QUEST_COND_QUEST_VAR_LESS")]
    QuestVarLess = 26,
    #[serde(alias = "QUEST_COND_FORGE_HAVE_FINISH")]
    ForgeHaveFinish = 27,
    #[serde(alias = "QUEST_COND_DAILY_TASK_IN_PROGRESS")]
    DailyTaskInProgress = 28,
    #[serde(alias = "QUEST_COND_DAILY_TASK_FINISHED")]
    DailyTaskFinished = 29,
    #[serde(alias = "QUEST_COND_ACTIVITY_COND")]
    ActivityCond = 30,
    #[serde(alias = "QUEST_COND_ACTIVITY_OPEN")]
    ActivityOpen = 31,
    #[serde(alias = "QUEST_COND_DAILY_TASK_VAR_GT")]
    DailyTaskVarGt = 32,
    #[serde(alias = "QUEST_COND_DAILY_TASK_VAR_EQ")]
    DailyTaskVarEq = 33,
    #[serde(alias = "QUEST_COND_DAILY_TASK_VAR_LT")]
    DailyTaskVarLt = 34,
    #[serde(alias = "QUEST_COND_BARGAIN_ITEM_GT")]
    BargainItemGt = 35,
    #[serde(alias = "QUEST_COND_BARGAIN_ITEM_EQ")]
    BargainItemEq = 36,
    #[serde(alias = "QUEST_COND_BARGAIN_ITEM_LT")]
    BargainItemLt = 37,
    #[serde(alias = "QUEST_COND_COMPLETE_TALK")]
    CompleteTalk = 38,
    #[serde(alias = "QUEST_COND_NOT_HAVE_BLOSSOM_TALK")]
    NotHaveBlossomTalk = 39,
    #[serde(alias = "QUEST_COND_IS_CUR_BLOSSOM_TALK")]
    IsCurBlossomTalk = 40,
    #[serde(alias = "QUEST_COND_QUEST_NOT_RECEIVE")]
    QuestNotReceive = 41,
    #[serde(alias = "QUEST_COND_QUEST_SERVER_COND_VALID")]
    QuestServerCondValid = 42,
    #[serde(alias = "QUEST_COND_ACTIVITY_CLIENT_COND")]
    ActivityClientCond = 43,
    #[serde(alias = "QUEST_COND_QUEST_GLOBAL_VAR_EQUAL")]
    QuestGlobalVarEqual = 44,
    #[serde(alias = "QUEST_COND_QUEST_GLOBAL_VAR_GREATER")]
    QuestGlobalVarGreater = 45,
    #[serde(alias = "QUEST_COND_QUEST_GLOBAL_VAR_LESS")]
    QuestGlobalVarLess = 46,
    #[serde(alias = "QUEST_COND_PERSONAL_LINE_UNLOCK")]
    PersonalLineUnlock = 47,
    #[serde(alias = "QUEST_COND_CITY_REPUTATION_REQUEST")]
    CityReputationRequest = 48,
    #[serde(alias = "QUEST_COND_MAIN_COOP_START")]
    MainCoopStart = 49,
    #[serde(alias = "QUEST_COND_MAIN_COOP_ENTER_SAVE_POINT")]
    MainCoopEnterSavePoint = 50,
    #[serde(alias = "QUEST_COND_CITY_REPUTATION_LEVEL")]
    CityReputationLevel = 51,
    #[serde(alias = "QUEST_COND_CITY_REPUTATION_UNLOCK")]
    CityReputationUnlock = 52,
    #[serde(alias = "QUEST_COND_LUA_NOTIFY")]
    LuaNotify = 53,
    #[serde(alias = "QUEST_COND_CUR_CLIMATE")]
    CurClimate = 54,
    #[serde(alias = "QUEST_COND_ACTIVITY_END")]
    ActivityEnd = 55,
    #[serde(alias = "QUEST_COND_COOP_POINT_RUNNING")]
    CoopPointRunning = 56,
    #[serde(alias = "QUEST_COND_GADGET_TALK_STATE_EQUAL")]
    GadgetTalkStateEqual = 57,
    #[serde(alias = "QUEST_COND_AVATAR_FETTER_GT")]
    AvatarFetterGt = 58,
    #[serde(alias = "QUEST_COND_AVATAR_FETTER_EQ")]
    AvatarFetterEq = 59,
    #[serde(alias = "QUEST_COND_AVATAR_FETTER_LT")]
    AvatarFetterLt = 60,
    #[serde(alias = "QUEST_COND_NEW_HOMEWORLD_MOUDLE_UNLOCK")]
    NewHomeworldModuleUnlock = 61,
    #[serde(alias = "QUEST_COND_NEW_HOMEWORLD_LEVEL_REWARD")]
    NewHomeworldLevelReward = 62,
    #[serde(alias = "QUEST_COND_NEW_HOMEWORLD_MAKE_FINISH")]
    NewHomeworldMakeFinish = 63,
    #[serde(alias = "QUEST_COND_HOMEWORLD_NPC_EVENT")]
    HomeworldNpcEvent = 64,
    #[serde(alias = "QUEST_COND_TIME_VAR_GT_EQ")]
    TimeVarGtEq = 65,
    #[serde(alias = "QUEST_COND_TIME_VAR_PASS_DAY")]
    TimeVarPassDay = 66,
    #[serde(alias = "QUEST_COND_HOMEWORLD_NPC_NEW_TALK")]
    HomeworldNpcNewTalk = 67,
    #[serde(alias = "QUEST_COND_PLAYER_CHOOSE_MALE")]
    PlayerChooseMale = 68,
    #[serde(alias = "QUEST_COND_HISTORY_GOT_ANY_ITEM")]
    HistoryGotAnyItem = 69,
    #[serde(alias = "QUEST_COND_LEARNED_RECIPE")]
    LearnedRecipe = 70,
    #[serde(alias = "QUEST_COND_LUNARITE_REGION_UNLOCKED")]
    LunariteRegionUnlocked = 71,
    #[serde(alias = "QUEST_COND_LUNARITE_HAS_REGION_HINT_COUNT")]
    LunariteHasRegionHintCount = 72,
    #[serde(alias = "QUEST_COND_LUNARITE_COLLECT_FINISH")]
    LunariteCollectFinish = 73,
    #[serde(alias = "QUEST_COND_LUNARITE_MARK_ALL_FINISH")]
    LunariteMarkAllFinish = 74,
    #[serde(alias = "QUEST_COND_NEW_HOMEWORLD_SHOP_ITEM")]
    NewHomeworldShopItem = 75,
    #[serde(alias = "QUEST_COND_SCENE_POINT_UNLOCK")]
    ScenePointUnlock = 76,
    #[serde(alias = "QUEST_COND_SCENE_LEVEL_TAG_EQ")]
    SceneLevelTagEq = 77,
    #[serde(alias = "QUEST_COND_PLAYER_ENTER_REGION")]
    PlayerEnterRegion = 78,
    #[serde(alias = "QUEST_COND_ACTIVITY_SCHEDULE_OPEN_AND_COND")]
    ActivityScheduleOpenAndCond = 79,
    #[serde(alias = "QUEST_COND_ARANARA_COLLECTION_STATE_EQ")]
    AranaraCollectionStateEq = 80,
    #[serde(alias = "QUEST_COND_INFERENCE_FINISH_PAGE_CONCLUSION")]
    InferenceFinishPageConclusion = 81,
    #[serde(alias = "QUEST_COND_SCENE_LEVEL_TAG_VALID")]
    SceneLevelTagValid = 82,
    #[serde(alias = "QUEST_COND_NEW_HOMEWORLD_WOOD_EXCHANGE_UNLOCK")]
    NewHomeworldWoodExchangeUnlock = 83,
    #[serde(alias = "QUEST_COND_IN_CITY")]
    InCity = 84,
    #[serde(alias = "QUEST_COND_GCG_EXP_OVERFLOW")]
    GcgExpOverflow = 85,
    #[serde(alias = "QUEST_COND_GCG_NPC_TYPE")]
    GcgNpcType = 86,
    #[serde(alias = "QUEST_COND_ACTIVITY_NEW_FUNGUS_CAPTURE")]
    ActivityNewFungusCapture = 87,
    #[serde(alias = "QUEST_COND_GCG_LEVEL_REWARD_CAN_TAKE")]
    GcgLevelRewardCanTake = 88,
    #[serde(alias = "QUEST_COND_GCG_SHOP_NEW_GOODS")]
    GcgShopNewGoods = 89,
    #[serde(alias = "QUEST_COND_GCG_CHALLENGE_NEW_BOSS")]
    GcgChallengeNewBoss = 90,
    #[serde(alias = "QUEST_COND_GCG_CHALLENGE_NEW_CHAR")]
    GcgChallengeNewChar = 91,
    #[serde(alias = "QUEST_COND_GCG_LEVEL_UNLOCKED")]
    GcgLevelUnlocked = 92,
    #[serde(alias = "QUEST_COND_GCG_WORLD_CHALLENGE_RESULT")]
    GcgWorldChallengeResult = 93,
    #[serde(alias = "QUEST_COND_HIT_WANDERER_RENAME_EASTER_EGG")]
    HitWandererRenameEasterEgg = 94,
    #[serde(alias = "QUEST_COND_HIT_KEYWORD_EASTER_EGG")]
    HitKeywordEasterEgg = 95,
    #[serde(alias = "QUEST_COND_GCG_INVITE_TYPE")]
    GcgInviteType = 96,
    #[serde(alias = "QUEST_COND_TMPVALUE_HIT_NICKNAME")]
    TmpvalueHitNickname = 97,
    #[serde(alias = "QUEST_COND_NONE")]
    #[serde(other)]
    #[default]
    None = 0,
}
#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
#[repr(u32)]
pub enum QuestContent {
    #[serde(alias = "QUEST_CONTENT_KILL_MONSTER")]
    KillMonster = 1,
    #[serde(alias = "QUEST_CONTENT_COMPLETE_TALK")]
    CompleteTalk = 2,
    #[serde(alias = "QUEST_CONTENT_MONSTER_DIE")]
    MonsterDie = 3,
    #[serde(alias = "QUEST_CONTENT_FINISH_PLOT")]
    FinishPlot = 4,
    #[serde(alias = "QUEST_CONTENT_OBTAIN_ITEM")]
    ObtainItem = 5,
    #[serde(alias = "QUEST_CONTENT_TRIGGER_FIRE")]
    TriggerFire = 6,
    #[serde(alias = "QUEST_CONTENT_CLEAR_GROUP_MONSTER")]
    ClearGroupMonster = 7,
    #[serde(alias = "QUEST_CONTENT_NOT_FINISH_PLOT")]
    NotFinishPlot = 8,
    #[serde(alias = "QUEST_CONTENT_ENTER_DUNGEON")]
    EnterDungeon = 9,
    #[serde(alias = "QUEST_CONTENT_ENTER_MY_WORLD")]
    EnterMyWorld = 10,
    #[serde(alias = "QUEST_CONTENT_FINISH_DUNGEON")]
    FinishDungeon = 11,
    #[serde(alias = "QUEST_CONTENT_DESTROY_GADGET")]
    DestroyGadget = 12,
    #[serde(alias = "QUEST_CONTENT_OBTAIN_MATERIAL_WITH_SUBTYPE")]
    ObtainMaterialWithSubtype = 13,
    #[serde(alias = "QUEST_CONTENT_NICK_NAME")]
    NickName = 14,
    #[serde(alias = "QUEST_CONTENT_WORKTOP_SELECT")]
    WorktopSelect = 15,
    #[serde(alias = "QUEST_CONTENT_SEAL_BATTLE_RESULT")]
    SealBattleResult = 16,
    #[serde(alias = "QUEST_CONTENT_ENTER_ROOM")]
    EnterRoom = 17,
    #[serde(alias = "QUEST_CONTENT_GAME_TIME_TICK")]
    GameTimeTick = 18,
    #[serde(alias = "QUEST_CONTENT_FAIL_DUNGEON")]
    FailDungeon = 19,
    #[serde(alias = "QUEST_CONTENT_LUA_NOTIFY")]
    LuaNotify = 20,
    #[serde(alias = "QUEST_CONTENT_TEAM_DEAD")]
    TeamDead = 21,
    #[serde(alias = "QUEST_CONTENT_COMPLETE_ANY_TALK")]
    CompleteAnyTalk = 22,
    #[serde(alias = "QUEST_CONTENT_UNLOCK_TRANS_POINT")]
    UnlockTransPoint = 23,
    #[serde(alias = "QUEST_CONTENT_ADD_QUEST_PROGRESS")]
    AddQuestProgress = 24,
    #[serde(alias = "QUEST_CONTENT_INTERACT_GADGET")]
    InteractGadget = 25,
    #[serde(alias = "QUEST_CONTENT_DAILY_TASK_COMP_FINISH")]
    DailyTaskCompFinish = 26,
    #[serde(alias = "QUEST_CONTENT_FINISH_ITEM_GIVING")]
    FinishItemGiving = 27,
    #[serde(alias = "QUEST_CONTENT_SKILL")]
    Skill = 107,
    #[serde(alias = "QUEST_CONTENT_CITY_LEVEL_UP")]
    CityLevelUp = 109,
    #[serde(alias = "QUEST_CONTENT_PATTERN_GROUP_CLEAR_MONSTER")]
    PatternGroupClearMonster = 110,
    #[serde(alias = "QUEST_CONTENT_ITEM_LESS_THAN")]
    ItemLessThan = 111,
    #[serde(alias = "QUEST_CONTENT_PLAYER_LEVEL_UP")]
    PlayerLevelUp = 112,
    #[serde(alias = "QUEST_CONTENT_DUNGEON_OPEN_STATUE")]
    DungeonOpenStatue = 113,
    #[serde(alias = "QUEST_CONTENT_UNLOCK_AREA")]
    UnlockArea = 114,
    #[serde(alias = "QUEST_CONTENT_OPEN_CHEST_WITH_GADGET_ID")]
    OpenChestWithGadgetId = 115,
    #[serde(alias = "QUEST_CONTENT_UNLOCK_TRANS_POINT_WITH_TYPE")]
    UnlockTransPointWithType = 116,
    #[serde(alias = "QUEST_CONTENT_FINISH_DAILY_DUNGEON")]
    FinishDailyDungeon = 117,
    #[serde(alias = "QUEST_CONTENT_FINISH_WEEKLY_DUNGEON")]
    FinishWeeklyDungeon = 118,
    #[serde(alias = "QUEST_CONTENT_QUEST_VAR_EQUAL")]
    QuestVarEqual = 119,
    #[serde(alias = "QUEST_CONTENT_QUEST_VAR_GREATER")]
    QuestVarGreater = 120,
    #[serde(alias = "QUEST_CONTENT_QUEST_VAR_LESS")]
    QuestVarLess = 121,
    #[serde(alias = "QUEST_CONTENT_OBTAIN_VARIOUS_ITEM")]
    ObtainVariousItem = 122,
    #[serde(alias = "QUEST_CONTENT_FINISH_TOWER_LEVEL")]
    FinishTowerLevel = 123,
    #[serde(alias = "QUEST_CONTENT_BARGAIN_SUCC")]
    BargainSucc = 124,
    #[serde(alias = "QUEST_CONTENT_BARGAIN_FAIL")]
    BargainFail = 125,
    #[serde(alias = "QUEST_CONTENT_ITEM_LESS_THAN_BARGAIN")]
    ItemLessThanBargain = 126,
    #[serde(alias = "QUEST_CONTENT_ACTIVITY_TRIGGER_FAILED")]
    ActivityTriggerFailed = 127,
    #[serde(alias = "QUEST_CONTENT_MAIN_COOP_ENTER_SAVE_POINT")]
    MainCoopEnterSavePoint = 128,
    #[serde(alias = "QUEST_CONTENT_ANY_MANUAL_TRANSPORT")]
    AnyManualTransport = 129,
    #[serde(alias = "QUEST_CONTENT_USE_ITEM")]
    UseItem = 130,
    #[serde(alias = "QUEST_CONTENT_MAIN_COOP_ENTER_ANY_SAVE_POINT")]
    MainCoopEnterAnySavePoint = 131,
    #[serde(alias = "QUEST_CONTENT_ENTER_MY_HOME_WORLD")]
    EnterMyHomeWorld = 132,
    #[serde(alias = "QUEST_CONTENT_ENTER_MY_WORLD_SCENE")]
    EnterMyWorldScene = 133,
    #[serde(alias = "QUEST_CONTENT_TIME_VAR_GT_EQ")]
    TimeVarGtEq = 134,
    #[serde(alias = "QUEST_CONTENT_TIME_VAR_PASS_DAY")]
    TimeVarPassDay = 135,
    #[serde(alias = "QUEST_CONTENT_QUEST_STATE_EQUAL")]
    QuestStateEqual = 136,
    #[serde(alias = "QUEST_CONTENT_QUEST_STATE_NOT_EQUAL")]
    QuestStateNotEqual = 137,
    #[serde(alias = "QUEST_CONTENT_UNLOCKED_RECIPE")]
    UnlockedRecipe = 138,
    #[serde(alias = "QUEST_CONTENT_NOT_UNLOCKED_RECIPE")]
    NotUnlockedRecipe = 139,
    #[serde(alias = "QUEST_CONTENT_FISHING_SUCC")]
    FishingSucc = 140,
    #[serde(alias = "QUEST_CONTENT_ENTER_ROGUE_DUNGEON")]
    EnterRogueDungeon = 141,
    #[serde(alias = "QUEST_CONTENT_USE_WIDGET")]
    UseWidget = 142,
    #[serde(alias = "QUEST_CONTENT_CAPTURE_SUCC")]
    CaptureSucc = 143,
    #[serde(alias = "QUEST_CONTENT_CAPTURE_USE_CAPTURETAG_LIST")]
    CaptureUseCapturetagList = 144,
    #[serde(alias = "QUEST_CONTENT_CAPTURE_USE_MATERIAL_LIST")]
    CaptureUseMaterialList = 145,
    #[serde(alias = "QUEST_CONTENT_ENTER_VEHICLE")]
    EnterVehicle = 146,
    #[serde(alias = "QUEST_CONTENT_SCENE_LEVEL_TAG_EQ")]
    SceneLevelTagEq = 147,
    #[serde(alias = "QUEST_CONTENT_LEAVE_SCENE")]
    LeaveScene = 148,
    #[serde(alias = "QUEST_CONTENT_LEAVE_SCENE_RANGE")]
    LeaveSceneRange = 149,
    #[serde(alias = "QUEST_CONTENT_IRODORI_FINISH_FLOWER_COMBINATION")]
    IrodoriFinishFlowerCombination = 151,
    #[serde(alias = "QUEST_CONTENT_IRODORI_POETRY_REACH_MIN_PROGRESS")]
    IrodoriPoetryReachMinProgress = 152,
    #[serde(alias = "QUEST_CONTENT_IRODORI_POETRY_FINISH_FILL_POETRY")]
    IrodoriPoetryFinishFillPoetry = 153,
    #[serde(alias = "QUEST_CONTENT_NONE")]
    #[serde(other)]
    #[default]
    None = 0,
}

impl QuestContent {
    /// 从 u32 值转换为 QuestContent，未知值返回 None
    pub fn from_u32(value: u32) -> Option<Self> {
        match value {
            0 => Some(QuestContent::None),
            1 => Some(QuestContent::KillMonster),
            2 => Some(QuestContent::CompleteTalk),
            3 => Some(QuestContent::MonsterDie),
            4 => Some(QuestContent::FinishPlot),
            5 => Some(QuestContent::ObtainItem),
            6 => Some(QuestContent::TriggerFire),
            7 => Some(QuestContent::ClearGroupMonster),
            8 => Some(QuestContent::NotFinishPlot),
            9 => Some(QuestContent::EnterDungeon),
            10 => Some(QuestContent::EnterMyWorld),
            11 => Some(QuestContent::FinishDungeon),
            12 => Some(QuestContent::DestroyGadget),
            13 => Some(QuestContent::ObtainMaterialWithSubtype),
            14 => Some(QuestContent::NickName),
            15 => Some(QuestContent::WorktopSelect),
            16 => Some(QuestContent::SealBattleResult),
            17 => Some(QuestContent::EnterRoom),
            18 => Some(QuestContent::GameTimeTick),
            19 => Some(QuestContent::FailDungeon),
            20 => Some(QuestContent::LuaNotify),
            21 => Some(QuestContent::TeamDead),
            22 => Some(QuestContent::CompleteAnyTalk),
            23 => Some(QuestContent::UnlockTransPoint),
            24 => Some(QuestContent::AddQuestProgress),
            25 => Some(QuestContent::InteractGadget),
            26 => Some(QuestContent::DailyTaskCompFinish),
            27 => Some(QuestContent::FinishItemGiving),
            107 => Some(QuestContent::Skill),
            109 => Some(QuestContent::CityLevelUp),
            110 => Some(QuestContent::PatternGroupClearMonster),
            111 => Some(QuestContent::ItemLessThan),
            112 => Some(QuestContent::PlayerLevelUp),
            113 => Some(QuestContent::DungeonOpenStatue),
            114 => Some(QuestContent::UnlockArea),
            115 => Some(QuestContent::OpenChestWithGadgetId),
            116 => Some(QuestContent::UnlockTransPointWithType),
            117 => Some(QuestContent::FinishDailyDungeon),
            118 => Some(QuestContent::FinishWeeklyDungeon),
            119 => Some(QuestContent::QuestVarEqual),
            120 => Some(QuestContent::QuestVarGreater),
            121 => Some(QuestContent::QuestVarLess),
            122 => Some(QuestContent::ObtainVariousItem),
            123 => Some(QuestContent::FinishTowerLevel),
            124 => Some(QuestContent::BargainSucc),
            125 => Some(QuestContent::BargainFail),
            126 => Some(QuestContent::ItemLessThanBargain),
            127 => Some(QuestContent::ActivityTriggerFailed),
            128 => Some(QuestContent::MainCoopEnterSavePoint),
            129 => Some(QuestContent::AnyManualTransport),
            130 => Some(QuestContent::UseItem),
            131 => Some(QuestContent::MainCoopEnterAnySavePoint),
            132 => Some(QuestContent::EnterMyHomeWorld),
            133 => Some(QuestContent::EnterMyWorldScene),
            134 => Some(QuestContent::TimeVarGtEq),
            135 => Some(QuestContent::TimeVarPassDay),
            136 => Some(QuestContent::QuestStateEqual),
            137 => Some(QuestContent::QuestStateNotEqual),
            138 => Some(QuestContent::UnlockedRecipe),
            139 => Some(QuestContent::NotUnlockedRecipe),
            140 => Some(QuestContent::FishingSucc),
            141 => Some(QuestContent::EnterRogueDungeon),
            142 => Some(QuestContent::UseWidget),
            143 => Some(QuestContent::CaptureSucc),
            144 => Some(QuestContent::CaptureUseCapturetagList),
            145 => Some(QuestContent::CaptureUseMaterialList),
            146 => Some(QuestContent::EnterVehicle),
            147 => Some(QuestContent::SceneLevelTagEq),
            148 => Some(QuestContent::LeaveScene),
            149 => Some(QuestContent::LeaveSceneRange),
            151 => Some(QuestContent::IrodoriFinishFlowerCombination),
            152 => Some(QuestContent::IrodoriPoetryReachMinProgress),
            153 => Some(QuestContent::IrodoriPoetryFinishFillPoetry),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct TalkData {
    pub id: u32,
    pub hero_talk: InternString,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestExecParam {
    #[serde(default)]
    #[serde(alias = "_type")]
    pub r#type: QuestExec,
    #[serde(default)]
    #[serde(alias = "_param")]
    pub param: Vec<InternString>,
    #[serde(default)]
    #[serde(alias = "_count")]
    pub count: u32,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestCondition<T> {
    #[serde(default)]
    #[serde(alias = "_type")]
    pub r#type: T,
    #[serde(default)]
    #[serde(alias = "_param")]
    pub param: Vec<u32>,
    #[serde(default)]
    #[serde(alias = "_param_str")]
    pub param_str: InternString,
    #[serde(default)]
    #[serde(alias = "_count")]
    pub count: u32,
}

pub type QuestAcceptCondition = QuestCondition<QuestCond>;
pub type QuestContentCondition = QuestCondition<QuestContent>;

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemParamData {
    #[serde(alias = "itemId")]
    pub id: u32,
    #[serde(alias = "_count")]
    pub count: u32,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Guide {
    #[serde(default)]
    #[serde(alias = "_type")]
    pub r#type: InternString,
    #[serde(default)]
    pub param: Vec<InternString>,
    #[serde(default)]
    pub guide_scene: u32,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubQuestData {
    pub main_id: u32,
    pub sub_id: u32,
    #[serde(default)]
    pub order: u32,
    #[serde(default)]
    pub is_rewind: bool,
    #[serde(default)]
    pub finish_parent: bool,
    #[serde(default)]
    pub fail_parent: bool,
    #[serde(default)]
    pub desc_text_map_hash: u64,
    #[serde(default)]
    pub accept_cond_comb: LogicType,
    #[serde(default)]
    pub finish_cond_comb: LogicType,
    #[serde(default)]
    pub fail_cond_comb: LogicType,

    #[serde(default)]
    #[serde(alias = "beginCond")]
    pub accept_cond: Vec<QuestAcceptCondition>,
    #[serde(default)]
    pub finish_cond: Vec<QuestContentCondition>,
    #[serde(default)]
    pub fail_cond: Vec<QuestContentCondition>,

    #[serde(default)]
    pub accept_exec: Vec<QuestExecParam>,
    #[serde(default)]
    pub finish_exec: Vec<QuestExecParam>,
    #[serde(default)]
    pub fail_exec: Vec<QuestExecParam>,

    #[serde(default)]
    pub guide: Guide,
    #[serde(default)]
    pub trial_avatar_list: Vec<u32>,
    #[serde(default)]
    pub gain_items: Vec<ItemParamData>,
    #[serde(default)]
    pub exclusive_npc_list: Vec<u32>,
    #[serde(default)]
    pub exclusive_place_list: Vec<u32>,
    #[serde(default)]
    pub shared_npc_list: Vec<u32>,
    #[serde(default)]
    pub shared_place_list: Vec<u32>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestConfig {
    pub id: u32,
    #[serde(default)]
    pub series: u32,
    #[serde(default)]
    #[serde(alias = "type")]
    pub r#type: QuestType,
    #[serde(default)]
    pub title_text_map_hash: u64,
    #[serde(default)]
    pub suggest_track_main_quest_list: Vec<u32>,
    #[serde(default)]
    pub reward_id_list: Vec<u32>,
    pub sub_quests: Option<Vec<SubQuestData>>,
    #[serde(default)]
    pub talks: Vec<TalkData>,
    #[serde(default)]
    pub preload_lua_list: Vec<InternString>,
}

pub fn load_quest_configs_from_bin(bin_output_path: &str) {
    use rayon::prelude::*;

    let entries: Vec<_> = fs::read_dir(format!("{bin_output_path}/Quest/"))
        .unwrap()
        .collect();

    let results: Vec<(HashMap<u32, QuestConfig>, HashMap<u32, SubQuestData>)> = entries
        .into_par_iter()
        .filter_map(|entry| match entry {
            Ok(entry) => {
                let file_name = entry.file_name().to_string_lossy().replace(".json", "");
                match u32::from_str(file_name.as_str()) {
                    Ok(file_quest_id) => {
                        let json = std::fs::read(entry.path()).ok()?;
                        let result: serde_json::Result<QuestConfig> =
                            serde_json::from_slice(&*json);
                        match result {
                            Ok(config) => {
                                if config.sub_quests.is_none() {
                                    eprintln!(
                                        "quest : parent_quest_id:{} no sub_quests",
                                        file_quest_id
                                    );
                                    return None;
                                };
                                let mut sub_map = HashMap::new();

                                if let Some(ref sub_quests) = config.sub_quests {
                                    for sub_quest in sub_quests {
                                        sub_map.insert(sub_quest.sub_id, sub_quest.clone());
                                    }
                                }

                                let mut quest_map = HashMap::new();
                                quest_map.insert(config.id, config);

                                Some((quest_map, sub_map))
                            }
                            Err(error) => {
                                println!("error :{} quest_id:{}", error, file_quest_id);
                                None
                            }
                        }
                    }
                    Err(_) => None,
                }
            }
            _ => None,
        })
        .collect();

    let (data, sub_data) = results.into_iter().fold(
        (HashMap::new(), HashMap::new()),
        |(mut quest_acc, mut sub_acc), (quest_map, sub_map)| {
            quest_acc.extend(quest_map);
            sub_acc.extend(sub_map);
            (quest_acc, sub_acc)
        },
    );

    let _ = QUEST_CONFIG_COLLECTION.set(Arc::new(data));
    let _ = SUB_QUEST_CONFIG_COLLECTION.set(Arc::new(sub_data));
}
