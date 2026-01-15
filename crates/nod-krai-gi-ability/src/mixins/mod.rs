use crate::server_invoke::ExecuteMixinEvent;
use bevy_ecs::prelude::*;

pub fn execute_mixin_system(mut events: MessageReader<ExecuteMixinEvent>) {
    for ExecuteMixinEvent(_ability, mixin, _ability_data, _entity, _target_entity) in events.read()
    {
        let type_name = mixin.type_name.as_deref().unwrap_or("");

        match type_name {
            _ => {
                tracing::debug!("Unhandled mixin type: {}", type_name);
            }
        }
    }
}
