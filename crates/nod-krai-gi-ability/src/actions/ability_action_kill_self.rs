use bevy_ecs::prelude::*;
use nod_krai_gi_data::GAME_SERVER_CONFIG;
use nod_krai_gi_entity::common::{FightProperties, InstancedAbilities};
use nod_krai_gi_event::ability::AbilityActionKillSelfEvent;
use crate::util::eval_option;

pub fn ability_action_kill_self_event(
    mut events: MessageReader<AbilityActionKillSelfEvent>,
    mut fight_props_query: Query<&mut FightProperties>,
    abilities_query: Query<&InstancedAbilities>,
) {
    for AbilityActionKillSelfEvent(
        ability_index,
        ability_entity,
        action,
        _ability_data,
        target_entities,
    ) in events.read()
    {
        let ability = match abilities_query.get(*ability_entity) {
            Ok(abilities) => abilities.list.get(*ability_index as usize).cloned(),
            Err(_) => None,
        };

        let ban_drop = action.ban_drop;
        let ban_hppercentage_drop = action.ban_hppercentage_drop;
        let kill_self_type = action.kill_self_type.as_str();
        let duration = eval_option(&ability.unwrap_or_default(), None, &action.duration, 0.0);

        if GAME_SERVER_CONFIG.plugin.ability_log {
            tracing::debug!(
                "[ability_action_kill_self_event] Kill self: targets count={}, ban_drop={}, ban_hppercentage_drop={}, kill_self_type={}, duration={}",
                target_entities.len(),
                ban_drop,
                ban_hppercentage_drop,
                kill_self_type,
                duration
            );
        }

        if duration > 0.0 {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_kill_self_event] Delayed kill self not yet implemented with duration {}",
                    duration
                );
            }
        }

        for target_entity in target_entities {
            let Ok(mut target_props) = fight_props_query.get_mut(*target_entity) else {
                if GAME_SERVER_CONFIG.plugin.ability_log {
                    tracing::debug!("[ability_action_kill_self_event] target_entity props not found");
                }
                continue;
            };

            target_props.change_cur_hp(-100000000.0);
        }
    }
}