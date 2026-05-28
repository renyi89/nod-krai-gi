use bevy_ecs::prelude::*;

use nod_krai_gi_entity::common::FightProperties;
use nod_krai_gi_event::ability::ExecuteActionEvent;

pub fn ability_action_kill_self_event(
    mut events: MessageReader<ExecuteActionEvent>,
    mut fight_props_query: Query<&mut FightProperties>,
) {
    for ExecuteActionEvent(_ability_index, ability_entity, action, _ability_data, target_entity) in
        events.read()
    {
        if action.type_name != "KillSelf" {
            continue;
        }

        let target_entity = target_entity.unwrap_or(*ability_entity);

        let Ok(mut target_props) = fight_props_query.get_mut(target_entity) else {
            tracing::debug!(target: "ability", "[ability_action_kill_self_event] target_entity props not found");
            continue;
        };

        target_props.change_cur_hp(-100000000.0);
    }
}
