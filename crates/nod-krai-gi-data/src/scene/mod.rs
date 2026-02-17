pub mod scene_block_template;
pub mod scene_config_template;
pub mod scene_group_template;
pub mod scene_point_config;
pub mod script_cache;

pub use scene_point_config::*;

use crate::excel::common::LuaEnum;
use crate::lua_enum;
use mlua::{IntoLua, Result, Value};
use mlua::{Lua, Result as LuaResult};

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Position {
    #[serde(alias = "_x", alias = "X")]
    pub x: f32,
    #[serde(alias = "_y", alias = "Y")]
    pub y: f32,
    #[serde(alias = "_z", alias = "Z")]
    pub z: f32,
}

impl Position {
    pub fn distance(&self, other: &Position) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn distance_squared(&self, other: &Position) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

impl From<(f32, f32, f32)> for Position {
    fn from(v: (f32, f32, f32)) -> Self {
        Position { x: v.0, y: v.1, z: v.2 }
    }
}



pub fn inject_enum<E: LuaEnum>(lua: &Lua, table_name: &str) -> LuaResult<()> {
    let globals = lua.globals();
    let tbl = lua.create_table()?;

    for (name, value) in E::variants() {
        tbl.set(*name, *value)?;
    }

    globals.set(table_name, tbl)?;
    Ok(())
}

lua_enum! {
    pub enum EventType {
        alias("EVENT_NONE")
        EventNone = 0,

        alias("EVENT_ANY_MONSTER_DIE")
        EventAnyMonsterDie = 1,

        alias("EVENT_ANY_GADGET_DIE")
        EventAnyGadgetDie = 2,

        alias("EVENT_VARIABLE_CHANGE")
        EventVariableChange = 3,

        alias("EVENT_ENTER_REGION")
        EventEnterRegion = 4,

        alias("EVENT_LEAVE_REGION")
        EventLeaveRegion = 5,

        alias("EVENT_GADGET_CREATE")
        EventGadgetCreate = 6,

        alias("EVENT_GADGET_STATE_CHANGE")
        EventGadgetStateChange = 7,

        alias("EVENT_DUNGEON_SETTLE")
        EventDungeonSettle = 8,

        alias("EVENT_SELECT_OPTION")
        EventSelectOption = 9,

        alias("EVENT_CLIENT_EXECUTE")
        EventClientExecute = 10,

        alias("EVENT_ANY_MONSTER_LIVE")
        EventAnyMonsterLive = 11,

        alias("EVENT_SPECIFIC_MONSTER_HP_CHANGE")
        EventSpecificMonsterHpChange = 12,

        alias("EVENT_CITY_LEVELUP_UNLOCK_DUNGEON_ENTRY")
        EventCityLevelupUnlockDungeonEntry = 13,

        alias("EVENT_DUNGEON_BROADCAST_ONTIMER")
        EventDungeonBroadcastOntimer = 14,

        alias("EVENT_TIMER_EVENT")
        EventTimerEvent = 15,

        alias("EVENT_CHALLENGE_SUCCESS")
        EventChallengeSuccess = 16,

        alias("EVENT_CHALLENGE_FAIL")
        EventChallengeFail = 17,

        alias("EVENT_SEAL_BATTLE_BEGIN")
        EventSealBattleBegin = 18,

        alias("EVENT_SEAL_BATTLE_END")
        EventSealBattleEnd = 19,

        alias("EVENT_GATHER")
        EventGather = 20,

        alias("EVENT_QUEST_FINISH")
        EventQuestFinish = 21,

        alias("EVENT_MONSTER_BATTLE")
        EventMonsterBattle = 22,

        alias("EVENT_CITY_LEVELUP")
        EventCityLevelup = 23,

        alias("EVENT_CUTSCENE_END")
        EventCutsceneEnd = 24,

        alias("EVENT_AVATAR_NEAR_PLATFORM")
        EventAvatarNearPlatform = 25,

        alias("EVENT_PLATFORM_REACH_POINT")
        EventPlatformReachPoint = 26,

        alias("EVENT_UNLOCK_TRANS_POINT")
        EventUnlockTransPoint = 27,

        alias("EVENT_QUEST_START")
        EventQuestStart = 28,

        alias("EVENT_GROUP_LOAD")
        EventGroupLoad = 29,

        alias("EVENT_GROUP_WILL_UNLOAD")
        EventGroupWillUnload = 30,

        alias("EVENT_GROUP_WILL_REFRESH")
        EventGroupWillRefresh = 31,

        alias("EVENT_GROUP_REFRESH")
        EventGroupRefresh = 32,

        alias("EVENT_DUNGEON_REWARD_GET")
        EventDungeonRewardGet = 33,

        alias("EVENT_SPECIFIC_GADGET_HP_CHANGE")
        EventSpecificGadgetHpChange = 34,

        alias("EVENT_MONSTER_TIDE_OVER")
        EventMonsterTideOver = 35,

        alias("EVENT_MONSTER_TIDE_CREATE")
        EventMonsterTideCreate = 36,

        alias("EVENT_MONSTER_TIDE_DIE")
        EventMonsterTideDie = 37,

        alias("EVENT_SEALAMP_PHASE_CHANGE")
        EventSealampPhaseChange = 38,

        alias("EVENT_BLOSSOM_PROGRESS_FINISH")
        EventBlossomProgressFinish = 39,

        alias("EVENT_BLOSSOM_CHEST_DIE")
        EventBlossomChestDie = 40,

        alias("EVENT_GADGET_PLAY_START")
        EventGadgetPlayStart = 41,

        alias("EVENT_GADGET_PLAY_START_CD")
        EventGadgetPlayStartCd = 42,

        alias("EVENT_GADGET_PLAY_STOP")
        EventGadgetPlayStop = 43,

        alias("EVENT_GADGET_LUA_NOTIFY")
        EventGadgetLuaNotify = 44,

        alias("EVENT_MP_PLAY_PREPARE")
        EventMpPlayPrepare = 45,

        alias("EVENT_MP_PLAY_BATTLE")
        EventMpPlayBattle = 46,

        alias("EVENT_MP_PLAY_PREPARE_INTERRUPT")
        EventMpPlayPrepareInterrupt = 47,

        alias("EVENT_SELECT_DIFFICULTY")
        EventSelectDifficulty = 48,

        alias("EVENT_SCENE_MP_PLAY_BATTLE_STATE")
        EventSceneMpPlayBattleState = 49,

        alias("EVENT_SCENE_MP_PLAY_BATTLE_STAGE_CHANGE")
        EventSceneMpPlayBattleStageChange = 50,

        alias("EVENT_SCENE_MP_PLAY_BATTLE_RESULT")
        EventSceneMpPlayBattleResult = 51,

        alias("EVENT_SEAL_BATTLE_PROGRESS_DECREASE")
        EventSealBattleProgressDecrease = 52,

        alias("EVENT_GENERAL_REWARD_DIE")
        EventGeneralRewardDie = 53,

        alias("EVENT_SCENE_MP_PLAY_BATTLE_INTERRUPT")
        EventSceneMpPlayBattleInterrupt = 54,

        alias("EVENT_MONSTER_DIE_BEFORE_LEAVE_SCENE")
        EventMonsterDieBeforeLeaveScene = 55,

        alias("EVENT_SCENE_MP_PLAY_OPEN")
        EventSceneMpPlayOpen = 56,

        alias("EVENT_OFFERING_LEVELUP")
        EventOfferingLevelup = 57,

        alias("EVENT_DUNGEON_REVIVE")
        EventDungeonRevive = 58,

        alias("EVENT_SCENE_MP_PLAY_ALL_AVATAR_DIE")
        EventSceneMpPlayAllAvatarDie = 59,

        alias("EVENT_DUNGEON_ALL_AVATAR_DIE")
        EventDungeonAllAvatarDie = 60,

        alias("EVENT_GENERAL_REWARD_TAKEN")
        EventGeneralRewardTaken = 61,

        alias("EVENT_PLATFORM_REACH_ARRAYPOINT")
        EventPlatformReachArraypoint = 62,

        alias("EVENT_SCENE_MULTISTAGE_PLAY_STAGE_END")
        EventSceneMultistagePlayStageEnd = 63,

        alias("EVENT_SCENE_MULTISTAGE_PLAY_END_STAGE_REQ")
        EventSceneMultistagePlayEndStageReq = 64,

        alias("EVENT_MECHANICUS_PICKED_CARD")
        EventMechanicusPickedCard = 65,

        alias("EVENT_POOL_MONSTER_TIDE_OVER")
        EventPoolMonsterTideOver = 66,

        alias("EVENT_POOL_MONSTER_TIDE_CREATE")
        EventPoolMonsterTideCreate = 67,

        alias("EVENT_POOL_MONSTER_TIDE_DIE")
        EventPoolMonsterTideDie = 68,

        alias("EVENT_DUNGEON_AVATAR_SLIP_DIE")
        EventDungeonAvatarSlipDie = 69,

        alias("EVENT_GALLERY_START")
        EventGalleryStart = 70,

        alias("EVENT_GALLERY_STOP")
        EventGalleryStop = 71,

        alias("EVENT_TIME_AXIS_PASS")
        EventTimeAxisPass = 72,

        alias("EVENT_FLEUR_FAIR_DUNGEON_ALL_PLAYER_ENTER")
        EventFleurFairDungeonAllPlayerEnter = 73,

        alias("EVENT_GADGETTALK_DONE")
        EventGadgettalkDone = 74,

        alias("EVENT_SET_GAME_TIME")
        EventSetGameTime = 75,

        alias("EVENT_HIDE_AND_SEEK_PLAYER_QUIT")
        EventHideAndSeekPlayerQuit = 76,

        alias("EVENT_AVATAR_DIE")
        EventAvatarDie = 77,

        alias("EVENT_SCENE_MULTISTAGE_PLAY_STAGE_START")
        EventSceneMultistagePlayStageStart = 78,

        alias("EVENT_GALLERY_PROGRESS_PASS")
        EventGalleryProgressPass = 79,

        alias("EVENT_GALLERY_PROGRESS_EMPTY")
        EventGalleryProgressEmpty = 80,

        alias("EVENT_GALLERY_PROGRESS_FULL")
        EventGalleryProgressFull = 81,

        alias("EVENT_HUNTING_FINISH_FINAL")
        EventHuntingFinishFinal = 82,

        alias("EVENT_USE_WIDGET_TOY_FOX_CAMERA")
        EventUseWidgetToyFoxCamera = 83,

        alias("EVENT_LUNA_RITE_SACRIFICE")
        EventLunaRiteSacrifice = 84,

        alias("EVENT_SUMO_SWITCH_TEAM_EVENT")
        EventSumoSwitchTeamEvent = 85,

        alias("EVENT_FISHING_START")
        EventFishingStart = 86,

        alias("EVENT_FISHING_STOP")
        EventFishingStop = 87,

        alias("EVENT_FISHING_QTE_FINISH")
        EventFishingQteFinish = 88,

        alias("EVENT_FISHING_TIMEOUT_FLEE")
        EventFishingTimeoutFlee = 89,

        alias("EVENT_ROGUE_CELL_STATE_CHANGE")
        EventRogueCellStateChange = 90,

        alias("EVENT_ROGUE_CELL_CONSTRUCT")
        EventRogueCellConstruct = 91,

        alias("EVENT_ROGUE_CELL_FINISH_SELECT_CARD")
        EventRogueCellFinishSelectCard = 92,

        alias("EVENT_ANY_MONSTER_CAPTURE")
        EventAnyMonsterCapture = 93,

        alias("EVENT_ACTIVITY_INTERACT_GADGET")
        EventActivityInteractGadget = 94,

        alias("EVENT_CHALLENGE_PAUSE")
        EventChallengePause = 95,

        alias("EVENT_LEVEL_TAG_CHANGE")
        EventLevelTagChange = 96,

        alias("EVENT_CUSTOM_DUNGEON_START")
        EventCustomDungeonStart = 97,

        alias("EVENT_CUSTOM_DUNGEON_RESTART")
        EventCustomDungeonRestart = 98,

        alias("EVENT_CUSTOM_DUNGEON_REACTIVE")
        EventCustomDungeonReactive = 99,

        alias("EVENT_CUSTOM_DUNGEON_OUT_STUCK")
        EventCustomDungeonOutStuck = 100,

        alias("EVENT_CUSTOM_DUNGEON_EXIT_TRY")
        EventCustomDungeonExitTry = 101,

        alias("EVENT_CUSTOM_DUNGEON_OFFICIAL_RESTART")
        EventCustomDungeonOfficialRestart = 102,

        alias("EVENT_ANY_MONSTER_CAPTURE_AND_DISAPPEAR")
        EventAnyMonsterCaptureAndDisappear = 103,

        alias("EVENT_MICHIAE_INTERACT")
        EventMichiaeInteract = 104,

        alias("EVENT_SELECT_UIINTERACT")
        EventSelectUiinteract = 105,

        alias("EVENT_LUA_NOTIFY")
        EventLuaNotify = 106,

        alias("EVENT_PHOTO_FINISH")
        EventPhotoFinish = 107,

        alias("EVENT_IRODORI_MASTER_READY")
        EventIrodoriMasterReady = 108,

        alias("EVENT_ROGUE_START_FIGHT")
        EventRogueStartFight = 109,

        alias("EVENT_ROGUE_CREAGE_FIGHT_GADGET")
        EventRogueCreageFightGadget = 110,

        alias("EVENT_ROGUE_CREAGE_REPAIR_GADGET")
        EventRogueCreageRepairGadget = 111,

        alias("EVENT_ROGUE_OPEN_ACCESS")
        EventRogueOpenAccess = 112,

        alias("EVENT_GADGET_GIVING_FINISHED")
        EventGadgetGivingFinished = 113,

        alias("EVENT_OBSERVATION_POINT_NOTIFY")
        EventObservationPointNotify = 114,

        alias("EVENT_GADGET_GIVING_TAKEBACK")
        EventGadgetGivingTakeback = 115,

        alias("EVENT_ECHO_SHELL_INTERACT")
        EventEchoShellInteract = 116,

        alias("EVENT_PLATFORM_ARRIVAL")
        EventPlatformArrival = 2701,

        alias("EVENT_PLAYER_BACK_GALLERY_REVIVE_POINT")
        EventPlayerBackGalleryRevivePoint = 2800,

        alias("EVENT_GALLERY_CANNOT_START_AFTER_COUNTDOWN")
        EventGalleryCannotStartAfterCountdown = 2801,
    }
}

lua_enum! {
    pub enum GadgetState {
        alias("Default")
        Default = 0,
        alias("GatherDrop")
        GatherDrop = 1,

        alias("ChestLocked")
        ChestLocked = 101,
        alias("ChestOpened")
        ChestOpened = 102,
        alias("ChestTrap")
        ChestTrap = 103,
        alias("ChestBramble")
        ChestBramble = 104,
        alias("ChestFrozen")
        ChestFrozen = 105,
        alias("ChestRock")
        ChestRock = 106,

        alias("GearStart")
        GearStart = 201,
        alias("GearStop")
        GearStop = 202,
        alias("GearAction1")
        GearAction1 = 203,
        alias("GearAction2")
        GearAction2 = 204,

        alias("CrystalResonate1")
        CrystalResonate1 = 301,
        alias("CrystalResonate2")
        CrystalResonate2 = 302,
        alias("CrystalExplode")
        CrystalExplode = 303,
        alias("CrystalDrain")
        CrystalDrain = 304,

        alias("StatueActive")
        StatueActive = 401,

        alias("Action01")
        Action01 = 901,
        alias("Action02")
        Action02 = 902,
        alias("Action03")
        Action03 = 903,
    }
}

lua_enum! {
    pub enum RegionShape {
        alias("NONE")
        None = 0,
        alias("SPHERE")
        Sphere = 1,
        alias("CUBIC")
        Cubic = 2,
        alias("CYLINDER")
        Cylinder = 3,
        alias("POLYGON")
        Polygon = 4,
    }
}

lua_enum! {
    pub enum GroupKillPolicy {
        alias("GROUP_KILL_NONE")
        GroupKillNone = 0,
        alias("GROUP_KILL_ALL")
        GroupKillAll = 1,
        alias("GROUP_KILL_MONSTER")
        GroupKillMonster = 2,
        alias("GROUP_KILL_GADGET")
        GroupKillGadget = 3,
        alias("GROUP_KILL_NPC")
        GroupKillNpc = 4,
    }
}

lua_enum! {
    pub enum SealBattleType {
        alias("NONE")
        None = 0,
        alias("ENERGY_CHARGE")
        EnergyCharge = 1,
        alias("KILL_MONSTER")
        KillMonster = 2,
    }
}

lua_enum! {
    pub enum FatherChallengeProperty {
        alias("DURATION")
        Duration= 0,
        alias("CUR_SUCC")
        CurSucc= 1,
        alias("CUR_FAIL")
        CurFail= 2,
        alias("SUM_SUCC")
        SumSucc= 3,
        alias("SUM_FAIL")
        SumFail= 4,
    }
}

lua_enum! {
    pub enum ChallengeEventMarkType {
        alias("CHALLENGE_EVENT_NONE")
        ChallengeEventNone= 0,

        alias("FLIGHT_TIME")
        FlightTime= 1,

        alias("FLIGHT_GATHER_POINT")
        FlightGatherPoint= 2,

        alias("SUMMER_TIME_SPRINT_BOAT_TIME")
        SummerTimeSprintBoatTime= 3,

        alias("SUMMER_TIME_SPRINT_BOAT_GATHER_POINT")
        SummerTimeSprintBoatGatherPoint= 4,
    }
}

#[derive(Copy, Clone)]
pub struct LuaContext {
    pub scene_id: u32,
    pub group_id: u32,
    pub uid: u32,
}

#[derive(Copy, Clone)]
pub struct LuaEvt {
    pub param1: u32,
    pub param2: u32,
    pub source_eid: u32,
}

impl IntoLua for LuaContext {
    fn into_lua(self, lua: &Lua) -> Result<Value> {
        let tbl = lua.create_table()?;
        tbl.set("scene_id", self.scene_id)?;
        tbl.set("group_id", self.group_id)?;
        Ok(Value::Table(tbl))
    }
}

impl IntoLua for LuaEvt {
    fn into_lua(self, lua: &Lua) -> Result<Value> {
        let tbl = lua.create_table()?;
        tbl.set("param1", self.param1)?;
        tbl.set("param2", self.param2)?;
        tbl.set("source_eid", self.source_eid)?;
        Ok(Value::Table(tbl))
    }
}
