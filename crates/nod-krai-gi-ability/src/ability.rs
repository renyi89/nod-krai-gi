use bevy_ecs::prelude::*;
use nod_krai_gi_entity::common::{FightProperties, OwnerPlayerUID, ProtocolEntityID};
use nod_krai_gi_proto::{AbilityInvokeArgument, AbilityInvokeEntry, ProtEntityType};

#[derive(Message)]
pub struct AbilityInvokeEvent(pub AbilityInvokeEntry);

pub fn on_ability_invoke(
    mut events: MessageReader<AbilityInvokeEvent>,
    mut entities: Query<(
        &mut FightProperties,
        &ProtocolEntityID,
        Option<&OwnerPlayerUID>,
    )>,
) {
    for AbilityInvokeEvent(invoke) in events.read() {
        let now_entity_id;
        let mut now_target_id = 0;
        if (invoke.entity_id >> 22) < ProtEntityType::ProtEntityMax as u32 {
            let Some((mut _fight_props, _, _)) = entities
                .iter_mut()
                .find(|(_, id, _)| id.0 == invoke.entity_id)
            else {
                tracing::trace!(
                    "AbilityInvokeEvent invoke.entity_id {} not found",
                    invoke.entity_id
                );
                continue;
            };
            now_entity_id = invoke.entity_id;
            let mut _target_fight_props = _fight_props;
            match invoke.head {
                None => {}
                Some(head) => {
                    if head.target_id != 0 {
                        match entities
                            .iter_mut()
                            .find(|(_, id, _)| id.0 == head.target_id)
                        {
                            None => {
                                tracing::trace!(
                                    "AbilityInvokeEvent head.target_id {} not found",
                                    head.target_id
                                );
                                now_target_id = 0;
                            }
                            Some((temp_fight_props, _, _)) => {
                                now_target_id = head.target_id;
                                _target_fight_props = temp_fight_props;
                            }
                        }
                    }
                    if head.local_id != 0 {
                        tracing::trace!(
                            "AbilityInvokeEvent head.local_id:{}",
                            head.local_id
                        );
                        continue;
                    }
                }
            }
            tracing::trace!(
                "AbilityInvokeEvent now_entity_id:{},now_target_id:{}",
                now_entity_id,
                now_target_id
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

fn handle_server_invoke() {}
