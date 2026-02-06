use bevy_ecs::message::Message;
use nod_krai_gi_entity::transform::Vector3;
use nod_krai_gi_proto::EnterType;

#[derive(Message)]
pub struct BeginEnterSceneEvent {
    pub uid: u32,
    pub scene_id: u32,
    pub enter_type: EnterType,
    pub position: Vector3,
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
pub struct ScenePlayerJumpEvent(pub u32, pub u32, pub Vector3);

#[derive(Message)]
pub struct ScenePlayerJumpByPointEvent(pub u32, pub u32, pub u32);
