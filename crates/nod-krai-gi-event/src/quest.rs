use bevy_ecs::message::Message;

#[derive(Message)]
pub struct QuestAcceptEvent(pub u32, pub u32);

#[derive(Message)]
pub struct QuestFinishEvent(pub u32, pub u32);

#[derive(Message)]
pub struct QuestFailEvent(pub u32, pub u32);

#[derive(Message)]
pub struct QuestContentProgressEvent {
    pub player_uid: u32,
    pub content_type: nod_krai_gi_data::quest::quest_config::QuestContent,
    pub param: u32,
    pub param2: u32,
    pub param3: u32,
    pub add_progress: u32,
}

#[derive(Message)]
pub struct QuestAcceptCondEvent {
    pub player_uid: u32,
    pub cond_type: nod_krai_gi_data::quest::quest_config::QuestCond,
    pub param: u32,
}

#[derive(Message)]
pub struct QuestExecEvent {
    pub player_uid: u32,
    pub quest_id: u32,
    pub parent_quest_id: u32,
    pub exec_type: u32,
}
