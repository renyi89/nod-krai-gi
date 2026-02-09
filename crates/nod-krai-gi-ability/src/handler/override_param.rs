use bevy_ecs::prelude::*;
use nod_krai_gi_data::GAME_SERVER_CONFIG;
use nod_krai_gi_entity::common::{EntityById, InstancedAbilities, ProtocolEntityID};
use nod_krai_gi_proto::normal::AbilityScalarValueEntry;

use crate::util::get_ability_name;
use nod_krai_gi_event::ability::*;

pub fn handle_override_param(
    index: Res<EntityById>,
    mut events: MessageReader<OverrideParamEvent>,
    mut entities: Query<(&mut InstancedAbilities, &ProtocolEntityID)>,
) {
    for OverrideParamEvent(invoke, version) in events.read() {
        let entity = match index.0.get(&invoke.entity_id) {
            Some(e) => *e,
            None => {
                if GAME_SERVER_CONFIG.plugin.ability_log {
                    tracing::debug!("[OverrideParam] Entity {} not found", invoke.entity_id);
                }
                continue;
            }
        };

        let Some(head) = invoke.head else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!("[OverrideParam] AbilityInvokeEntry head is missing");
            }
            continue;
        };

        let instanced_ability_id = head.instanced_ability_id;

        let Ok((mut instanced_abilities, _)) = entities.get_mut(entity) else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[OverrideParam] Failed to get entity components for {}",
                    invoke.entity_id
                );
            }
            continue;
        };

        match instanced_abilities.find_by_instanced_ability_id_mut(instanced_ability_id) {
            None => {
                if GAME_SERVER_CONFIG.plugin.ability_log {
                    tracing::debug!(
                        "[OverrideParam] Invalid instanced_ability_id: {} for entity {}",
                        instanced_ability_id,
                        invoke.entity_id
                    );
                }
                continue;
            }
            Some((_index, instanced_ability)) => {
                match nod_krai_gi_proto::dy_parser::decode_from_vec_by_name_version::<
                    AbilityScalarValueEntry,
                >(version, "AbilityScalarValueEntry", &invoke.ability_data)
                {
                    None => {
                        if GAME_SERVER_CONFIG.plugin.ability_log {
                            tracing::debug!(
                                "[OverrideParam] Failed to decode AbilityScalarValueEntry"
                            );
                        }
                    }
                    Some(entry) => match get_ability_name(entry.key) {
                        None => {
                            if GAME_SERVER_CONFIG.plugin.ability_log {
                                tracing::debug!(
                                    "[OverrideParam] No key provided for override param"
                                );
                            }
                            continue;
                        }
                        Some(key) => {
                            let value = entry.float_value;

                            match instanced_ability.ability_data {
                                None => {
                                    if GAME_SERVER_CONFIG.plugin.ability_log {
                                        tracing::debug!(
                                            "[OverrideParam] Setting ability_specials {} = {} None ability on entity {}",
                                            key,
                                            value,
                                            invoke.entity_id
                                        );
                                    }
                                }
                                Some(ability_data) => {
                                    if GAME_SERVER_CONFIG.plugin.ability_log {
                                        tracing::debug!(
                                            "[OverrideParam] Setting ability_specials {} = {} for ability {} on entity {}",
                                            key,
                                            value,
                                            ability_data.ability_name,
                                            invoke.entity_id
                                        );
                                    }
                                }
                            }

                            instanced_ability.ability_specials.insert(key, value);
                        }
                    },
                }
            }
        }
    }
}
