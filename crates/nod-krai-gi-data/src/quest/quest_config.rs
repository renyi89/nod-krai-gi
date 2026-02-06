use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::sync::Arc;

pub static QUEST_CONFIG_COLLECTION: std::sync::OnceLock<Arc<HashMap<u32, QuestConfig>>> =
    std::sync::OnceLock::new();

pub static SUB_QUEST_CONFIG_COLLECTION: std::sync::OnceLock<Arc<HashMap<u32, SubQuestData>>> =
    std::sync::OnceLock::new();

#[derive(Debug, Default, Clone, serde::Deserialize)]
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
#[derive(Debug, Default, Clone, serde::Deserialize)]
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

#[derive(Debug, Default, Clone, serde::Deserialize)]
pub enum QuestExec {
    #[serde(alias = "QUEST_EXEC_NONE")]
    #[default]
    None,

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

    #[serde(other)]
    #[serde(alias = "QUEST_EXEC_UNKNOWN")]
    Unknown,
}
#[derive(Debug, Default, Clone, serde::Deserialize)]
pub enum QuestCond {
    #[serde(alias = "QUEST_COND_NONE")]
    #[default]
    None,

    #[serde(alias = "QUEST_COND_STATE_EQUAL")]
    StateEqual,

    #[serde(alias = "QUEST_COND_STATE_NOT_EQUAL")]
    StateNotEqual,

    #[serde(alias = "QUEST_COND_PACK_HAVE_ITEM")]
    PackHaveItem,

    #[serde(alias = "QUEST_COND_AVATAR_ELEMENT_EQUAL")]
    AvatarElementEqual,

    #[serde(alias = "QUEST_COND_AVATAR_ELEMENT_NOT_EQUAL")]
    AvatarElementNotEqual,

    #[serde(alias = "QUEST_COND_AVATAR_CAN_CHANGE_ELEMENT")]
    AvatarCanChangeElement,

    #[serde(alias = "QUEST_COND_CITY_LEVEL_EQUAL_GREATER")]
    CityLevelEqualGreater,

    #[serde(alias = "QUEST_COND_ITEM_NUM_LESS_THAN")]
    ItemNumLessThan,

    #[serde(alias = "QUEST_COND_DAILY_TASK_START")]
    DailyTaskStart,

    #[serde(alias = "QUEST_COND_OPEN_STATE_EQUAL")]
    OpenStateEqual,

    #[serde(alias = "QUEST_COND_DAILY_TASK_OPEN")]
    DailyTaskOpen,

    #[serde(alias = "QUEST_COND_DAILY_TASK_REWARD_CAN_GET")]
    DailyTaskRewardCanGet,

    #[serde(alias = "QUEST_COND_DAILY_TASK_REWARD_RECEIVED")]
    DailyTaskRewardReceived,

    #[serde(alias = "QUEST_COND_PLAYER_LEVEL_REWARD_CAN_GET")]
    PlayerLevelRewardCanGet,

    #[serde(alias = "QUEST_COND_EXPLORATION_REWARD_CAN_GET")]
    ExplorationRewardCanGet,

    #[serde(alias = "QUEST_COND_IS_WORLD_OWNER")]
    IsWorldOwner,

    #[serde(alias = "QUEST_COND_PLAYER_LEVEL_EQUAL_GREATER")]
    PlayerLevelEqualGreater,

    #[serde(alias = "QUEST_COND_SCENE_AREA_UNLOCKED")]
    SceneAreaUnlocked,

    #[serde(alias = "QUEST_COND_ITEM_GIVING_ACTIVED")]
    ItemGivingActived,

    #[serde(alias = "QUEST_COND_ITEM_GIVING_FINISHED")]
    ItemGivingFinished,

    #[serde(alias = "QUEST_COND_IS_DAYTIME")]
    IsDaytime,

    #[serde(alias = "QUEST_COND_CURRENT_AVATAR")]
    CurrentAvatar,

    #[serde(alias = "QUEST_COND_CURRENT_AREA")]
    CurrentArea,

    #[serde(alias = "QUEST_COND_QUEST_VAR_EQUAL")]
    QuestVarEqual,

    #[serde(alias = "QUEST_COND_QUEST_VAR_GREATER")]
    QuestVarGreater,

    #[serde(alias = "QUEST_COND_QUEST_VAR_LESS")]
    QuestVarLess,

    #[serde(alias = "QUEST_COND_FORGE_HAVE_FINISH")]
    ForgeHaveFinish,

    #[serde(alias = "QUEST_COND_DAILY_TASK_IN_PROGRESS")]
    DailyTaskInProgress,

    #[serde(alias = "QUEST_COND_DAILY_TASK_FINISHED")]
    DailyTaskFinished,

    #[serde(alias = "QUEST_COND_ACTIVITY_COND")]
    ActivityCond,

    #[serde(alias = "QUEST_COND_ACTIVITY_OPEN")]
    ActivityOpen,

    #[serde(alias = "QUEST_COND_DAILY_TASK_VAR_GT")]
    DailyTaskVarGt,

    #[serde(alias = "QUEST_COND_DAILY_TASK_VAR_EQ")]
    DailyTaskVarEq,

    #[serde(alias = "QUEST_COND_DAILY_TASK_VAR_LT")]
    DailyTaskVarLt,

    #[serde(alias = "QUEST_COND_BARGAIN_ITEM_GT")]
    BargainItemGt,

    #[serde(alias = "QUEST_COND_BARGAIN_ITEM_EQ")]
    BargainItemEq,

    #[serde(alias = "QUEST_COND_BARGAIN_ITEM_LT")]
    BargainItemLt,

    #[serde(alias = "QUEST_COND_COMPLETE_TALK")]
    CompleteTalk,

    #[serde(alias = "QUEST_COND_NOT_HAVE_BLOSSOM_TALK")]
    NotHaveBlossomTalk,

    #[serde(alias = "QUEST_COND_IS_CUR_BLOSSOM_TALK")]
    IsCurBlossomTalk,

    #[serde(alias = "QUEST_COND_QUEST_NOT_RECEIVE")]
    QuestNotReceive,

    #[serde(alias = "QUEST_COND_QUEST_SERVER_COND_VALID")]
    QuestServerCondValid,

    #[serde(alias = "QUEST_COND_ACTIVITY_CLIENT_COND")]
    ActivityClientCond,

    #[serde(alias = "QUEST_COND_QUEST_GLOBAL_VAR_EQUAL")]
    QuestGlobalVarEqual,

    #[serde(alias = "QUEST_COND_QUEST_GLOBAL_VAR_GREATER")]
    QuestGlobalVarGreater,

    #[serde(alias = "QUEST_COND_QUEST_GLOBAL_VAR_LESS")]
    QuestGlobalVarLess,

    #[serde(alias = "QUEST_COND_PERSONAL_LINE_UNLOCK")]
    PersonalLineUnlock,

    #[serde(alias = "QUEST_COND_CITY_REPUTATION_REQUEST")]
    CityReputationRequest,

    #[serde(alias = "QUEST_COND_MAIN_COOP_START")]
    MainCoopStart,

    #[serde(alias = "QUEST_COND_MAIN_COOP_ENTER_SAVE_POINT")]
    MainCoopEnterSavePoint,

    #[serde(alias = "QUEST_COND_CITY_REPUTATION_LEVEL")]
    CityReputationLevel,

    #[serde(alias = "QUEST_COND_CITY_REPUTATION_UNLOCK")]
    CityReputationUnlock,

    #[serde(alias = "QUEST_COND_LUA_NOTIFY")]
    LuaNotify,

    #[serde(alias = "QUEST_COND_CUR_CLIMATE")]
    CurClimate,

    #[serde(alias = "QUEST_COND_ACTIVITY_END")]
    ActivityEnd,

    #[serde(alias = "QUEST_COND_COOP_POINT_RUNNING")]
    CoopPointRunning,

    #[serde(alias = "QUEST_COND_GADGET_TALK_STATE_EQUAL")]
    GadgetTalkStateEqual,

    #[serde(alias = "QUEST_COND_AVATAR_FETTER_GT")]
    AvatarFetterGt,

    #[serde(alias = "QUEST_COND_AVATAR_FETTER_EQ")]
    AvatarFetterEq,

    #[serde(alias = "QUEST_COND_AVATAR_FETTER_LT")]
    AvatarFetterLt,

    #[serde(alias = "QUEST_COND_NEW_HOMEWORLD_MOUDLE_UNLOCK")]
    NewHomeworldModuleUnlock,

    #[serde(alias = "QUEST_COND_NEW_HOMEWORLD_LEVEL_REWARD")]
    NewHomeworldLevelReward,

    #[serde(alias = "QUEST_COND_NEW_HOMEWORLD_MAKE_FINISH")]
    NewHomeworldMakeFinish,

    #[serde(alias = "QUEST_COND_HOMEWORLD_NPC_EVENT")]
    HomeworldNpcEvent,

    #[serde(alias = "QUEST_COND_TIME_VAR_GT_EQ")]
    TimeVarGtEq,

    #[serde(alias = "QUEST_COND_TIME_VAR_PASS_DAY")]
    TimeVarPassDay,

    #[serde(alias = "QUEST_COND_HOMEWORLD_NPC_NEW_TALK")]
    HomeworldNpcNewTalk,

    #[serde(alias = "QUEST_COND_PLAYER_CHOOSE_MALE")]
    PlayerChooseMale,

    #[serde(alias = "QUEST_COND_HISTORY_GOT_ANY_ITEM")]
    HistoryGotAnyItem,

    #[serde(alias = "QUEST_COND_LEARNED_RECIPE")]
    LearnedRecipe,

    #[serde(alias = "QUEST_COND_LUNARITE_REGION_UNLOCKED")]
    LunariteRegionUnlocked,

    #[serde(alias = "QUEST_COND_LUNARITE_HAS_REGION_HINT_COUNT")]
    LunariteHasRegionHintCount,

    #[serde(alias = "QUEST_COND_LUNARITE_COLLECT_FINISH")]
    LunariteCollectFinish,

    #[serde(alias = "QUEST_COND_LUNARITE_MARK_ALL_FINISH")]
    LunariteMarkAllFinish,

    #[serde(alias = "QUEST_COND_NEW_HOMEWORLD_SHOP_ITEM")]
    NewHomeworldShopItem,

    #[serde(alias = "QUEST_COND_SCENE_POINT_UNLOCK")]
    ScenePointUnlock,

    #[serde(alias = "QUEST_COND_SCENE_LEVEL_TAG_EQ")]
    SceneLevelTagEq,

    #[serde(alias = "QUEST_COND_PLAYER_ENTER_REGION")]
    PlayerEnterRegion,

    #[serde(other)]
    #[serde(alias = "QUEST_COND_UNKNOWN")]
    Unknown,
}
#[derive(Debug, Default, Clone, serde::Deserialize)]
pub enum QuestContent {
    #[serde(alias = "QUEST_CONTENT_NONE")]
    #[default]
    None,

    #[serde(alias = "QUEST_CONTENT_KILL_MONSTER")]
    KillMonster,

    #[serde(alias = "QUEST_CONTENT_COMPLETE_TALK")]
    CompleteTalk,

    #[serde(alias = "QUEST_CONTENT_MONSTER_DIE")]
    MonsterDie,

    #[serde(alias = "QUEST_CONTENT_FINISH_PLOT")]
    FinishPlot,

    #[serde(alias = "QUEST_CONTENT_OBTAIN_ITEM")]
    ObtainItem,

    #[serde(alias = "QUEST_CONTENT_TRIGGER_FIRE")]
    TriggerFire,

    #[serde(alias = "QUEST_CONTENT_CLEAR_GROUP_MONSTER")]
    ClearGroupMonster,

    #[serde(alias = "QUEST_CONTENT_NOT_FINISH_PLOT")]
    NotFinishPlot,

    #[serde(alias = "QUEST_CONTENT_ENTER_DUNGEON")]
    EnterDungeon,

    #[serde(alias = "QUEST_CONTENT_ENTER_MY_WORLD")]
    EnterMyWorld,

    #[serde(alias = "QUEST_CONTENT_FINISH_DUNGEON")]
    FinishDungeon,

    #[serde(alias = "QUEST_CONTENT_DESTROY_GADGET")]
    DestroyGadget,

    #[serde(alias = "QUEST_CONTENT_OBTAIN_MATERIAL_WITH_SUBTYPE")]
    ObtainMaterialWithSubtype,

    #[serde(alias = "QUEST_CONTENT_NICK_NAME")]
    NickName,

    #[serde(alias = "QUEST_CONTENT_WORKTOP_SELECT")]
    WorktopSelect,

    #[serde(alias = "QUEST_CONTENT_SEAL_BATTLE_RESULT")]
    SealBattleResult,

    #[serde(alias = "QUEST_CONTENT_ENTER_ROOM")]
    EnterRoom,

    #[serde(alias = "QUEST_CONTENT_GAME_TIME_TICK")]
    GameTimeTick,

    #[serde(alias = "QUEST_CONTENT_FAIL_DUNGEON")]
    FailDungeon,

    #[serde(alias = "QUEST_CONTENT_LUA_NOTIFY")]
    LuaNotify,

    #[serde(alias = "QUEST_CONTENT_TEAM_DEAD")]
    TeamDead,

    #[serde(alias = "QUEST_CONTENT_COMPLETE_ANY_TALK")]
    CompleteAnyTalk,

    #[serde(alias = "QUEST_CONTENT_UNLOCK_TRANS_POINT")]
    UnlockTransPoint,

    #[serde(alias = "QUEST_CONTENT_ADD_QUEST_PROGRESS")]
    AddQuestProgress,

    #[serde(alias = "QUEST_CONTENT_INTERACT_GADGET")]
    InteractGadget,

    #[serde(alias = "QUEST_CONTENT_DAILY_TASK_COMP_FINISH")]
    DailyTaskCompFinish,

    #[serde(alias = "QUEST_CONTENT_FINISH_ITEM_GIVING")]
    FinishItemGiving,

    #[serde(alias = "QUEST_CONTENT_SKILL")]
    Skill,

    #[serde(alias = "QUEST_CONTENT_CITY_LEVEL_UP")]
    CityLevelUp,

    #[serde(alias = "QUEST_CONTENT_PATTERN_GROUP_CLEAR_MONSTER")]
    PatternGroupClearMonster,

    #[serde(alias = "QUEST_CONTENT_ITEM_LESS_THAN")]
    ItemLessThan,

    #[serde(alias = "QUEST_CONTENT_PLAYER_LEVEL_UP")]
    PlayerLevelUp,

    #[serde(alias = "QUEST_CONTENT_DUNGEON_OPEN_STATUE")]
    DungeonOpenStatue,

    #[serde(alias = "QUEST_CONTENT_UNLOCK_AREA")]
    UnlockArea,

    #[serde(alias = "QUEST_CONTENT_OPEN_CHEST_WITH_GADGET_ID")]
    OpenChestWithGadgetId,

    #[serde(alias = "QUEST_CONTENT_UNLOCK_TRANS_POINT_WITH_TYPE")]
    UnlockTransPointWithType,

    #[serde(alias = "QUEST_CONTENT_FINISH_DAILY_DUNGEON")]
    FinishDailyDungeon,

    #[serde(alias = "QUEST_CONTENT_FINISH_WEEKLY_DUNGEON")]
    FinishWeeklyDungeon,

    #[serde(alias = "QUEST_CONTENT_QUEST_VAR_EQUAL")]
    QuestVarEqual,

    #[serde(alias = "QUEST_CONTENT_QUEST_VAR_GREATER")]
    QuestVarGreater,

    #[serde(alias = "QUEST_CONTENT_QUEST_VAR_LESS")]
    QuestVarLess,

    #[serde(alias = "QUEST_CONTENT_OBTAIN_VARIOUS_ITEM")]
    ObtainVariousItem,

    #[serde(alias = "QUEST_CONTENT_FINISH_TOWER_LEVEL")]
    FinishTowerLevel,

    #[serde(alias = "QUEST_CONTENT_BARGAIN_SUCC")]
    BargainSucc,

    #[serde(alias = "QUEST_CONTENT_BARGAIN_FAIL")]
    BargainFail,

    #[serde(alias = "QUEST_CONTENT_ITEM_LESS_THAN_BARGAIN")]
    ItemLessThanBargain,

    #[serde(alias = "QUEST_CONTENT_ACTIVITY_TRIGGER_FAILED")]
    ActivityTriggerFailed,

    #[serde(alias = "QUEST_CONTENT_MAIN_COOP_ENTER_SAVE_POINT")]
    MainCoopEnterSavePoint,

    #[serde(alias = "QUEST_CONTENT_ANY_MANUAL_TRANSPORT")]
    AnyManualTransport,

    #[serde(alias = "QUEST_CONTENT_USE_ITEM")]
    UseItem,

    #[serde(alias = "QUEST_CONTENT_MAIN_COOP_ENTER_ANY_SAVE_POINT")]
    MainCoopEnterAnySavePoint,

    #[serde(alias = "QUEST_CONTENT_ENTER_MY_HOME_WORLD")]
    EnterMyHomeWorld,

    #[serde(alias = "QUEST_CONTENT_ENTER_MY_WORLD_SCENE")]
    EnterMyWorldScene,

    #[serde(alias = "QUEST_CONTENT_TIME_VAR_GT_EQ")]
    TimeVarGtEq,

    #[serde(alias = "QUEST_CONTENT_TIME_VAR_PASS_DAY")]
    TimeVarPassDay,

    #[serde(alias = "QUEST_CONTENT_QUEST_STATE_EQUAL")]
    QuestStateEqual,

    #[serde(alias = "QUEST_CONTENT_QUEST_STATE_NOT_EQUAL")]
    QuestStateNotEqual,

    #[serde(alias = "QUEST_CONTENT_UNLOCKED_RECIPE")]
    UnlockedRecipe,

    #[serde(alias = "QUEST_CONTENT_NOT_UNLOCKED_RECIPE")]
    NotUnlockedRecipe,

    #[serde(alias = "QUEST_CONTENT_FISHING_SUCC")]
    FishingSucc,

    #[serde(alias = "QUEST_CONTENT_ENTER_ROGUE_DUNGEON")]
    EnterRogueDungeon,

    #[serde(alias = "QUEST_CONTENT_USE_WIDGET")]
    UseWidget,

    #[serde(alias = "QUEST_CONTENT_CAPTURE_SUCC")]
    CaptureSucc,

    #[serde(alias = "QUEST_CONTENT_CAPTURE_USE_CAPTURETAG_LIST")]
    CaptureUseCapturetagList,

    #[serde(alias = "QUEST_CONTENT_CAPTURE_USE_MATERIAL_LIST")]
    CaptureUseMaterialList,

    #[serde(alias = "QUEST_CONTENT_ENTER_VEHICLE")]
    EnterVehicle,

    #[serde(alias = "QUEST_CONTENT_SCENE_LEVEL_TAG_EQ")]
    SceneLevelTagEq,

    #[serde(alias = "QUEST_CONTENT_LEAVE_SCENE")]
    LeaveScene,

    #[serde(alias = "QUEST_CONTENT_LEAVE_SCENE_RANGE")]
    LeaveSceneRange,

    #[serde(alias = "QUEST_CONTENT_IRODORI_FINISH_FLOWER_COMBINATION")]
    IrodoriFinishFlowerCombination,

    #[serde(alias = "QUEST_CONTENT_IRODORI_POETRY_REACH_MIN_PROGRESS")]
    IrodoriPoetryReachMinProgress,

    #[serde(alias = "QUEST_CONTENT_IRODORI_POETRY_FINISH_FILL_POETRY")]
    IrodoriPoetryFinishFillPoetry,

    #[serde(alias = "QUEST_CONTENT_ACTIVITY_TRIGGER_UPDATE")]
    ActivityTriggerUpdate,

    #[serde(alias = "QUEST_CONTENT_GADGET_STATE_CHANGE")]
    GadgetStateChange,

    #[serde(other)]
    #[serde(alias = "QUEST_CONTENT_UNKNOWN")]
    Unknown,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct TalkData {
    pub id: u32,
    pub hero_talk: String,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestExecParam {
    #[serde(default)]
    #[serde(alias = "_type")]
    pub r#type: QuestExec,
    #[serde(default)]
    #[serde(alias = "_param")]
    pub param: Vec<String>,
    #[serde(default)]
    #[serde(alias = "_count")]
    pub count: String,
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
    pub param_str: String,
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
    pub r#type: String,
    #[serde(default)]
    pub param: Vec<String>,
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
    pub desc_text_map_hash: u64,
    #[serde(default)]
    pub accept_cond_comb: LogicType,
    #[serde(default)]
    pub finish_cond_comb: LogicType,
    #[serde(default)]
    pub fail_cond_comb: LogicType,

    #[serde(default)]
    #[serde(alias = "acceptCond")]
    pub begin_cond: Vec<QuestAcceptCondition>,
    #[serde(default)]
    pub finish_cond: Vec<QuestContentCondition>,
    #[serde(default)]
    pub fail_cond: Vec<QuestContentCondition>,

    #[serde(default)]
    pub begin_exec: Vec<QuestExecParam>,
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
    pub preload_lua_list: Vec<String>,
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
                        let file = fs::File::open(entry.path()).ok()?;
                        let content = std::io::BufReader::new(file);
                        let result: serde_json::Result<QuestConfig> =
                            serde_json::from_reader(content);
                        match result {
                            Ok(mut config) => {
                                if config.sub_quests.is_none() {
                                    println!(
                                        "error : no sub_quests parent_quest_id:{}",
                                        file_quest_id
                                    );
                                    return None;
                                };
                                let mut sub_map = HashMap::new();

                                if let Some(sub_quests) = config.sub_quests.take() {
                                    for sub_quest in sub_quests {
                                        sub_map.insert(sub_quest.sub_id, sub_quest);
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
