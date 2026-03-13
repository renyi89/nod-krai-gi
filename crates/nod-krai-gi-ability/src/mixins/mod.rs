use bevy_ecs::prelude::*;
use nod_krai_gi_data::GAME_SERVER_CONFIG;
use nod_krai_gi_event::ability::*;

pub fn execute_mixin_system(
    mut events: MessageReader<ExecuteMixinEvent>,
) {
    for ExecuteMixinEvent(_ability_index, _ability_entity, mixin, _ability_data, _target_entity) in
        events.read()
    {

        if GAME_SERVER_CONFIG.plugin.ability_log {
            tracing::debug!("[execute_mixin_system] mixin type: {}", mixin.type_name);
        }

        match mixin.type_name.as_str() {
            _ => {
            }
        }
    }
}
