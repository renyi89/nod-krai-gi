use crate::util::calc_amount;
use bevy_ecs::prelude::*;
use nod_krai_gi_data::prop_type::FightPropType;
use nod_krai_gi_entity::common::{EntityById, FightProperties, OwnerProtocolEntityID};
use nod_krai_gi_event::ability::*;

pub fn ability_action_lose_hp_event(
    index: Res<EntityById>,
    mut events: MessageReader<AbilityActionLoseHPEvent>,
    mut fight_props_query: Query<&mut FightProperties>,
    abilities_query: Query<(
        Option<&OwnerProtocolEntityID>,
        &nod_krai_gi_entity::common::InstancedAbilities,
    )>,
) {
    for AbilityActionLoseHPEvent(
        ability_index,
        ability_entity,
        action,
        _ability_data,
        target_entity,
    ) in events.read()
    {
        let Ok((owner_protocol_entity_id, abilities)) = abilities_query.get(*ability_entity) else {
            tracing::debug!(
                "[AbilityActionLoseHPEvent] Failed to get entity components for {}",
                ability_entity
            );
            continue;
        };
        let Some(ability) = abilities.list.get(*ability_index as usize).cloned() else {
            tracing::debug!(
                "[AbilityActionLoseHPEvent] Ability not found for index: {} entity: {}",
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
                tracing::debug!("[AbilityActionLoseHPEvent] owner_entity.is_none ");
                continue;
            }
            Some(owner_entity) => {
                let Ok(caster_props) = fight_props_query.get(owner_entity) else {
                    tracing::debug!("[AbilityActionLoseHPEvent] owner_entity props not found");
                    continue;
                };

                let Ok(target_props) = fight_props_query.get(*target_entity) else {
                    tracing::debug!("[AbilityActionLoseHPEvent] target_entity props not found");
                    continue;
                };

                let mut amount = calc_amount(&ability, caster_props, target_props, action);

                if target_props.get_property(FightPropType::FIGHT_PROP_CUR_HP) < amount + 0.01
                    && !action.lethal.unwrap_or_default()
                {
                    amount = 0.0;
                }

                let Ok(mut target_props) = fight_props_query.get_mut(*target_entity) else {
                    tracing::debug!("[AbilityActionLoseHPEvent] target_entity props not found");
                    continue;
                };

                tracing::debug!(
                    "[AbilityActionLoseHPEvent] change_cur_hp_value: {}",
                    -amount
                );

                target_props.change_cur_hp(-amount);
            }
        }
    }
}
