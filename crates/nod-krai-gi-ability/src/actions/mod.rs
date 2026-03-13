pub(crate) mod ability_action_add_global_value;
pub(crate) mod ability_action_add_hp_debts;
pub(crate) mod ability_action_apply_modifier;
pub(crate) mod ability_action_attach_modifier;
pub(crate) mod ability_action_avatar_skill_start;
pub(crate) mod ability_action_clear_global_value;
pub(crate) mod ability_action_copy_global_value;
pub(crate) mod ability_action_get_hp_paid_debts;
pub(crate) mod ability_action_heal_hp;
pub(crate) mod ability_action_kill_self;
pub(crate) mod ability_action_lose_hp;
pub(crate) mod ability_action_modify_avatar_skill_cd;
pub(crate) mod ability_action_reduce_hp_debts;
pub(crate) mod ability_action_remove_modifier;
pub(crate) mod ability_action_remove_unique_modifier;
pub(crate) mod ability_action_set_global_value;
pub(crate) mod ability_action_set_global_value_to_override_map;
pub(crate) mod ability_action_set_override_map_value;
pub(crate) mod ability_action_set_random_override_map_value;
pub(crate) mod ability_action_trigger_ability;

use crate::util::resolve_target_entity;
use bevy_ecs::prelude::*;
use nod_krai_gi_data::GAME_SERVER_CONFIG;
use nod_krai_gi_entity::avatar::{CurrentPlayerAvatarMarker, CurrentTeam};
use nod_krai_gi_entity::common::{EntityById, OwnerProtocolEntityID, ProtocolEntityID};
use nod_krai_gi_entity::team::TeamEntityMarker;
use nod_krai_gi_event::ability::*;

pub fn execute_action_system_ability(
    index: Res<EntityById>,
    mut events: MessageReader<ExecuteActionEvent>,
    mut trigger_ability_events: MessageWriter<AbilityActionTriggerAbilityEvent>,
    entity_query: Query<(
        Entity,
        &ProtocolEntityID,
        Option<&OwnerProtocolEntityID>,
        Option<&CurrentPlayerAvatarMarker>,
        Option<&CurrentTeam>,
        Option<&TeamEntityMarker>,
    )>,
) {
    for ExecuteActionEvent(ability_index, ability_entity, action, ability_data, target_entity) in
        events.read()
    {
        if GAME_SERVER_CONFIG.plugin.ability_log {
            tracing::debug!("[execute_action_system] action type: {}", action.type_name);
        }
        match action.type_name.as_str() {
            "TriggerAbility" => {
                let target = resolve_target_entity(
                    action.target,
                    *ability_entity,
                    *target_entity,
                    &index,
                    &entity_query,
                );
                let Some(target) = target else { continue };
                trigger_ability_events.write(AbilityActionTriggerAbilityEvent(
                    *ability_index,
                    *ability_entity,
                    action.clone(),
                    ability_data.clone(),
                    target,
                ));
            }
            _ => {}
        }
    }
}

pub fn execute_action_system_modifier(
    index: Res<EntityById>,
    mut events: MessageReader<ExecuteActionEvent>,
    mut apply_modifier_events: MessageWriter<AbilityActionApplyModifierEvent>,
    mut attach_modifier_events: MessageWriter<AbilityActionAttachModifierEvent>,
    mut remove_modifier_events: MessageWriter<AbilityActionRemoveModifierEvent>,
    mut remove_unique_modifier_events: MessageWriter<AbilityActionRemoveUniqueModifierEvent>,
    entity_query: Query<(
        Entity,
        &ProtocolEntityID,
        Option<&OwnerProtocolEntityID>,
        Option<&CurrentPlayerAvatarMarker>,
        Option<&CurrentTeam>,
        Option<&TeamEntityMarker>,
    )>,
) {
    for ExecuteActionEvent(ability_index, ability_entity, action, ability_data, target_entity) in
        events.read()
    {
        match action.type_name.as_str() {
            "ApplyModifier" => {
                let target = resolve_target_entity(
                    action.target,
                    *ability_entity,
                    *target_entity,
                    &index,
                    &entity_query,
                );
                let Some(target) = target else { continue };
                apply_modifier_events.write(AbilityActionApplyModifierEvent(
                    *ability_index,
                    *ability_entity,
                    action.clone(),
                    ability_data.clone(),
                    target,
                ));
            }
            "AttachModifier" => {
                let target = resolve_target_entity(
                    action.target,
                    *ability_entity,
                    *target_entity,
                    &index,
                    &entity_query,
                );
                let Some(target) = target else { continue };
                attach_modifier_events.write(AbilityActionAttachModifierEvent(
                    *ability_index,
                    *ability_entity,
                    action.clone(),
                    ability_data.clone(),
                    target,
                ));
            }
            "RemoveModifier" => {
                let target = resolve_target_entity(
                    action.target,
                    *ability_entity,
                    *target_entity,
                    &index,
                    &entity_query,
                );
                let Some(target) = target else { continue };
                remove_modifier_events.write(AbilityActionRemoveModifierEvent(
                    *ability_index,
                    *ability_entity,
                    action.clone(),
                    ability_data.clone(),
                    target,
                ));
            }
            "RemoveUniqueModifier" => {
                let target = resolve_target_entity(
                    action.target,
                    *ability_entity,
                    *target_entity,
                    &index,
                    &entity_query,
                );
                let Some(target) = target else { continue };
                remove_unique_modifier_events.write(AbilityActionRemoveUniqueModifierEvent(
                    *ability_index,
                    *ability_entity,
                    action.clone(),
                    ability_data.clone(),
                    target,
                ));
            }
            _ => {}
        }
    }
}

pub fn execute_action_system_override_map(
    index: Res<EntityById>,
    mut events: MessageReader<ExecuteActionEvent>,
    mut set_global_value_to_override_map_events: MessageWriter<
        AbilityActionSetGlobalValueToOverrideMapEvent,
    >,
    mut set_override_map_value_events: MessageWriter<AbilityActionSetOverrideMapValueEvent>,
    mut set_random_override_map_value_events: MessageWriter<
        AbilityActionSetRandomOverrideMapValueEvent,
    >,
    entity_query: Query<(
        Entity,
        &ProtocolEntityID,
        Option<&OwnerProtocolEntityID>,
        Option<&CurrentPlayerAvatarMarker>,
        Option<&CurrentTeam>,
        Option<&TeamEntityMarker>,
    )>,
) {
    for ExecuteActionEvent(ability_index, ability_entity, action, ability_data, target_entity) in
        events.read()
    {
        match action.type_name.as_str() {
            "SetGlobalValueToOverrideMap" => {
                let target = resolve_target_entity(
                    action.target,
                    *ability_entity,
                    *target_entity,
                    &index,
                    &entity_query,
                );
                let Some(target) = target else { continue };
                set_global_value_to_override_map_events.write(
                    AbilityActionSetGlobalValueToOverrideMapEvent(
                        *ability_index,
                        *ability_entity,
                        action.clone(),
                        ability_data.clone(),
                        target,
                    ),
                );
            }
            "SetOverrideMapValue" => {
                let target = resolve_target_entity(
                    action.target,
                    *ability_entity,
                    *target_entity,
                    &index,
                    &entity_query,
                );
                let Some(target) = target else { continue };
                set_override_map_value_events.write(AbilityActionSetOverrideMapValueEvent(
                    *ability_index,
                    *ability_entity,
                    action.clone(),
                    ability_data.clone(),
                    target,
                ));
            }
            "SetRandomOverrideMapValue" => {
                let target = resolve_target_entity(
                    action.target,
                    *ability_entity,
                    *target_entity,
                    &index,
                    &entity_query,
                );
                let Some(target) = target else { continue };
                set_random_override_map_value_events.write(
                    AbilityActionSetRandomOverrideMapValueEvent(
                        *ability_index,
                        *ability_entity,
                        action.clone(),
                        ability_data.clone(),
                        target,
                    ),
                );
            }
            _ => {}
        }
    }
}

pub fn execute_action_system_global_value(
    index: Res<EntityById>,
    mut events: MessageReader<ExecuteActionEvent>,
    mut set_global_value_events: MessageWriter<AbilityActionSetGlobalValueEvent>,
    mut add_global_value_events: MessageWriter<AbilityActionAddGlobalValueEvent>,
    mut copy_global_value_events: MessageWriter<AbilityActionCopyGlobalValueEvent>,
    mut clear_global_value_events: MessageWriter<AbilityActionClearGlobalValueEvent>,
    entity_query: Query<(
        Entity,
        &ProtocolEntityID,
        Option<&OwnerProtocolEntityID>,
        Option<&CurrentPlayerAvatarMarker>,
        Option<&CurrentTeam>,
        Option<&TeamEntityMarker>,
    )>,
) {
    for ExecuteActionEvent(ability_index, ability_entity, action, ability_data, target_entity) in
        events.read()
    {
        match action.type_name.as_str() {
            "SetGlobalValue" => {
                let target = resolve_target_entity(
                    action.target,
                    *ability_entity,
                    *target_entity,
                    &index,
                    &entity_query,
                );
                let Some(target) = target else { continue };
                set_global_value_events.write(AbilityActionSetGlobalValueEvent(
                    *ability_index,
                    *ability_entity,
                    action.clone(),
                    ability_data.clone(),
                    target,
                ));
            }
            "AddGlobalValue" => {
                let target = resolve_target_entity(
                    action.target,
                    *ability_entity,
                    *target_entity,
                    &index,
                    &entity_query,
                );
                let Some(target) = target else { continue };
                add_global_value_events.write(AbilityActionAddGlobalValueEvent(
                    *ability_index,
                    *ability_entity,
                    action.clone(),
                    ability_data.clone(),
                    target,
                ));
            }
            "CopyGlobalValue" => {
                let target = resolve_target_entity(
                    action.target,
                    *ability_entity,
                    *target_entity,
                    &index,
                    &entity_query,
                );
                let Some(target) = target else { continue };
                copy_global_value_events.write(AbilityActionCopyGlobalValueEvent(
                    *ability_index,
                    *ability_entity,
                    action.clone(),
                    ability_data.clone(),
                    target,
                ));
            }
            "ClearGlobalValue" => {
                let target = resolve_target_entity(
                    action.target,
                    *ability_entity,
                    *target_entity,
                    &index,
                    &entity_query,
                );
                let Some(target) = target else { continue };
                clear_global_value_events.write(AbilityActionClearGlobalValueEvent(
                    *ability_index,
                    *ability_entity,
                    action.clone(),
                    ability_data.clone(),
                    target,
                ));
            }
            _ => {}
        }
    }
}

pub fn execute_action_system_hp(
    index: Res<EntityById>,
    mut events: MessageReader<ExecuteActionEvent>,
    mut heal_hp_events: MessageWriter<AbilityActionHealHPEvent>,
    mut lose_hp_events: MessageWriter<AbilityActionLoseHPEvent>,
    mut kill_self_events: MessageWriter<AbilityActionKillSelfEvent>,
    entity_query: Query<(
        Entity,
        &ProtocolEntityID,
        Option<&OwnerProtocolEntityID>,
        Option<&CurrentPlayerAvatarMarker>,
        Option<&CurrentTeam>,
        Option<&TeamEntityMarker>,
    )>,
) {
    for ExecuteActionEvent(ability_index, ability_entity, action, ability_data, target_entity) in
        events.read()
    {
        match action.type_name.as_str() {
            "HealHP" => {
                let targets = crate::util::resolve_target_entities(
                    action.target,
                    *ability_entity,
                    *target_entity,
                    &index,
                    &entity_query,
                );
                if targets.is_empty() {
                    continue;
                };
                heal_hp_events.write(AbilityActionHealHPEvent(
                    *ability_index,
                    *ability_entity,
                    action.clone(),
                    ability_data.clone(),
                    targets,
                ));
            }
            "LoseHP" => {
                let targets = crate::util::resolve_target_entities(
                    action.target,
                    *ability_entity,
                    *target_entity,
                    &index,
                    &entity_query,
                );
                if targets.is_empty() {
                    continue;
                };
                lose_hp_events.write(AbilityActionLoseHPEvent(
                    *ability_index,
                    *ability_entity,
                    action.clone(),
                    ability_data.clone(),
                    targets,
                ));
            }
            "KillSelf" => {
                let targets = crate::util::resolve_target_entities(
                    action.target,
                    *ability_entity,
                    *target_entity,
                    &index,
                    &entity_query,
                );
                if targets.is_empty() {
                    continue;
                };
                kill_self_events.write(AbilityActionKillSelfEvent(
                    *ability_index,
                    *ability_entity,
                    action.clone(),
                    ability_data.clone(),
                    targets,
                ));
            }
            _ => {}
        }
    }
}

pub fn execute_action_system_debts(
    index: Res<EntityById>,
    mut events: MessageReader<ExecuteActionEvent>,
    mut get_hp_paid_debts_events: MessageWriter<AbilityActionGetHPPaidDebtsEvent>,
    mut add_hp_debts_events: MessageWriter<AbilityActionAddHPDebtsEvent>,
    mut reduce_hp_debts_events: MessageWriter<AbilityActionReduceHPDebtsEvent>,
    entity_query: Query<(
        Entity,
        &ProtocolEntityID,
        Option<&OwnerProtocolEntityID>,
        Option<&CurrentPlayerAvatarMarker>,
        Option<&CurrentTeam>,
        Option<&TeamEntityMarker>,
    )>,
) {
    for ExecuteActionEvent(ability_index, ability_entity, action, ability_data, target_entity) in
        events.read()
    {
        match action.type_name.as_str() {
            "GetHPPaidDebts" => {
                let targets = crate::util::resolve_target_entities(
                    action.target,
                    *ability_entity,
                    *target_entity,
                    &index,
                    &entity_query,
                );
                if targets.is_empty() {
                    continue;
                };
                get_hp_paid_debts_events.write(AbilityActionGetHPPaidDebtsEvent(
                    *ability_index,
                    *ability_entity,
                    action.clone(),
                    ability_data.clone(),
                    targets,
                ));
            }
            "AddHPDebts" => {
                let targets = crate::util::resolve_target_entities(
                    action.target,
                    *ability_entity,
                    *target_entity,
                    &index,
                    &entity_query,
                );
                if targets.is_empty() {
                    continue;
                };
                add_hp_debts_events.write(AbilityActionAddHPDebtsEvent(
                    *ability_index,
                    *ability_entity,
                    action.clone(),
                    ability_data.clone(),
                    targets,
                ));
            }
            "ReduceHPDebts" => {
                let targets = crate::util::resolve_target_entities(
                    action.target,
                    *ability_entity,
                    *target_entity,
                    &index,
                    &entity_query,
                );
                if targets.is_empty() {
                    continue;
                };
                reduce_hp_debts_events.write(AbilityActionReduceHPDebtsEvent(
                    *ability_index,
                    *ability_entity,
                    action.clone(),
                    ability_data.clone(),
                    targets,
                ));
            }
            _ => {}
        }
    }
}

pub fn execute_action_system_misc(
    index: Res<EntityById>,
    mut events: MessageReader<ExecuteActionEvent>,
    mut modify_avatar_skill_cd_events: MessageWriter<AbilityActionModifyAvatarSkillCDEvent>,
    mut avatar_skill_start_events: MessageWriter<AbilityActionAvatarSkillStartEvent>,
    entity_query: Query<(
        Entity,
        &ProtocolEntityID,
        Option<&OwnerProtocolEntityID>,
        Option<&CurrentPlayerAvatarMarker>,
        Option<&CurrentTeam>,
        Option<&TeamEntityMarker>,
    )>,
) {
    for ExecuteActionEvent(ability_index, ability_entity, action, ability_data, target_entity) in
        events.read()
    {
        match action.type_name.as_str() {
            "ModifyAvatarSkillCD" => {
                let targets = crate::util::resolve_target_entities(
                    action.target,
                    *ability_entity,
                    *target_entity,
                    &index,
                    &entity_query,
                );
                if targets.is_empty() {
                    continue;
                };
                modify_avatar_skill_cd_events.write(AbilityActionModifyAvatarSkillCDEvent(
                    *ability_index,
                    *ability_entity,
                    action.clone(),
                    ability_data.clone(),
                    targets,
                ));
            }
            "AvatarSkillStart" => {
                let targets = crate::util::resolve_target_entities(
                    action.target,
                    *ability_entity,
                    *target_entity,
                    &index,
                    &entity_query,
                );
                if targets.is_empty() {
                    continue;
                };
                avatar_skill_start_events.write(AbilityActionAvatarSkillStartEvent(
                    *ability_index,
                    *ability_entity,
                    action.clone(),
                    ability_data.clone(),
                    targets,
                ));
            }
            _ => {}
        }
    }
}
