use bevy_ecs::prelude::*;
use nod_krai_gi_entity::common::{EntityById, FightProperties, OwnerPlayerUID, ProtocolEntityID};
use nod_krai_gi_proto::{AbilityInvokeArgument, AbilityInvokeEntry, ProtEntityType};

#[derive(Message)]
pub struct AbilityInvokeEvent(pub AbilityInvokeEntry);

pub fn on_ability_invoke(
    index: Res<EntityById>,
    mut events: MessageReader<AbilityInvokeEvent>,
    mut entities: Query<(
        &mut FightProperties,
        &ProtocolEntityID,
        Option<&OwnerPlayerUID>,
    )>,
) {
    for AbilityInvokeEvent(invoke) in events.read() {
        let invoke_entity_id;
        let mut head_target_id = 0;
        if (invoke.entity_id >> 22) < ProtEntityType::ProtEntityMax as u32 {
            let invoke_entity = match index.0.get(&invoke.entity_id) {
                Some(e) => *e,
                None => {
                    tracing::trace!(
                        "AbilityInvokeEvent invoke.entity_id {} not found",
                        invoke.entity_id
                    );
                    continue;
                }
            };

            let Ok((_target_fight_props, _, _)) = entities.get_mut(invoke_entity) else {
                tracing::trace!(
                    "AbilityInvokeEvent invoke.entity_id {} entitie not found",
                    invoke.entity_id
                );
                continue;
            };

            invoke_entity_id = invoke.entity_id;
            let mut _head_fight_props;
            match invoke.head {
                None => {}
                Some(head) => {
                    if head.target_id != 0 {
                        let head_target_entity = match index.0.get(&head.target_id) {
                            Some(e) => *e,
                            None => {
                                tracing::trace!(
                                    "AbilityInvokeEvent head.target_id {} not found",
                                    head.target_id
                                );
                                continue;
                            }
                        };

                        let Ok((head_target_fight_props, _, _)) =
                            entities.get_mut(head_target_entity)
                        else {
                            tracing::trace!(
                                "AbilityInvokeEvent head.target_id {} entitie not found",
                                head.target_id
                            );
                            continue;
                        };

                        head_target_id = head.target_id;
                        _head_fight_props = head_target_fight_props;
                    }
                    if head.local_id != 0 {
                        tracing::trace!("AbilityInvokeEvent head.local_id:{}", head.local_id);
                        continue;
                    }
                }
            }

            tracing::trace!(
                "AbilityInvokeEvent now_entity_id:{},now_target_id:{}",
                invoke_entity_id,
                head_target_id
            );

            match invoke.argument_type() {
                AbilityInvokeArgument::AbilityMetaModifierChange => {}
                AbilityInvokeArgument::AbilityMetaOverrideParam => {}
                AbilityInvokeArgument::AbilityMetaReinitOverridemap => {}
                AbilityInvokeArgument::AbilityMetaGlobalFloatValue => {}
                AbilityInvokeArgument::AbilityMetaClearGlobalFloatValue => {}
                AbilityInvokeArgument::AbilityMetaSetKilledSetate => {}
                AbilityInvokeArgument::AbilityMetaAddNewAbility => {}
                AbilityInvokeArgument::AbilityMetaModifierDurabilityChange => {}
                AbilityInvokeArgument::AbilityMetaAddSpecialEnergyValue => {}
                AbilityInvokeArgument::AbilityActionGenerateElemBall => {}
                AbilityInvokeArgument::AbilityMixinChangePhlogiston => {}
                _ => {}
            }
        }
    }
}
