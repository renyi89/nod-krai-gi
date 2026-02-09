use crate::util::eval_option;
use bevy_ecs::prelude::*;
use nod_krai_gi_data::prop_type::FightPropType;
use nod_krai_gi_data::GAME_SERVER_CONFIG;
use nod_krai_gi_entity::common::ProtocolEntityID;
use nod_krai_gi_entity::fight::{ChangeReason, EntityFightPropChangeReasonNotifyEvent};
use nod_krai_gi_event::ability::*;
use nod_krai_gi_proto::normal::{ChangeHpDebtsReason, PropChangeReason};

pub fn ability_action_reduce_hp_debts_event(
    mut events: MessageReader<AbilityActionReduceHPDebtsEvent>,
    mut fight_props_query: Query<(
        &ProtocolEntityID,
        &mut nod_krai_gi_entity::common::FightProperties,
    )>,
    abilities_query: Query<&nod_krai_gi_entity::common::InstancedAbilities>,
    mut reason_events: MessageWriter<EntityFightPropChangeReasonNotifyEvent>,
) {
    for AbilityActionReduceHPDebtsEvent(
        ability_index,
        ability_entity,
        action,
        _ability_data,
        target_entity,
    ) in events.read()
    {
        // Get fight properties from target entity
        let Ok((target_entity_id, mut fight_props)) = fight_props_query.get_mut(*target_entity)
        else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[AbilityActionReduceHPDebtsEvent] Failed to get fight properties for entity {}",
                    target_entity
                );
            }
            continue;
        };

        // Get abilities from ability entity to access ability specials
        let Ok(abilities) = abilities_query.get(*ability_entity) else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[AbilityActionReduceHPDebtsEvent] Failed to get abilities for entity {}",
                    ability_entity
                );
            }
            continue;
        };

        // Calculate debt reduction amount using ratio property
        let ability = match abilities.list.get(*ability_index as usize) {
            Some(ability) => ability,
            None => {
                if GAME_SERVER_CONFIG.plugin.ability_log {
                    tracing::debug!(
                        "[AbilityActionReduceHPDebtsEvent] Failed to get ability at index {} for entity {}",
                        ability_index,
                        ability_entity
                    );
                }
                continue;
            }
        };

        let debt_reduction = eval_option(ability, Some(&fight_props), &action.value, 0.0);
        if GAME_SERVER_CONFIG.plugin.ability_log {
            tracing::debug!(
                "[AbilityActionReduceHPDebtsEvent] Calculated debt reduction: {}",
                debt_reduction
            );
        }

        // Get current HP debts and calculate new debt
        let cur_debt = fight_props.get_property(FightPropType::FIGHT_PROP_CUR_HP_DEBTS);
        let mut new_debt = cur_debt - debt_reduction;

        // Ensure debt is non-negative
        if new_debt < 0.0 {
            new_debt = 0.0;
        }

        // Ensure debt doesn't exceed 2x max HP
        let max_hp = fight_props.get_property(FightPropType::FIGHT_PROP_MAX_HP);
        let max_debt_limit = 2.0 * max_hp;
        if new_debt > max_debt_limit {
            tracing::warn!(
                "[AbilityActionReduceHPDebtsEvent] HP debt surpassed its limit, setting to max"
            );
            new_debt = max_debt_limit;
        }

        // Update the target entity's HP debts
        fight_props.set_property(FightPropType::FIGHT_PROP_CUR_HP_DEBTS, new_debt);
        fight_props.flush_property();
        if new_debt != cur_debt {
            reason_events.write(EntityFightPropChangeReasonNotifyEvent {
                entity_id: target_entity_id.0,
                prop_type: FightPropType::FIGHT_PROP_CUR_HP_DEBTS,
                value: new_debt - cur_debt,
                param_list: None,
                reason: PropChangeReason::PropChangeAbility,
                change_reason: {
                    if new_debt > cur_debt {
                        ChangeReason::ChangeHpDebtsReason(
                            ChangeHpDebtsReason::ChangeHpDebtsAddAbility,
                        )
                    } else {
                        ChangeReason::ChangeHpDebtsReason(
                            ChangeHpDebtsReason::ChangeHpDebtsReduceAbility,
                        )
                    }
                },
            });
        }
        if GAME_SERVER_CONFIG.plugin.ability_log {
            tracing::debug!(
                "[AbilityActionReduceHPDebtsEvent] Updated HP debts from {} to {} for entity {}",
                cur_debt,
                new_debt,
                target_entity
            );
        }
    }
}
