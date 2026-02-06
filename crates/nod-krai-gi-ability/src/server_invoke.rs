use bevy_ecs::prelude::*;
use nod_krai_gi_entity::common::{
    EntityById, InstancedAbilities, InstancedAbility, InstancedModifiers, ProtocolEntityID,
};

use crate::enums::{
    AbilityConfigIdxEnum, AbilityModifierConfigIdxEnum, ConfigAbilitySubContainerType,
};

use nod_krai_gi_event::ability::*;

pub fn server_invoke(
    index: Res<EntityById>,
    mut events: MessageReader<ServerInvokeEvent>,
    entities: Query<(&InstancedAbilities, &InstancedModifiers, &ProtocolEntityID)>,
    mut execute_action_events: MessageWriter<ExecuteActionEvent>,
    mut execute_mixin_events: MessageWriter<ExecuteMixinEvent>,
) {
    for ServerInvokeEvent(invoke) in events.read() {
        if index.0.get(&invoke.entity_id).is_none() {
            tracing::debug!(
                "AbilityInvokeEvent invoke.entity_id {} not found",
                invoke.entity_id
            );
            continue;
        }

        let head = invoke.head.unwrap();

        tracing::debug!("\n[server_invoke] invoke: {:?}\n", invoke);

        let entity = match index.0.get(&invoke.entity_id) {
            Some(e) => *e,
            None => {
                tracing::debug!(
                    "[server_invoke] Entity not found by server invoke: {}",
                    invoke.entity_id
                );
                continue;
            }
        };

        let target_entity = if head.target_id != 0 {
            index.0.get(&head.target_id).copied()
        } else {
            Some(entity)
        };

        let mut ability: Option<(u32, Entity, InstancedAbility)> = None;

        if ability.is_none() && head.instanced_modifier_id != 0 {
            if let Ok((_, instanced_modifiers, _)) = entities.get(entity) {
                if let Some(modifier) = instanced_modifiers.0.get(&head.instanced_modifier_id) {
                    if let Some(idx) = modifier.ability_index {
                        let entity_to_get = modifier.target_entity.unwrap_or(entity);
                        if let Ok((target_abilities, _, _)) = entities.get(entity_to_get) {
                            if let Some(item) = target_abilities.list.get(idx as usize) {
                                ability = Some((idx, entity_to_get, item.clone()));
                            }
                        }
                    }
                }
            }
        }

        if ability.is_none() && head.instanced_ability_id != 0 {
            if let Ok((instanced_abilities, _, _)) = entities.get(entity) {
                match instanced_abilities.find_by_instanced_ability_id(head.instanced_ability_id) {
                    None => {}
                    Some((index, item)) => {
                        ability = Some((index, entity, item.clone()));
                    }
                }
            }
        }

        let ability_data = match ability
            .as_ref()
            .and_then(|(_, _, a)| a.ability_data.as_ref())
        {
            Some(data) => data,
            None => {
                tracing::debug!(
                                "[server_invoke] Ability not found: instanced_ability_id: {} instanced_modifier_id: {} invoke.entity_id: {}",
                                head.instanced_ability_id,
                                head.instanced_modifier_id,
                                invoke.entity_id
                            );
                continue;
            }
        };

        tracing::debug!(
            "[server_invoke] ability_name: {}",
            ability_data.ability_name
        );

        let local_id_info = parse_local_id(head.local_id);
        tracing::debug!(
            "[server_invoke] AbilityInvokeEvent local_id {} local_id_info {:?}",
            head.local_id,
            local_id_info
        );

        match local_id_info.type_tag {
            ConfigAbilitySubContainerType::Action => {
                let actions = match AbilityConfigIdxEnum::try_from(local_id_info.config_idx) {
                    Ok(AbilityConfigIdxEnum::OnAdded) => &ability_data.on_added,
                    Ok(AbilityConfigIdxEnum::OnRemoved) => &ability_data.on_removed,
                    Ok(AbilityConfigIdxEnum::OnAbilityStart) => &ability_data.on_ability_start,
                    Ok(AbilityConfigIdxEnum::OnKill) => &ability_data.on_kill,
                    Ok(AbilityConfigIdxEnum::OnFieldEnter) => &ability_data.on_field_enter,
                    Ok(AbilityConfigIdxEnum::OnFieldExit) => &ability_data.on_field_exit,
                    Ok(AbilityConfigIdxEnum::OnAttach) => &ability_data.on_attach,
                    Ok(AbilityConfigIdxEnum::OnDetach) => &ability_data.on_detach,
                    Ok(AbilityConfigIdxEnum::OnAvatarIn) => &ability_data.on_avatar_in,
                    Ok(AbilityConfigIdxEnum::OnAvatarOut) => &ability_data.on_avatar_out,
                    Ok(AbilityConfigIdxEnum::OnTriggerAvatarRay) => {
                        &ability_data.on_trigger_avatar_ray
                    }
                    Ok(AbilityConfigIdxEnum::OnVehicleIn) => &ability_data.on_vehicle_in,
                    Ok(AbilityConfigIdxEnum::OnVehicleOut) => &ability_data.on_vehicle_out,
                    Ok(AbilityConfigIdxEnum::Unknown(_)) | Err(_) => {
                        tracing::debug!(
                            "[server_invoke] Unknown config_idx {} for Action",
                            local_id_info.config_idx
                        );
                        continue;
                    }
                };

                let mut collect_actions = Vec::new();
                for action in actions.iter() {
                    collect_actions.push(action);
                    if !action.actions.is_empty() {
                        collect_actions.extend(action.actions.iter());
                    } else {
                        if !action.success_actions.is_empty() {
                            collect_actions.extend(action.success_actions.iter());
                        }
                        if !action.fail_actions.is_empty() {
                            collect_actions.extend(action.fail_actions.iter());
                        }
                    }
                }

                let action_idx = local_id_info.action_idx as usize;
                if action_idx <= collect_actions.len() {
                    let action = collect_actions[action_idx - 1];
                    tracing::debug!(
                                    "[server_invoke] Found Action: config_idx={}, action_idx={}, action_type={:?}",
                                    local_id_info.config_idx,
                                    local_id_info.action_idx,
                                    action.type_name
                                );
                    if let Some((ability_index, ability_entity, _)) = ability {
                        execute_action_events.write(ExecuteActionEvent(
                            ability_index,
                            ability_entity,
                            action.clone(),
                            invoke.ability_data.clone(),
                            target_entity,
                        ));
                    }
                } else {
                    tracing::debug!(
                                    "[server_invoke] Action index {} out of bounds for config_idx {} collect_actions len {}",
                                    action_idx,
                                    local_id_info.config_idx,
                                    collect_actions.len()
                                );
                }
            }
            ConfigAbilitySubContainerType::Mixin => {
                let mixin_idx = local_id_info.mixin_idx as usize;
                if mixin_idx < ability_data.ability_mixins.len() {
                    let mixin = &ability_data.ability_mixins[mixin_idx];
                    tracing::debug!(
                        "Found Mixin: mixin_idx={}, mixin_type={:?}",
                        local_id_info.mixin_idx,
                        mixin.type_name
                    );
                    if let Some((ability_index, ability_entity, _)) = ability {
                        execute_mixin_events.write(ExecuteMixinEvent(
                            ability_index,
                            ability_entity,
                            mixin.clone(),
                            invoke.ability_data.clone(),
                            target_entity,
                        ));
                    }
                } else {
                    tracing::debug!(
                                    "[server_invoke] Mixin index {} out of bounds for config_idx {} mixins len {}",
                                    mixin_idx,
                                    local_id_info.config_idx,
                                    ability_data.ability_mixins.len()
                                );
                }
            }
            ConfigAbilitySubContainerType::ModifierAction => {
                let modifier_idx = local_id_info.modifier_idx as usize;
                let modifiers: Vec<_> = ability_data.modifiers.iter().collect();

                if modifier_idx < modifiers.len() {
                    let (_, modifier) = modifiers[modifier_idx];

                    let actions =
                        match AbilityModifierConfigIdxEnum::try_from(local_id_info.config_idx) {
                            Ok(AbilityModifierConfigIdxEnum::OnAdded) => &modifier.on_added,
                            Ok(AbilityModifierConfigIdxEnum::OnRemoved) => &modifier.on_removed,
                            Ok(AbilityModifierConfigIdxEnum::OnBeingHit) => &modifier.on_being_hit,
                            Ok(AbilityModifierConfigIdxEnum::OnAttackLanded) => {
                                &modifier.on_attack_landed
                            }
                            Ok(AbilityModifierConfigIdxEnum::OnHittingOther) => {
                                &modifier.on_hitting_other
                            }
                            Ok(AbilityModifierConfigIdxEnum::OnThinkInterval) => {
                                &modifier.on_think_interval
                            }
                            Ok(AbilityModifierConfigIdxEnum::OnKill) => &modifier.on_kill,
                            Ok(AbilityModifierConfigIdxEnum::OnCrash) => &modifier.on_crash,
                            Ok(AbilityModifierConfigIdxEnum::OnAvatarIn) => &modifier.on_avatar_in,
                            Ok(AbilityModifierConfigIdxEnum::OnAvatarOut) => {
                                &modifier.on_avatar_out
                            }
                            Ok(AbilityModifierConfigIdxEnum::OnReconnect) => &modifier.on_reconnect,
                            Ok(AbilityModifierConfigIdxEnum::OnChangeAuthority) => {
                                &modifier.on_change_authority
                            }
                            Ok(AbilityModifierConfigIdxEnum::OnVehicleIn) => {
                                &modifier.on_vehicle_in
                            }
                            Ok(AbilityModifierConfigIdxEnum::OnVehicleOut) => {
                                &modifier.on_vehicle_out
                            }
                            Ok(AbilityModifierConfigIdxEnum::OnZoneEnter) => {
                                &modifier.on_zone_enter
                            }
                            Ok(AbilityModifierConfigIdxEnum::OnZoneExit) => &modifier.on_zone_exit,
                            Ok(AbilityModifierConfigIdxEnum::OnHeal) => &modifier.on_heal,
                            Ok(AbilityModifierConfigIdxEnum::OnBeingHealed) => {
                                &modifier.on_being_healed
                            }
                            Ok(AbilityModifierConfigIdxEnum::Unknown(_)) | Err(_) => {
                                tracing::debug!(
                                    "[server_invoke] Unknown config_idx {} for ModifierAction",
                                    local_id_info.config_idx
                                );
                                continue;
                            }
                        };

                    let mut collect_actions = Vec::new();
                    for action in actions.iter() {
                        collect_actions.push(action);
                        if !action.actions.is_empty() {
                            collect_actions.extend(action.actions.iter());
                        } else {
                            if !action.success_actions.is_empty() {
                                collect_actions.extend(action.success_actions.iter());
                            }
                            if !action.fail_actions.is_empty() {
                                collect_actions.extend(action.fail_actions.iter());
                            }
                        }
                    }

                    let action_idx = local_id_info.action_idx as usize;
                    if action_idx <= collect_actions.len() {
                        let action = collect_actions[action_idx - 1];
                        tracing::debug!(
                                        "[server_invoke] Found ModifierAction: modifier_idx={}, modifier_name={}, config_idx={}, action_idx={}, action_type={:?}",
                                        local_id_info.modifier_idx,
                                        modifier.modifier_name,
                                        local_id_info.config_idx,
                                        local_id_info.action_idx,
                                        action.type_name
                                    );
                        if let Some((ability_index, ability_entity, _)) = ability {
                            execute_action_events.write(ExecuteActionEvent(
                                ability_index,
                                ability_entity,
                                action.clone(),
                                invoke.ability_data.clone(),
                                target_entity,
                            ));
                        }
                    } else {
                        tracing::debug!(
                                        "[server_invoke] Action index {} out of bounds for modifier {} config_idx {} collect_actions len {}",
                                        action_idx,
                                        modifier.modifier_name,
                                        local_id_info.config_idx,
                                        collect_actions.len()
                                    );
                    }
                } else {
                    tracing::debug!(
                                    "[server_invoke] Modifier index {} out of bounds for config_idx {} modifiers len {}",
                                    modifier_idx,
                                    local_id_info.config_idx,
                                    modifiers.len()
                                );
                }
            }
            ConfigAbilitySubContainerType::ModifierMixin => {
                let modifier_idx = local_id_info.modifier_idx as usize;
                let modifiers: Vec<_> = ability_data.modifiers.iter().collect();

                if modifier_idx < modifiers.len() {
                    let (_, modifier) = modifiers[modifier_idx];
                    let mixin_idx = local_id_info.mixin_idx as usize;

                    if mixin_idx < modifier.modifier_mixins.len() {
                        let mixin = &modifier.modifier_mixins[mixin_idx];
                        tracing::debug!(
                                        "[server_invoke] Found ModifierMixin: modifier_idx={}, modifier_name={}, mixin_idx={}, mixin_type={:?}",
                                        local_id_info.modifier_idx,
                                        modifier.modifier_name,
                                        local_id_info.mixin_idx,
                                        mixin.type_name
                                    );
                        if let Some((ability_index, ability_entity, _)) = ability {
                            execute_mixin_events.write(ExecuteMixinEvent(
                                ability_index,
                                ability_entity,
                                mixin.clone(),
                                invoke.ability_data.clone(),
                                target_entity,
                            ));
                        }
                    } else {
                        tracing::debug!(
                                        "[server_invoke] Mixin index {} out of bounds for modifier {} config_idx {} mixins len {}",
                                        mixin_idx,
                                        modifier.modifier_name,
                                        local_id_info.config_idx,
                                        modifier.modifier_mixins.len()
                                    );
                    }
                } else {
                    tracing::debug!(
                                    "[server_invoke] Modifier index {} out of bounds for config_idx {} modifiers len {}",
                                    modifier_idx,
                                    local_id_info.config_idx,
                                    modifiers.len()
                                );
                }
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct LocalIdInfo {
    type_tag: ConfigAbilitySubContainerType,
    action_idx: i32,
    config_idx: i32,
    mixin_idx: i32,
    modifier_idx: i32,
}

fn parse_local_id(id: i32) -> LocalIdInfo {
    let type_tag = id & 0b111; // 低 3 bit

    match type_tag {
        1 => {
            let config_idx = (id >> 3) & 0x3F;
            let action_idx = (id >> 9) & 0x3F;

            LocalIdInfo {
                type_tag: ConfigAbilitySubContainerType::Action,
                action_idx,
                config_idx,
                mixin_idx: 0,
                modifier_idx: 0,
            }
        }

        2 => {
            let mixin_idx = (id >> 3) & 0x3F;
            let config_idx = (id >> 9) & 0x3F;
            let action_idx = (id >> 15) & 0x3F;

            LocalIdInfo {
                type_tag: ConfigAbilitySubContainerType::Mixin,
                action_idx,
                config_idx,
                mixin_idx,
                modifier_idx: 0,
            }
        }

        3 => {
            let modifier_idx = (id >> 3) & 0x3F;
            let config_idx = (id >> 9) & 0x3F;
            let action_idx = (id >> 15) & 0x3F;

            LocalIdInfo {
                type_tag: ConfigAbilitySubContainerType::ModifierAction,
                action_idx,
                config_idx,
                mixin_idx: 0,
                modifier_idx,
            }
        }

        4 => {
            let modifier_idx = (id >> 3) & 0x3F;
            let mixin_idx = (id >> 9) & 0x3F;
            let config_idx = (id >> 15) & 0x3F;
            let action_idx = (id >> 21) & 0x3F;

            LocalIdInfo {
                type_tag: ConfigAbilitySubContainerType::ModifierMixin,
                action_idx,
                config_idx,
                mixin_idx,
                modifier_idx,
            }
        }

        _ => panic!("Invalid type tag {}", type_tag),
    }
}
