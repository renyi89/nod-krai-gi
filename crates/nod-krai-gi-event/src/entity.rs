use bevy_ecs::message::Message;

#[derive(Message)]
pub struct GadgetInteractEvent(pub u32, pub u32, pub u32);

#[derive(Message)]
pub struct EvtCreateGadgetEvent(pub u32, pub u32, pub u32);

#[derive(Message)]
pub struct EvtDestroyGadgetEvent(pub u32);
