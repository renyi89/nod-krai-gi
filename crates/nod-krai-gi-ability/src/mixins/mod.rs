use bevy_ecs::prelude::*;
use nod_krai_gi_data::GAME_SERVER_CONFIG;
use nod_krai_gi_event::ability::*;

pub fn execute_mixin_system(
    mut events: MessageReader<ExecuteMixinEvent>,
    abilities_query: Query<&nod_krai_gi_entity::common::InstancedAbilities>,
) {
    for ExecuteMixinEvent(ability_index, ability_entity, mixin, _ability_data, _target_entity) in
        events.read()
    {
        let type_name = mixin.type_name;

        // Get ability from ability_index and ability_entity
        let ability = match abilities_query.get(*ability_entity) {
            Ok(abilities) => abilities.list.get(*ability_index as usize).cloned(),
            Err(_) => None,
        };
        let Some(_ability) = ability else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[execute_mixin_system] Ability not found for index: {} entity: {}",
                    ability_index,
                    ability_entity
                );
            }
            continue;
        };

        match type_name {
            _ => {
                if GAME_SERVER_CONFIG.plugin.ability_log {
                    tracing::debug!("Unhandled mixin type: {}", type_name);
                }
            }
        }
    }
}
