use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use hit::deal_damage_on_hit;
use movement::{entity_movement, track_player_position};
use nod_krai_gi_message::event::*;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_proto::normal::{
    CombatInvocationsNotify, CombatInvokeEntry, EntityMoveInfo, EvtAnimatorParameterInfo,
    EvtBeingHitInfo, ForwardType,
};
use tracing::{error, instrument};

mod hit;
mod movement;

use nod_krai_gi_event::combat::{EntityBeingHitEvent, EntityMoveEvent, PlayerMoveEvent};
use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

pub fn next_seq() -> u32 {
    COUNTER.fetch_add(1, Ordering::Relaxed) + 1
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, combat_invocation_processor)
            .add_systems(Update, entity_movement)
            .add_systems(Update, deal_damage_on_hit)
            .add_systems(PostUpdate, track_player_position)
            .add_message::<PlayerMoveEvent>();
    }
}

#[instrument(skip_all)]
fn combat_invocation_processor(
    mut events: MessageReader<ClientMessageEvent>,
    message_output: Res<MessageOutput>,
    mut movement_events: MessageWriter<EntityMoveEvent>,
    mut hit_events: MessageWriter<EntityBeingHitEvent>,
) {
    for message in events.read() {
        match message.message_name() {
            "CombatInvocationsNotify" => {
                if let Some(notify) = message.decode::<CombatInvocationsNotify>() {
                    let mut invoke_list_to_all: Vec<CombatInvokeEntry> = Vec::new();
                    let mut invoke_list_to_ohers: Vec<CombatInvokeEntry> = Vec::new();
                    let mut invoke_list_to_host: Vec<CombatInvokeEntry> = Vec::new();
                    for mut invoke in notify.invoke_list {
                        use nod_krai_gi_proto::normal::CombatTypeArgument::*;

                        match invoke.argument_type() {
                            EntityMove => {
                                match nod_krai_gi_proto::dy_parser::decode_from_vec_by_name_version::<
                                    EntityMoveInfo,
                                >(
                                    message.version().as_str(),
                                    "EntityMoveInfo",
                                    invoke.combat_data.as_ref(),
                                ) {
                                    None => {}
                                    Some(info) => {
                                        movement_events
                                            .write(EntityMoveEvent(message.sender_uid(), info));
                                    }
                                };
                            }
                            CombatEvtBeingHit => {
                                match nod_krai_gi_proto::dy_parser::decode_from_vec_by_name_version::<
                                    EvtBeingHitInfo,
                                >(
                                    message.version().as_str(),
                                    "EvtBeingHitInfo",
                                    invoke.combat_data.as_ref(),
                                ) {
                                    None => {}
                                    Some(info) => match info.attack_result {
                                        None => {}
                                        Some(attack_result) => {
                                            hit_events.write(EntityBeingHitEvent(
                                                message.sender_uid(),
                                                attack_result,
                                            ));
                                        }
                                    },
                                };
                            }
                            CombatAnimatorParameterChanged => {
                                match nod_krai_gi_proto::dy_parser::decode_from_vec_by_name_version::<
                                    EvtAnimatorParameterInfo,
                                >(
                                    message.version().as_str(),
                                    "EvtAnimatorParameterInfo",
                                    invoke.combat_data.as_ref(),
                                ) {
                                    None => {}
                                    Some(mut info) => {
                                        if info.is_server_cache {
                                            info.is_server_cache = false;
                                            match nod_krai_gi_proto::dy_parser::encode_to_vec_by_name_version::<
                                                EvtAnimatorParameterInfo,
                                            >(
                                                message.version().as_str(), "EvtAnimatorParameterInfo", &info
                                            ) {
                                                None => {
                                                    error!(
                                                "version:{} message_name:{} error",
                                                message.version().as_str(), "EvtAnimatorParameterInfo"
                                            );
                                                }
                                                Some(body) => {
                                                    invoke.combat_data = body;
                                                }
                                            }
                                        }
                                    }
                                };
                            }
                            _ => {}
                        }

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
                    }

                    if !invoke_list_to_all.is_empty() {
                        message_output.send_to_all(
                            "CombatInvocationsNotify",
                            CombatInvocationsNotify {
                                invoke_list: invoke_list_to_all,
                                combat_unk_seq: next_seq(),
                            },
                        );
                    }

                    if !invoke_list_to_ohers.is_empty() {
                        message_output.send_to_others(
                            message.sender_uid(),
                            "CombatInvocationsNotify",
                            CombatInvocationsNotify {
                                invoke_list: invoke_list_to_ohers,
                                combat_unk_seq: next_seq(),
                            },
                        );
                    }

                    if !invoke_list_to_host.is_empty() {
                        message_output.send(
                            message.sender_uid(),
                            "CombatInvocationsNotify",
                            CombatInvocationsNotify {
                                invoke_list: invoke_list_to_host,
                                combat_unk_seq: next_seq(),
                            },
                        );
                    }
                }
            }
            &_ => {}
        }
    }
}
