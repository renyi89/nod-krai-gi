use crate::util::{calc_amount, eval_option};
use bevy_ecs::prelude::*;
use nod_krai_gi_data::prop_type::FightPropType;
use nod_krai_gi_entity::common::{EntityById, FightProperties, OwnerProtocolEntityID};
use nod_krai_gi_event::ability::*;

pub fn ability_action_heal_hp_event(
    index: Res<EntityById>,
    mut events: MessageReader<AbilityActionHealHPEvent>,
    mut fight_props_query: Query<&mut FightProperties>,
    abilities_query: Query<(
        Option<&OwnerProtocolEntityID>,
        &nod_krai_gi_entity::common::InstancedAbilities,
    )>,
) {
    for AbilityActionHealHPEvent(
        ability_index,
        ability_entity,
        action,
        _ability_data,
        target_entity,
    ) in events.read()
    {
        let Ok((owner_protocol_entity_id, abilities)) = abilities_query.get(*ability_entity) else {
            tracing::debug!(
                "[AbilityActionHealHPEvent] Failed to get entity components for {}",
                ability_entity
            );
            continue;
        };
        let Some(ability) = abilities.list.get(*ability_index as usize).cloned() else {
            tracing::debug!(
                "[AbilityActionHealHPEvent] Ability not found for index: {} entity: {}",
                ability_index,
                ability_entity
            );
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
                Some(owner_protocol_entity_id) => {
                    match index.0.get(&owner_protocol_entity_id) {
                        None => {
                            tracing::debug!(
                                "[AbilityActionHealHPEvent] owner_protocol_entity_id.0 {} not found",owner_protocol_entity_id);
                            continue;
                        }
                        Some(temp_entity) => {
                            owner_entity = Some(*temp_entity);
                        }
                    }
                }
            },
        }

        match owner_entity {
            None => {
                tracing::debug!("[AbilityActionHealHPEvent] owner_entity.is_none ");
                continue;
            }
            Some(owner_entity) => {
                let Ok(caster_props) = fight_props_query.get(owner_entity) else {
                    tracing::debug!("[AbilityActionHealHPEvent] owner_entity props not found");
                    continue;
                };

                let Ok(target_props) = fight_props_query.get(*target_entity) else {
                    tracing::debug!("[AbilityActionHealHPEvent] target_entity props not found");
                    continue;
                };

                let amount = calc_amount(&ability, caster_props, target_props, action);

                let mut ability_ratio = 1.0f32;
                if !action.ignore_ability_property.unwrap_or_default() {
                    ability_ratio += caster_props.get_property(FightPropType::FIGHT_PROP_HEAL_ADD)
                        + target_props.get_property(FightPropType::FIGHT_PROP_HEALED_ADD);
                }

                let heal_ratio_value =
                    eval_option(&ability, Some(caster_props), &action.heal_ratio, 1.0);

                let Ok(mut target_props) = fight_props_query.get_mut(*target_entity) else {
                    tracing::debug!("[AbilityActionHealHPEvent] target_entity props not found");
                    continue;
                };

                let change_cur_hp_value = amount * ability_ratio * heal_ratio_value;

                tracing::debug!(
                    "[AbilityActionHealHPEvent] change_cur_hp_value: {}",
                    change_cur_hp_value
                );

                target_props.change_cur_hp(change_cur_hp_value);
            }
        }
    }
}
