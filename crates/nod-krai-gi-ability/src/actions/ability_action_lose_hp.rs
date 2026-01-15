use crate::util::calc_amount;
use bevy_ecs::prelude::*;
use nod_krai_gi_data::ability::AbilityModifierAction;
use nod_krai_gi_data::prop_type::FightPropType;
use nod_krai_gi_entity::common::{
    EntityById, FightProperties, InstancedAbility, OwnerProtocolEntityID, ProtocolEntityID,
};
use nod_krai_gi_proto::ProtEntityType;

#[derive(Message)]
pub struct AbilityActionLoseHPEvent(
    pub InstancedAbility,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Entity,
    pub Entity,
);
pub fn ability_action_lose_hp_event(
    index: Res<EntityById>,
    mut events: MessageReader<AbilityActionLoseHPEvent>,
    mut entities: Query<(
        &mut FightProperties,
        &ProtocolEntityID,
        Option<&OwnerProtocolEntityID>,
    )>,
) {
    for AbilityActionLoseHPEvent(ability, action, _ability_data, entity, target_entity) in
        events.read()
    {
        let mut owner_entity = None;
        let Ok((_, protocol_entity_id, owner_protocol_entity_id)) = entities.get(*entity) else {
            tracing::debug!(
                "[AbilityActionLoseHPEvent] Failed to get entity components for {}",
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
                            "[AbilityActionLoseHPEvent] owner_protocol_entity_id.0 {} not found",
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
                tracing::debug!("[AbilityActionLoseHPEvent] owner_entity.is_none ");
                continue;
            }
            Some(owner_entity) => {
                let Ok((caster_props, _, _)) = entities.get(owner_entity) else {
                    tracing::debug!("[AbilityActionLoseHPEvent] owner_entity  not found");
                    continue;
                };

                let Ok((target_props, _, _)) = entities.get(*target_entity) else {
                    tracing::debug!("[AbilityActionLoseHPEvent] owner_entity  not found");
                    continue;
                };

                let mut amount = calc_amount(ability, caster_props, target_props, action);

                if target_props.get_property(FightPropType::FIGHT_PROP_CUR_HP) < amount + 0.01
                    && !action.lethal.unwrap_or_default()
                {
                    amount = 0.0;
                }

                let Ok((mut target_props, _, _)) = entities.get_mut(*target_entity) else {
                    tracing::debug!("[AbilityActionLoseHPEvent] owner_entity  not found");
                    continue;
                };

                tracing::debug!(
                    "[AbilityActionLoseHPEvent] change_cur_hp_value : {}",
                    -amount
                );

                target_props.change_cur_hp(-amount);
            }
        }
    }
}
