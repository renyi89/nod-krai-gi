use crate::util::eval_option;
use bevy_ecs::prelude::*;

use nod_krai_gi_entity::common::{
    ConfigId, GroupId, InstancedAbilities, InstancedModifiers, OwnerProtocolEntityID,
    ProtocolEntityID,
};
use nod_krai_gi_event::ability::ExecuteActionEvent;
use nod_krai_gi_event::lua::OnClientExecuteReqEvent;

pub fn ability_action_execute_gadget_lua_event(
    mut events: MessageReader<ExecuteActionEvent>,
    entities_query: Query<(
        Option<&OwnerProtocolEntityID>,
        &InstancedAbilities,
        &InstancedModifiers,
    )>,
    target_query: Query<(Option<&GroupId>, Option<&ConfigId>, &ProtocolEntityID)>,
    mut execute_gadget_lua_events: MessageWriter<OnClientExecuteReqEvent>,
) {
    for ExecuteActionEvent(ability_index, ability_entity, action, _ability_data, target_entity) in
        events.read()
    {
        if action.type_name != "ExecuteGadgetLua" {
            continue;
        }

        let Ok((owner_protocol_entity_id, abilities, _)) = entities_query.get(*ability_entity)
        else {
            tracing::debug!(target: "ability",
                "[ability_action_execute_gadget_lua_event] Failed to get entity components for {}",
                ability_entity
            );
            continue;
        };

        let Some(ability) = abilities.list.get(*ability_index as usize).cloned() else {
            tracing::debug!(target: "ability",
                "[ability_action_execute_gadget_lua_event] Ability not found for index: {} entity: {}",
                ability_index,
                ability_entity
            );
            continue;
        };

        // Get the lua script name from gadget mapping
        let Some(lua_name) =
            nod_krai_gi_data::custom::GadgetMapping::get_gadget_lua_name(action.gadget_id)
        else {
            tracing::debug!(target: "ability",
                "[ability_action_execute_gadget_lua_event] No lua mapping found for gadget_id {} in ExecuteGadgetLua action",
                action.gadget_id
            );
            continue;
        };

        // Get additional parameters if needed
        let param1 = eval_option(&ability, None, &action.param1, 0.0);
        let param2 = eval_option(&ability, None, &action.param2, 0.0);
        let param3 = eval_option(&ability, None, &action.param3, 0.0);

        // Get group_id and config_id from target_entity (gadget)
        let (group_id, config_id, entity_id) = if let Some(target_entity) = target_entity {
            if let Ok((Some(group_id_comp), Some(config_id_comp), entity_id_comp)) =
                target_query.get(*target_entity)
            {
                (group_id_comp.0, config_id_comp.0, entity_id_comp.0)
            } else {
                tracing::debug!(target: "ability",
                    "[ability_action_execute_gadget_lua_event] Target entity {} missing GroupId or ConfigId component",
                    target_entity
                );
                continue;
            }
        } else {
            tracing::debug!(target: "ability",
                "[ability_action_execute_gadget_lua_event] No target entity for ExecuteGadgetLua action"
            );
            continue;
        };

        // Create LuaContext
        let lua_context = nod_krai_gi_data::scene::LuaContext {
            scene_id: 0,
            group_id,
            config_id,
            source_entity_id: entity_id,
            target_entity_id: entity_id,
            uid: owner_protocol_entity_id.and_then(|id| id.0).unwrap_or(0),
        };

        // Send event to script system to execute the gadget lua
        execute_gadget_lua_events.write(OnClientExecuteReqEvent {
            lua_name: lua_name.clone(),
            param1: param1 as u32,
            param2: param2 as u32,
            param3: param3 as u32,
            lua_context,
        });

        tracing::debug!(target: "ability",
            "[ability_action_execute_gadget_lua_event] Sent ExecuteGadgetLuaEvent for script {}",
            lua_name
        );
    }
}
