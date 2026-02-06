use bevy_ecs::prelude::*;
use nod_krai_gi_entity::common::{EntityById, GlobalAbilityValues};
use nod_krai_gi_proto::AbilityScalarValueEntry;

use nod_krai_gi_event::ability::*;
use crate::util::get_ability_name;

pub fn handle_global_float_value(
    index: Res<EntityById>,
    mut events: MessageReader<GlobalFloatValueEvent>,
    mut entities: Query<&mut GlobalAbilityValues>,
) {
    for GlobalFloatValueEvent(invoke, version) in events.read() {
        let entity = match index.0.get(&invoke.entity_id) {
            Some(e) => *e,
            None => {
                tracing::debug!("[GlobalFloatValue] Entity {} not found", invoke.entity_id);
                continue;
            }
        };

        let Ok(mut global_ability_values) = entities.get_mut(entity) else {
            tracing::debug!(
                "[GlobalFloatValue] Failed to get GlobalAbilityValues for entity {}",
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
                tracing::debug!("[GlobalFloatValue] Failed to decode AbilityScalarValueEntry");
            }
            Some(entry) => match get_ability_name(entry.key) {
                None => {
                    tracing::debug!("[GlobalFloatValue] No key provided for global float value");
                    continue;
                }
                Some(key) => {
                    if key.starts_with("SGV_") {
                        continue;
                    }

                    let value = entry.float_value;

                    tracing::debug!(
                        "[GlobalFloatValue] Setting global ability value {} = {} for entity {}",
                        key,
                        value,
                        invoke.entity_id
                    );

                    global_ability_values.0.insert(key, value);
                }
            },
        }
    }
}
