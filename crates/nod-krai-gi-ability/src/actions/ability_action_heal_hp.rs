use crate::util::{calc_amount, eval_option};
use bevy_ecs::prelude::*;
use nod_krai_gi_data::ability::AbilityModifierAction;
use nod_krai_gi_data::prop_type::FightPropType;
use nod_krai_gi_entity::common::{
    EntityById, FightProperties, InstancedAbility, OwnerProtocolEntityID, ProtocolEntityID,
};
use nod_krai_gi_proto::ProtEntityType;

#[derive(Message)]
pub struct AbilityActionHealHPEvent(
    pub InstancedAbility,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Entity,
    pub Entity,
);

pub fn ability_action_heal_hp_event(
    index: Res<EntityById>,
    mut events: MessageReader<AbilityActionHealHPEvent>,
    mut entities: Query<(
        &mut FightProperties,
        &ProtocolEntityID,
        Option<&OwnerProtocolEntityID>,
    )>,
) {
    for AbilityActionHealHPEvent(ability, action, _ability_data, entity, target_entity) in
        events.read()
    {
        let mut owner_entity = None;
        let Ok((_, protocol_entity_id, owner_protocol_entity_id)) = entities.get(*entity) else {
            tracing::debug!(
                "[AbilityActionHealHPEvent] Failed to get entity components for {}",
                entity
            );
            continue;
        };
        if protocol_entity_id.entity_type() == ProtEntityType::ProtEntityGadget {
            match owner_protocol_entity_id {
                None => {}
                Some(owner_protocol_entity_id) => match index.0.get(&owner_protocol_entity_id.0) {
                    None => {
                        tracing::debug!(
                            "[AbilityActionHealHPEvent] owner_protocol_entity_id.0 {} not found",
                            owner_protocol_entity_id.0
                        );
                        continue;
                    }
                    Some(temp_entity) => {
                        owner_entity = Some(*temp_entity);
                    }
                },
            }
        } else {
            owner_entity = Some(*entity);
        }

        match owner_entity {
            None => {
                tracing::debug!("[AbilityActionHealHPEvent] owner_entity.is_none ");
                continue;
            }
            Some(owner_entity) => {
                let Ok((caster_props, _, _)) = entities.get(owner_entity) else {
                    tracing::debug!("[AbilityActionHealHPEvent] owner_entity  not found");
                    continue;
                };

                let Ok((target_props, _, _)) = entities.get(*target_entity) else {
                    tracing::debug!("[AbilityActionHealHPEvent] owner_entity  not found");
                    continue;
                };

                let amount = calc_amount(ability, caster_props, target_props, action);

                let mut ability_ratio = 1.0f32;
                if !action.ignore_ability_property.unwrap_or_default() {
                    ability_ratio += caster_props.get_property(FightPropType::FIGHT_PROP_HEAL_ADD)
                        + target_props.get_property(FightPropType::FIGHT_PROP_HEALED_ADD);
                }

                let Ok((mut target_props, _, _)) = entities.get_mut(*target_entity) else {
                    tracing::debug!("[AbilityActionHealHPEvent] owner_entity  not found");
                    continue;
                };

                let change_cur_hp_value =
                    amount * ability_ratio * eval_option(ability, &action.heal_ratio, 1.0);

                tracing::debug!(
                    "[AbilityActionHealHPEvent] change_cur_hp_value: {}",
                    change_cur_hp_value
                );

                target_props.change_cur_hp(change_cur_hp_value);
            }
        }
    }
}
