use crate::ability::{on_ability_invoke, AbilityInvokeEvent};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use nod_krai_gi_message::event::ClientMessageEvent;
use nod_krai_gi_proto::{
    AbilityInvocationsNotify, ClientAbilitiesInitFinishCombineNotify, ClientAbilityChangeNotify,
    ClientAbilityInitFinishNotify,
};

mod ability;

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<AbilityInvokeEvent>()
            .add_systems(PreUpdate, on_ability_notify)
            .add_systems(Update, on_ability_invoke);
    }
}

fn on_ability_notify(
    mut events: MessageReader<ClientMessageEvent>,
    mut invoke_events: MessageWriter<AbilityInvokeEvent>,
) {
    for message in events.read() {
        match message.message_name() {
            "AbilityInvocationsNotify" => {
                if let Some(notify) = message.decode::<AbilityInvocationsNotify>() {
                    for invoke in notify.invokes {
                        invoke_events.write(AbilityInvokeEvent(invoke));
                    }
                }
            }
            "ClientAbilityInitFinishNotify" => {
                if let Some(notify) = message.decode::<ClientAbilityInitFinishNotify>() {
                    for invoke in notify.invokes {
                        invoke_events.write(AbilityInvokeEvent(invoke));
                    }
                }
            }
            "ClientAbilitiesInitFinishCombineNotify" => {
                if let Some(notify) = message.decode::<ClientAbilitiesInitFinishCombineNotify>() {
                    for invoke_list in notify.entity_invoke_list {
                        for invoke in invoke_list.invokes {
                            invoke_events.write(AbilityInvokeEvent(invoke));
                        }
                    }
                }
            }
            "ClientAbilityChangeNotify" => {
                if let Some(notify) = message.decode::<ClientAbilityChangeNotify>() {
                    for invoke in notify.invokes {
                        invoke_events.write(AbilityInvokeEvent(invoke));
                    }
                }
            }
            &_ => {}
        }
    }
}
