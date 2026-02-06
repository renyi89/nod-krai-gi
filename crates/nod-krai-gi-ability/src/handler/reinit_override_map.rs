use bevy_ecs::prelude::*;
use nod_krai_gi_entity::common::{EntityById, InstancedAbilities, ProtocolEntityID};
use nod_krai_gi_proto::AbilityMetaReInitOverrideMap;

use crate::util::get_ability_name;
use nod_krai_gi_event::ability::*;

pub fn handle_reinit_override_map(
    index: Res<EntityById>,
    mut events: MessageReader<ReinitOverrideMapEvent>,
    mut entities: Query<(&mut InstancedAbilities, &ProtocolEntityID)>,
) {
    for ReinitOverrideMapEvent(invoke, version) in events.read() {
        let entity = match index.0.get(&invoke.entity_id) {
            Some(e) => *e,
            None => {
                tracing::debug!("[ReinitOverrideMap] Entity {} not found", invoke.entity_id);
                continue;
            }
        };

        let Some(head) = invoke.head else {
            tracing::debug!("[ReinitOverrideMap] AbilityInvokeEntry head is missing");
            continue;
        };

        let instanced_ability_id = head.instanced_ability_id;

        let Ok((mut instanced_abilities, _)) = entities.get_mut(entity) else {
            tracing::debug!(
                "[ReinitOverrideMap] Failed to get entity components for {}",
                invoke.entity_id
            );
            continue;
        };

        match instanced_abilities.find_by_instanced_ability_id_mut(instanced_ability_id) {
            None => {
                tracing::debug!(
                    "[ReinitOverrideMap] Invalid instanced_ability_id: {} for entity {}",
                    instanced_ability_id,
                    invoke.entity_id
                );
                continue;
            }
            Some((_index, instanced_ability)) => {
                match nod_krai_gi_proto::dy_parser::decode_from_vec_by_name_version::<
                    AbilityMetaReInitOverrideMap,
                >(
                    version,
                    "AbilityMetaReInitOverrideMap",
                    &invoke.ability_data,
                ) {
                    None => {
                        tracing::debug!(
                            "[ReinitOverrideMap] Failed to decode AbilityMetaReInitOverrideMap"
                        );
                    }
                    Some(reinit_data) => {
                        for entry in reinit_data.override_map {
                            match get_ability_name(entry.key) {
                                None => {
                                    tracing::debug!(
                                        "[ReinitOverrideMap] No key provided for override param"
                                    );
                                    continue;
                                }
                                Some(key) => {
                                    let value = entry.float_value;

                                    match instanced_ability.ability_data {
                                        None => {
                                            tracing::debug!("[ReinitOverrideMap] Reinit ability_specials {} = {} None ability on entity {}",
                                                key,
                                                value,
                                                invoke.entity_id
                                            );
                                        }
                                        Some(ability_data) => {
                                            tracing::debug!("[ReinitOverrideMap] Reinit ability_specials {} = {} for ability {} on entity {}",
                                                key,
                                                value,
                                                ability_data.ability_name,
                                                invoke.entity_id
                                            );
                                        }
                                    }

                                    instanced_ability.ability_specials.insert(key, value);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
