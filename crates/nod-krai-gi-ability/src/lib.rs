use crate::server_invoke::server_invoke;

use crate::handler::{
    handle_add_new_ability, handle_clear_global_float_value, handle_global_float_value,
    handle_modifier_change, handle_override_param, handle_reinit_override_map,
};

use crate::actions::ability_action_add_hp_debts::ability_action_add_hp_debts_event;
use crate::actions::ability_action_get_hp_paid_debts::ability_action_get_hp_paid_debts_event;
use crate::actions::ability_action_heal_hp::ability_action_heal_hp_event;
use crate::actions::ability_action_lose_hp::ability_action_lose_hp_event;
use crate::actions::ability_action_reduce_hp_debts::ability_action_reduce_hp_debts_event;
use crate::actions::ability_action_set_global_value_to_override_map::ability_action_set_global_value_to_override_map_event;
use crate::actions::ability_action_set_override_map_value::ability_action_set_override_map_value_event;
use crate::actions::ability_action_set_random_override_map_value::ability_action_set_random_override_map_value_event;
use crate::actions::execute_action_system;
use crate::mixins::execute_mixin_system;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use nod_krai_gi_entity::client_gadget::EntitySystemSet;
use nod_krai_gi_event::ability::*;
use nod_krai_gi_message::event::ClientMessageEvent;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_proto::normal::{
    AbilityInvocationsNotify, AbilityInvokeArgument, AbilityInvokeEntry,
    ClientAbilitiesInitFinishCombineNotify, ClientAbilityChangeNotify,
    ClientAbilityInitFinishNotify, ForwardType,
};

mod actions;
mod enums;
mod handler;
mod mixins;
mod server_invoke;
mod util;

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, on_ability_notify).add_systems(
            Update,
            (
                (
                    handle_add_new_ability.after(EntitySystemSet::HandleEvtGadgetUpdate),
                    handle_modifier_change,
                    handle_override_param,
                    handle_reinit_override_map,
                    handle_global_float_value,
                    handle_clear_global_float_value,
                    server_invoke,
                )
                    .chain(),
                (
                    execute_action_system,
                    execute_mixin_system,
                    ability_action_set_override_map_value_event,
                    ability_action_set_random_override_map_value_event,
                    ability_action_set_global_value_to_override_map_event,
                    ability_action_heal_hp_event,
                    ability_action_lose_hp_event,
                    ability_action_get_hp_paid_debts_event,
                    ability_action_add_hp_debts_event,
                    ability_action_reduce_hp_debts_event,
                )
                    .chain(),
            )
                .chain(),
        );
    }
}

fn on_ability_notify(
    mut events: MessageReader<ClientMessageEvent>,
    message_output: Res<MessageOutput>,
    mut add_new_ability_events: MessageWriter<AddNewAbilityEvent>,
    mut modifier_events: MessageWriter<ModifierChangeEvent>,
    mut override_param_events: MessageWriter<OverrideParamEvent>,
    mut reinit_overridemap_events: MessageWriter<ReinitOverrideMapEvent>,
    mut global_float_value_events: MessageWriter<GlobalFloatValueEvent>,
    mut clear_global_float_value_events: MessageWriter<ClearGlobalFloatValueEvent>,
    mut server_invoke_events: MessageWriter<ServerInvokeEvent>,
) {
    for message in events.read() {
        match message.message_name() {
            "AbilityInvocationsNotify" => {
                if let Some(notify) = message.decode::<AbilityInvocationsNotify>() {
                    let mut invoke_list_to_all: Vec<AbilityInvokeEntry> = Vec::new();
                    let mut invoke_list_to_ohers: Vec<AbilityInvokeEntry> = Vec::new();
                    let mut invoke_list_to_host: Vec<AbilityInvokeEntry> = Vec::new();
                    for invoke in notify.invokes {
                        match invoke.forward_type() {
                            ForwardType::ForwardToAll => {
                                invoke_list_to_all.push(invoke.clone());
                            }
                            ForwardType::ForwardToAllExceptCur
                            | ForwardType::ForwardToAllExistExceptCur => {
                                invoke_list_to_ohers.push(invoke.clone());
                            }
                            ForwardType::ForwardToHost => {
                                invoke_list_to_host.push(invoke.clone());
                            }
                            _ => {}
                        }
                        on_ability_invoke(
                            invoke,
                            message.version(),
                            &mut add_new_ability_events,
                            &mut modifier_events,
                            &mut override_param_events,
                            &mut reinit_overridemap_events,
                            &mut global_float_value_events,
                            &mut clear_global_float_value_events,
                            &mut server_invoke_events,
                        );
                    }
                    if !invoke_list_to_all.is_empty() {
                        message_output.send_to_all(
                            "AbilityInvocationsNotify",
                            AbilityInvocationsNotify {
                                invokes: invoke_list_to_all,
                            },
                        );
                    }

                    if !invoke_list_to_ohers.is_empty() {
                        message_output.send_to_others(
                            message.sender_uid(),
                            "AbilityInvocationsNotify",
                            AbilityInvocationsNotify {
                                invokes: invoke_list_to_ohers,
                            },
                        );
                    }

                    if !invoke_list_to_host.is_empty() {
                        message_output.send(
                            message.sender_uid(),
                            "AbilityInvocationsNotify",
                            AbilityInvocationsNotify {
                                invokes: invoke_list_to_host,
                            },
                        );
                    }
                } else {
                    tracing::error!("AbilityInvocationsNotify forward_type not support");
                }
            }
            "ClientAbilityInitFinishNotify" => {
                if let Some(notify) = message.decode::<ClientAbilityInitFinishNotify>() {
                    let mut invoke_list_to_all: Vec<AbilityInvokeEntry> = Vec::new();
                    let mut invoke_list_to_ohers: Vec<AbilityInvokeEntry> = Vec::new();
                    let mut invoke_list_to_host: Vec<AbilityInvokeEntry> = Vec::new();
                    for invoke in notify.invokes {
                        match invoke.forward_type() {
                            ForwardType::ForwardToAll => {
                                invoke_list_to_all.push(invoke.clone());
                            }
                            ForwardType::ForwardToAllExceptCur
                            | ForwardType::ForwardToAllExistExceptCur => {
                                invoke_list_to_ohers.push(invoke.clone());
                            }
                            ForwardType::ForwardToHost => {
                                invoke_list_to_host.push(invoke.clone());
                            }
                            _ => {}
                        }
                        on_ability_invoke(
                            invoke,
                            message.version(),
                            &mut add_new_ability_events,
                            &mut modifier_events,
                            &mut override_param_events,
                            &mut reinit_overridemap_events,
                            &mut global_float_value_events,
                            &mut clear_global_float_value_events,
                            &mut server_invoke_events,
                        );
                    }
                    if !invoke_list_to_all.is_empty() {
                        message_output.send_to_all(
                            "ClientAbilityInitFinishNotify",
                            ClientAbilityInitFinishNotify {
                                entity_id: notify.entity_id,
                                invokes: invoke_list_to_all,
                            },
                        );
                    }

                    if !invoke_list_to_ohers.is_empty() {
                        message_output.send_to_others(
                            message.sender_uid(),
                            "ClientAbilityInitFinishNotify",
                            ClientAbilityInitFinishNotify {
                                entity_id: notify.entity_id,
                                invokes: invoke_list_to_ohers,
                            },
                        );
                    }

                    if !invoke_list_to_host.is_empty() {
                        message_output.send(
                            message.sender_uid(),
                            "ClientAbilityInitFinishNotify",
                            ClientAbilityInitFinishNotify {
                                entity_id: notify.entity_id,
                                invokes: invoke_list_to_host,
                            },
                        );
                    }
                } else {
                    tracing::error!("ClientAbilityInitFinishNotify forward_type not support");
                }
            }
            "ClientAbilitiesInitFinishCombineNotify" => {
                if let Some(notify) = message.decode::<ClientAbilitiesInitFinishCombineNotify>() {
                    for invoke_list in notify.entity_invoke_list {
                        for invoke in invoke_list.invokes {
                            on_ability_invoke(
                                invoke,
                                message.version(),
                                &mut add_new_ability_events,
                                &mut modifier_events,
                                &mut override_param_events,
                                &mut reinit_overridemap_events,
                                &mut global_float_value_events,
                                &mut clear_global_float_value_events,
                                &mut server_invoke_events,
                            );
                        }
                    }
                } else {
                    tracing::error!(
                        "ClientAbilitiesInitFinishCombineNotify forward_type not support"
                    );
                }
            }
            "ClientAbilityChangeNotify" => {
                if let Some(notify) = message.decode::<ClientAbilityChangeNotify>() {
                    for invoke in notify.invokes {
                        on_ability_invoke(
                            invoke,
                            message.version(),
                            &mut add_new_ability_events,
                            &mut modifier_events,
                            &mut override_param_events,
                            &mut reinit_overridemap_events,
                            &mut global_float_value_events,
                            &mut clear_global_float_value_events,
                            &mut server_invoke_events,
                        );
                    }
                } else {
                    tracing::error!("ClientAbilityChangeNotify forward_type not support");
                }
            }
            &_ => {}
        }
    }
}

pub fn on_ability_invoke(
    invoke: AbilityInvokeEntry,
    version: String,
    add_new_ability_events: &mut MessageWriter<AddNewAbilityEvent>,
    modifier_events: &mut MessageWriter<ModifierChangeEvent>,
    override_param_events: &mut MessageWriter<OverrideParamEvent>,
    reinit_overridemap_events: &mut MessageWriter<ReinitOverrideMapEvent>,
    global_float_value_events: &mut MessageWriter<GlobalFloatValueEvent>,
    clear_global_float_value_events: &mut MessageWriter<ClearGlobalFloatValueEvent>,
    server_invoke_events: &mut MessageWriter<ServerInvokeEvent>,
) {
    if let Some(head) = invoke.head {
        if head.local_id != 0 {
            server_invoke_events.write(ServerInvokeEvent(invoke.clone()));
            return;
        }
    }

    match invoke.argument_type() {
        AbilityInvokeArgument::AbilityMetaModifierChange => {
            modifier_events.write(ModifierChangeEvent(invoke.clone(), version.clone()));
        }
        AbilityInvokeArgument::AbilityMetaOverrideParam => {
            override_param_events.write(OverrideParamEvent(invoke.clone(), version.clone()));
        }
        AbilityInvokeArgument::AbilityMetaReinitOverridemap => {
            reinit_overridemap_events
                .write(ReinitOverrideMapEvent(invoke.clone(), version.clone()));
        }
        AbilityInvokeArgument::AbilityMetaGlobalFloatValue => {
            global_float_value_events.write(GlobalFloatValueEvent(invoke.clone(), version.clone()));
        }
        AbilityInvokeArgument::AbilityMetaClearGlobalFloatValue => {
            clear_global_float_value_events
                .write(ClearGlobalFloatValueEvent(invoke.clone(), version.clone()));
        }
        AbilityInvokeArgument::AbilityMetaAddNewAbility => {
            add_new_ability_events.write(AddNewAbilityEvent(invoke.clone(), version.clone()));
        }
        _ => {}
    }
}
