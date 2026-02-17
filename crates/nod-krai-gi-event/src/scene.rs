use bevy_ecs::message::Message;
use nod_krai_gi_proto::normal::EnterType;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnterReason {
    None = 0,
    Login = 1,
    DungeonReplay = 11,
    DungeonReviveOnWaypoint = 12,
    DungeonEnter = 13,
    DungeonQuit = 14,
    Gm = 21,
    QuestRollback = 31,
    Revival = 32,
    PersonalScene = 41,
    TransPoint = 42,
    ClientTransmit = 43,
    ForceDragBack = 44,
    TeamKick = 51,
    TeamJoin = 52,
    TeamBack = 53,
    Muip = 54,
    DungeonInviteAccept = 55,
    Lua = 56,
    ActivityLoadTerrain = 57,
    HostFromSingleToMp = 58,
    MpPlay = 59,
    AnchorPoint = 60,
    LuaSkipUi = 61,
    ReloadTerrain = 62,
    DraftTransfer = 63,
    EnterHome = 64,
    ExitHome = 65,
    ChangeHomeModule = 66,
    Gallery = 67,
    HomeSceneJump = 68,
    HideAndSeek = 69,
}

impl From<u32> for EnterReason {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Login,
            11 => Self::DungeonReplay,
            12 => Self::DungeonReviveOnWaypoint,
            13 => Self::DungeonEnter,
            14 => Self::DungeonQuit,
            21 => Self::Gm,
            31 => Self::QuestRollback,
            32 => Self::Revival,
            41 => Self::PersonalScene,
            42 => Self::TransPoint,
            43 => Self::ClientTransmit,
            44 => Self::ForceDragBack,
            51 => Self::TeamKick,
            52 => Self::TeamJoin,
            53 => Self::TeamBack,
            54 => Self::Muip,
            55 => Self::DungeonInviteAccept,
            56 => Self::Lua,
            57 => Self::ActivityLoadTerrain,
            58 => Self::HostFromSingleToMp,
            59 => Self::MpPlay,
            60 => Self::AnchorPoint,
            61 => Self::LuaSkipUi,
            62 => Self::ReloadTerrain,
            63 => Self::DraftTransfer,
            64 => Self::EnterHome,
            65 => Self::ExitHome,
            66 => Self::ChangeHomeModule,
            67 => Self::Gallery,
            68 => Self::HomeSceneJump,
            69 => Self::HideAndSeek,
            _ => Self::None,
        }
    }
}

#[derive(Message)]
pub struct BeginEnterSceneEvent {
    pub uid: u32,
    pub scene_id: u32,
    pub enter_type: EnterType,
    pub enter_reason: EnterReason,
    pub position: nod_krai_gi_proto::server_only::VectorBin,
}

#[derive(Message)]
pub struct EnterSceneReadyEvent(pub u32);

#[derive(Message)]
pub struct SceneInitFinishEvent(pub u32);

#[derive(Message)]
pub struct EnterSceneDoneEvent(pub u32);

#[derive(Message)]
pub struct PostEnterSceneEvent(pub u32);

#[derive(Message)]
pub struct PlayerJoinTeamEvent {
    pub player_uid: u32,
    pub avatar_guid_list: Vec<u64>,
    pub appear_avatar_guid: u64,
}

#[derive(Message)]
pub struct SceneTeamUpdateEvent;

#[derive(Message, Debug)]
pub struct PlayerAvatarTeamChanged {
    pub uid: u32,
    pub avatar_team_guid_list: Vec<u64>,
    pub cur_avatar_guid: u64,
}

#[derive(Message)]
pub struct ScenePlayerJumpEvent(pub u32, pub u32, pub EnterReason, pub (f32, f32, f32));

#[derive(Message)]
pub struct ScenePlayerJumpByPointEvent(pub u32, pub u32, pub u32);

#[derive(Message)]
pub struct ScenePlayerEnterDungeonEvent(pub u32, pub u32);
