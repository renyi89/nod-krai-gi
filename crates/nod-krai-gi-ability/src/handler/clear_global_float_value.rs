use bevy_ecs::prelude::*;
use nod_krai_gi_entity::common::{EntityById, GlobalAbilityValues};
use nod_krai_gi_proto::AbilityScalarValueEntry;

use crate::ClearGlobalFloatValueEvent;
use crate::util::get_ability_name;

pub fn handle_clear_global_float_value(
    index: Res<EntityById>,
    mut events: MessageReader<ClearGlobalFloatValueEvent>,
    mut entities: Query<&mut GlobalAbilityValues>,
) {
    for ClearGlobalFloatValueEvent(invoke, version) in events.read() {
        let entity = match index.0.get(&invoke.entity_id) {
            Some(e) => *e,
            None => {
                tracing::debug!(
                    "[ClearGlobalFloatValue] Entity {} not found",
                    invoke.entity_id
                );
                continue;
            }
        };

        let Ok(mut global_ability_values) = entities.get_mut(entity) else {
            tracing::debug!(
                "[ClearGlobalFloatValue] Failed to get GlobalAbilityValues for entity {}",
                invoke.entity_id
            );
            continue;
        };

        match nod_krai_gi_proto::dy_parser::decode_from_vec_by_name_version::<AbilityScalarValueEntry>(
            version,
            "AbilityScalarValueEntry",
            &invoke.ability_data,
        ) {
            None => {
                tracing::debug!("[ClearGlobalFloatValue] Failed to decode AbilityScalarValueEntry");
            }
            Some(entry) => match get_ability_name(entry.key) {
                None => {
                    tracing::debug!(
                        "[ClearGlobalFloatValue] No key provided for clear global float value"
                    );
                    continue;
                }
                Some(key) => {
                    tracing::debug!(
                        "[ClearGlobalFloatValue] Cleared global ability value {} for entity {}",
                        key,
                        invoke.entity_id
                    );

                    global_ability_values.0.remove(&key);
                }
            },
        }
    }
}
