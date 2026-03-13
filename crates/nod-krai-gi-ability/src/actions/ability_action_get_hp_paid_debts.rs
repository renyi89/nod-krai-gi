use bevy_ecs::prelude::*;
use nod_krai_gi_data::prop_type::FightPropType;
use nod_krai_gi_data::GAME_SERVER_CONFIG;
use nod_krai_gi_event::ability::*;

pub fn ability_action_get_hp_paid_debts_event(
    mut events: MessageReader<AbilityActionGetHPPaidDebtsEvent>,
    mut fight_props_query: Query<&mut nod_krai_gi_entity::common::FightProperties>,
    mut abilities_query: Query<&mut nod_krai_gi_entity::common::InstancedAbilities>,
) {
    for AbilityActionGetHPPaidDebtsEvent(
        ability_index,
        ability_entity,
        action,
        _ability_data,
        target_entities,
    ) in events.read()
    {
        let override_map_key = action.override_map_key;

        if override_map_key.is_empty() {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!("[ability_action_get_hp_paid_debts_event] Missing override_map_key");
            }
            continue;
        }

        for target_entity in target_entities {
            let Ok(mut fight_props) = fight_props_query.get_mut(*target_entity) else {
                if GAME_SERVER_CONFIG.plugin.ability_log {
                    tracing::debug!(
                        "[ability_action_get_hp_paid_debts_event] Failed to get fight properties for entity {}",
                        target_entity
                    );
                }
                continue;
            };

            let Ok(mut abilities) = abilities_query.get_mut(*ability_entity) else {
                if GAME_SERVER_CONFIG.plugin.ability_log {
                    tracing::debug!(
                        "[ability_action_get_hp_paid_debts_event] Failed to get abilities for entity {}",
                        ability_entity
                    );
                }
                continue;
            };

            let mut paid_debt = fight_props.get_property(FightPropType::FIGHT_PROP_CUR_HP_PAID_DEBTS);

            if paid_debt < 0.0 {
                paid_debt = 0.0;
            }

            if let Some(ability) = abilities.list.get_mut(*ability_index as usize) {
                ability.ability_specials.insert(override_map_key.clone(), paid_debt);
                if GAME_SERVER_CONFIG.plugin.ability_log {
                    tracing::debug!(
                        "[ability_action_get_hp_paid_debts_event] Setting override map value {} to {}",
                        override_map_key,
                        paid_debt
                    );
                }
            }

            fight_props.set_property(FightPropType::FIGHT_PROP_CUR_HP_PAID_DEBTS, paid_debt);
        }
    }
}
