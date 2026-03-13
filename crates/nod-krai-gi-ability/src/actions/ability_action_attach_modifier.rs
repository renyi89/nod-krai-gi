use bevy_ecs::prelude::*;
use nod_krai_gi_data::GAME_SERVER_CONFIG;
use nod_krai_gi_data::ability::{AbilityModifierProperty};
use nod_krai_gi_data::prop_type::FightPropType;
use nod_krai_gi_entity::common::{FightProperties, InstancedAbilities, InstancedModifiers, AbilityModifierController};
use nod_krai_gi_event::ability::{AbilityActionAttachModifierEvent, ExecuteActionEvent};
use std::collections::HashMap;
use common::string_util::InternString;

use crate::util::eval_option;

pub fn ability_action_attach_modifier_event(
    mut events: MessageReader<AbilityActionAttachModifierEvent>,
    mut abilities_query: Query<(&mut InstancedAbilities, &mut InstancedModifiers, Option<&mut FightProperties>)>,
    mut execute_action_events: MessageWriter<ExecuteActionEvent>,
) {
    for event in events.read() {
        let ability_entity = event.1;
        let ability_index = event.0;
        let action = &event.2;
        let target_entity = event.4;

        let Ok((mut abilities, mut modifiers, fight_props)) = abilities_query.get_mut(ability_entity) else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_attach_modifier_event] Entity {} not found or has no InstancedAbilities",
                    ability_entity
                );
            }
            continue;
        };

        let ability = match abilities.list.get_mut(ability_index as usize) {
            Some(ability) => ability,
            None => {
                if GAME_SERVER_CONFIG.plugin.ability_log {
                    tracing::debug!(
                        "[ability_action_attach_modifier_event] Ability index {} not found for entity {}",
                        ability_index,
                        ability_entity
                    );
                }
                continue;
            }
        };

        let modifier_name = action.modifier_name.as_str();

        let Some(ability_data) = &ability.ability_data else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_attach_modifier_event] No ability_data for ability index {} on entity {}",
                    ability_index,
                    ability_entity
                );
            }
            continue;
        };

        let Some((_, modifier_def)) = ability_data.modifiers.iter().find(|(name, _)| name.as_str() == modifier_name) else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_attach_modifier_event] Modifier {} not found in ability {}",
                    modifier_name,
                    ability_data.ability_name
                );
            }
            continue;
        };

        let duration = eval_option(&ability, None, &modifier_def.duration, -1.0);
        let think_interval = modifier_def.think_interval.as_ref().map(|t| {
            eval_option(&ability, None, &Some(t.clone()), 0.0)
        });

        let modifier_name_intern = InternString::from(modifier_name);

        let mut property_deltas = HashMap::new();
        if let Some(properties) = &modifier_def.properties {
            if let Some(mut fight_props) = fight_props {
                apply_modifier_properties(&ability, &mut fight_props, properties, &mut property_deltas);
            }
        }

        let instanced_modifier_id = ability.instanced_ability_id.unwrap_or(0) * 1000 + ability.modifiers.len() as u32 + 1;

        let modifier_controller = AbilityModifierController::new(
            instanced_modifier_id,
            modifier_name.to_string(),
            ability_index,
            Some(target_entity),
        );

        modifiers.insert(instanced_modifier_id, modifier_controller);
        ability.modifiers.insert(modifier_name_intern, instanced_modifier_id);

        for action in &modifier_def.on_added {
            execute_action_events.write(ExecuteActionEvent(
                ability_index,
                ability_entity,
                action.clone(),
                Vec::new(),
                Some(target_entity),
            ));
        }

        if GAME_SERVER_CONFIG.plugin.ability_log {
            tracing::debug!(
                "[ability_action_attach_modifier_event] Attached modifier: {} to entity {} (id: {}, duration: {:?}, think_interval: {:?})",
                modifier_name,
                target_entity,
                instanced_modifier_id,
                duration,
                think_interval
            );
        }
    }
}

fn apply_modifier_properties(
    ability: &nod_krai_gi_entity::common::InstancedAbility,
    fight_props: &mut FightProperties,
    properties: &AbilityModifierProperty,
    deltas: &mut HashMap<FightPropType, f32>,
) {
    if let Some(max_hp_ratio) = &properties.actor_max_hp_ratio {
        let value = eval_option(ability, Some(fight_props), &Some(max_hp_ratio.clone()), 0.0);
        if value != 0.0 {
            fight_props.change_property(FightPropType::FIGHT_PROP_HP_PERCENT, value);
            deltas.insert(FightPropType::FIGHT_PROP_HP_PERCENT, value);
        }
    }

    if let Some(attack_ratio) = &properties.actor_attack_s_ratio {
        let value = eval_option(ability, Some(fight_props), &Some(attack_ratio.clone()), 0.0);
        if value != 0.0 {
            fight_props.change_property(FightPropType::FIGHT_PROP_ATTACK_PERCENT, value);
            deltas.insert(FightPropType::FIGHT_PROP_ATTACK_PERCENT, value);
        }
    }

    if let Some(healed_add) = &properties.actor_healed_add_delta {
        let value = eval_option(ability, Some(fight_props), &Some(healed_add.clone()), 0.0);
        if value != 0.0 {
            fight_props.change_property(FightPropType::FIGHT_PROP_HEALED_ADD, value);
            deltas.insert(FightPropType::FIGHT_PROP_HEALED_ADD, value);
        }
    }

    if let Some(hp_threshold) = &properties.actor_hp_threshold_ratio {
        let value = eval_option(ability, Some(fight_props), &Some(hp_threshold.clone()), 0.0);
        if value != 0.0 && GAME_SERVER_CONFIG.plugin.ability_log {
            tracing::debug!(
                "[apply_modifier_properties] Actor_HpThresholdRatio = {} (not applied - no corresponding FightPropType)",
                value
            );
        }
    }

    fight_props.flush_property();
}
