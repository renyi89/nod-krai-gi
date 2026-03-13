use crate::util::{calc_amount, eval_option};
use bevy_ecs::prelude::*;
use nod_krai_gi_data::prop_type::FightPropType;
use nod_krai_gi_data::GAME_SERVER_CONFIG;
use nod_krai_gi_entity::common::{EntityById, FightProperties, InstancedAbilities, InstancedModifiers, OwnerProtocolEntityID};
use nod_krai_gi_event::ability::*;

pub fn ability_action_heal_hp_event(
    index: Res<EntityById>,
    mut events: MessageReader<AbilityActionHealHPEvent>,
    mut fight_props_query: Query<&mut FightProperties>,
    entities_query: Query<(
        Option<&OwnerProtocolEntityID>,
        &InstancedAbilities,
        &InstancedModifiers,
    )>,
) {
    for AbilityActionHealHPEvent(
        ability_index,
        ability_entity,
        action,
        _ability_data,
        target_entities,
    ) in events.read()
    {
        let Ok((owner_protocol_entity_id, abilities, _)) = entities_query.get(*ability_entity) else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_heal_hp_event] Failed to get entity components for {}",
                    ability_entity
                );
            }
            continue;
        };
        let Some(ability) = abilities.list.get(*ability_index as usize).cloned() else {
            if GAME_SERVER_CONFIG.plugin.ability_log {
                tracing::debug!(
                    "[ability_action_heal_hp_event] Ability not found for index: {} entity: {}",
                    ability_index,
                    ability_entity
                );
            }
            continue;
        };

        let owner_entity;

        match owner_protocol_entity_id {
            None => {
                owner_entity = Some(*ability_entity);
            }
            Some(owner_protocol_entity_id) => match owner_protocol_entity_id.0 {
                None => {
                    owner_entity = Some(*ability_entity);
                }
                Some(owner_protocol_entity_id) => match index.0.get(&owner_protocol_entity_id) {
                    None => {
                        if GAME_SERVER_CONFIG.plugin.ability_log {
                            tracing::debug!(
                                    "[ability_action_heal_hp_event] owner_protocol_entity_id.0 {} not found",owner_protocol_entity_id);
                        }
                        continue;
                    }
                    Some(temp_entity) => {
                        owner_entity = Some(*temp_entity);
                    }
                },
            },
        }

        match owner_entity {
            None => {
                if GAME_SERVER_CONFIG.plugin.ability_log {
                    tracing::debug!("[ability_action_heal_hp_event] owner_entity.is_none ");
                }
                continue;
            }
            Some(owner_entity) => {
                let caster_heal_add;
                {
                    let Ok(caster_props) = fight_props_query.get(owner_entity) else {
                        if GAME_SERVER_CONFIG.plugin.ability_log {
                            tracing::debug!("[ability_action_heal_hp_event] owner_entity props not found");
                        }
                        continue;
                    };
                    caster_heal_add = caster_props.get_property(FightPropType::FIGHT_PROP_HEAL_ADD);
                }

                for target_entity in target_entities {
                    let (target_healed_add, amount, ability_ratio, heal_ratio_value, change_cur_hp_value);
                    {
                        let Ok(caster_props) = fight_props_query.get(owner_entity) else {
                            if GAME_SERVER_CONFIG.plugin.ability_log {
                                tracing::debug!("[ability_action_heal_hp_event] owner_entity props not found");
                            }
                            continue;
                        };
                        let Ok(target_props) = fight_props_query.get(*target_entity) else {
                            if GAME_SERVER_CONFIG.plugin.ability_log {
                                tracing::debug!("[ability_action_heal_hp_event] target_entity props not found");
                            }
                            continue;
                        };

                        amount = calc_amount(&ability, caster_props, target_props, action);
                        target_healed_add = target_props.get_property(FightPropType::FIGHT_PROP_HEALED_ADD);
                        ability_ratio = 1.0f32 + if !action.ignore_ability_property {
                            caster_heal_add + target_healed_add
                        } else {
                            0.0
                        };
                        heal_ratio_value = eval_option(&ability, Some(caster_props), &action.heal_ratio, 1.0);
                        change_cur_hp_value = amount * ability_ratio * heal_ratio_value;
                    }

                    if GAME_SERVER_CONFIG.plugin.ability_log {
                        tracing::debug!(
                            "[ability_action_heal_hp_event] change_cur_hp_value: {}",
                            change_cur_hp_value
                        );
                    }

                    let Ok(mut target_props) = fight_props_query.get_mut(*target_entity) else {
                        if GAME_SERVER_CONFIG.plugin.ability_log {
                            tracing::debug!("[ability_action_heal_hp_event] target_entity props not found");
                        }
                        continue;
                    };

                    target_props.change_cur_hp(change_cur_hp_value);
                }
            }
        }
    }
}
