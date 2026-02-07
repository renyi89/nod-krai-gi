use bevy_ecs::message::Message;
use common::gm_util::QuestAction;

#[derive(Message)]
pub struct DebugCommandEvent {
    pub executor_uid: u32,
    pub kind: CommandKind,
}

#[derive(Debug)]
pub enum CommandKind {
    QuickSpawnMonster {
        monster_id: Option<u32>,
        position: (f32, f32),
    },
    QuickSpawnGadget {
        gadget_id: Option<u32>,
        position: (f32, f32),
    },
    QuickTravel {
        scene_id: Option<u32>,
        position: (f32, Option<f32>, f32),
    },
}

#[derive(Message)]
pub struct ConsoleChatReqEvent(pub u32, pub String);

#[derive(Message)]
pub struct ConsoleChatNotifyEvent(pub u32, pub String);

#[derive(Message)]
pub struct CommandQuestEvent(pub u32, pub QuestAction);