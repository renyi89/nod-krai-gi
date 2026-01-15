use bevy_ecs::prelude::*;
use nod_krai_gi_entity::common::{EntityById, InstancedAbilities, ProtocolEntityID};
use nod_krai_gi_proto::AbilityScalarValueEntry;

use crate::OverrideParamEvent;
use crate::util::get_ability_name;

pub fn handle_override_param(
    index: Res<EntityById>,
    mut events: MessageReader<OverrideParamEvent>,
    mut entities: Query<(&mut InstancedAbilities, &ProtocolEntityID)>,
) {
    for OverrideParamEvent(invoke, version) in events.read() {
        let entity = match index.0.get(&invoke.entity_id) {
            Some(e) => *e,
            None => {
                tracing::debug!("[OverrideParam] Entity {} not found", invoke.entity_id);
                continue;
            }
        };

        let Some(head) = invoke.head else {
            tracing::debug!("[OverrideParam] AbilityInvokeEntry head is missing");
            continue;
        };

        let instanced_ability_id = head.instanced_ability_id;

        let Ok((mut instanced_abilities, _)) = entities.get_mut(entity) else {
            tracing::debug!(
                "[OverrideParam] Failed to get entity components for {}",
                invoke.entity_id
            );
            continue;
        };

        if !instanced_abilities.0.contains_key(&instanced_ability_id) {
            tracing::debug!(
                "[OverrideParam] Invalid instanced_ability_id : {} for entity {}",
                instanced_ability_id,
                invoke.entity_id
            );
            continue;
        }

        let instanced_ability = instanced_abilities
            .0
            .get_mut(&instanced_ability_id)
            .unwrap();

        let ability_data = match instanced_ability.ability_data {
            Some(data) => data,
            None => {
                tracing::debug!(
                    "[OverrideParam] No ability data for instanced_ability_id : {}",
                    instanced_ability_id
                );
                continue;
            }
        };

        match nod_krai_gi_proto::dy_parser::decode_from_vec_by_name_version::<AbilityScalarValueEntry>(
            version,
            "AbilityScalarValueEntry",
            &invoke.ability_data,
        ) {
            None => {
                tracing::debug!("[OverrideParam] Failed to decode AbilityScalarValueEntry");
            }
            Some(entry) => match get_ability_name(entry.key) {
                None => {
                    tracing::debug!("[OverrideParam] No key provided for override param");
                    continue;
                }
                Some(key) => {
                    let value = entry.float_value;

                    tracing::debug!(
                        "[OverrideParam] Setting ability_specials {} = {} for ability {} on entity {}",
                        key,
                        value,
                        ability_data.ability_name,
                        invoke.entity_id
                    );

                    instanced_ability.ability_specials.insert(key, value);
                }
            },
        }
    }
}
