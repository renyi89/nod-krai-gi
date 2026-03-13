use crate::util::resolve_target_entity_by_str;
use bevy_ecs::prelude::*;
use nod_krai_gi_data::GAME_SERVER_CONFIG;
use nod_krai_gi_entity::avatar::{CurrentPlayerAvatarMarker, CurrentTeam};
use nod_krai_gi_entity::common::{
    EntityById, GlobalAbilityValues, InstancedAbilities, OwnerProtocolEntityID, ProtocolEntityID,
};
use nod_krai_gi_entity::team::TeamEntityMarker;
use nod_krai_gi_event::ability::AbilityActionCopyGlobalValueEvent;

pub fn ability_action_copy_global_value_event(
    mut events: MessageReader<AbilityActionCopyGlobalValueEvent>,
    abilities_query: Query<&InstancedAbilities>,
    mut global_values_query: Query<&mut GlobalAbilityValues>,
    entity_by_id: Res<EntityById>,
    entity_query: Query<(
        Entity,
        &ProtocolEntityID,
        Option<&OwnerProtocolEntityID>,
        Option<&CurrentPlayerAvatarMarker>,
        Option<&CurrentTeam>,
        Option<&TeamEntityMarker>,
    )>,
) {
    for AbilityActionCopyGlobalValueEvent(
        ability_index,
        ability_entity,
        action,
        _ability_data,
        target_entity,
    ) in events.read()
    {
        let ability = match abilities_query.get(*ability_entity) {
            Ok(abilities) => abilities.list.get(*ability_index as usize).cloned(),
            Err(_) => None,
        };

        let Some(_ability) = ability else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_copy_global_value_event] Ability not found for index: {} entity: {}",
                    ability_index,
                    ability_entity
                );
            }
            continue;
        };

        let src_key = action.src_key.as_str();
        let dst_key = action.dst_key.as_str();
        let src_target = action.src_target.as_str();
        let dst_target = action.dst_target.as_str();

        let source_entity = resolve_target_entity_by_str(
            src_target,
            *ability_entity,
            Some(*target_entity),
            &entity_by_id,
            &entity_query,
        );

        let dest_entity = resolve_target_entity_by_str(
            dst_target,
            *ability_entity,
            Some(*target_entity),
            &entity_by_id,
            &entity_query,
        );

        let Some(source_entity) = source_entity else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_copy_global_value_event] Failed to resolve source entity for src_target: {}",
                    src_target
                );
            }
            continue;
        };

        let Some(dest_entity) = dest_entity else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_copy_global_value_event] Failed to resolve dest entity for dst_target: {}",
                    dst_target
                );
            }
            continue;
        };

        let src_value = match global_values_query.get(source_entity) {
            Ok(global_values) => global_values.0.get(&src_key.into()).copied().unwrap_or(0.0),
            Err(_) => 0.0,
        };

        let is_add = action.is_add;

        let final_value = if is_add {
            let current_value = match global_values_query.get(dest_entity) {
                Ok(global_values) => global_values.0.get(&dst_key.into()).copied().unwrap_or(0.0),
                Err(_) => 0.0,
            };
            current_value + src_value
        } else {
            src_value
        };

        if let Ok(mut global_values) = global_values_query.get_mut(dest_entity) {
            global_values.0.insert(dst_key.into(), final_value);
        }

        if GAME_SERVER_CONFIG.plugin.ability_log {
            tracing::debug!(
                "[ability_action_copy_global_value_event] Copy global value: src_key={}, dst_key={}, src_target={}, dst_target={}, src_value={}, is_add={}, final_value={}",
                src_key,
                dst_key,
                src_target,
                dst_target,
                src_value,
                is_add,
                final_value
            );
        }
    }
}
