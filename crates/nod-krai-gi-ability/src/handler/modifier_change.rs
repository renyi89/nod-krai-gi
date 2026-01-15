use bevy_ecs::prelude::*;
use nod_krai_gi_data::ability::get_ability_data;
use nod_krai_gi_entity::common::{
    AbilityModifierController, EntityById, InstancedAbilities, InstancedModifiers,
};
use nod_krai_gi_proto::{AbilityMetaModifierChange, ModifierAction};

use crate::ModifierChangeEvent;
use crate::util::get_ability_name;

pub fn handle_modifier_change(
    index: Res<EntityById>,
    mut events: MessageReader<ModifierChangeEvent>,
    mut entities: Query<(&mut InstancedAbilities, &mut InstancedModifiers)>,
) {
    for ModifierChangeEvent(invoke, version) in events.read() {
        let entity = match index.0.get(&invoke.entity_id) {
            Some(e) => *e,
            None => {
                tracing::debug!("[ModifierChange] Entity {} not found", invoke.entity_id);
                continue;
            }
        };

        match invoke.head {
            None => {
                tracing::debug!("[ModifierChange] AbilityInvokeEntry head is missing");
            }
            Some(head) => {
                if head.instanced_modifier_id == 0 || head.instanced_modifier_id > 2000 {
                    continue;
                }

                match nod_krai_gi_proto::dy_parser::decode_from_vec_by_name_version::<
                    AbilityMetaModifierChange,
                >(version, "AbilityMetaModifierChange", &*invoke.ability_data)
                {
                    None => {
                        tracing::debug!(
                            "[ModifierChange] Failed to decode AbilityMetaModifierChange"
                        );
                    }
                    Some(mod_change) => {
                        let instanced_ability_id = head.instanced_ability_id;
                        let instanced_modifier_id = head.instanced_modifier_id;
                        let modifier_local_id = mod_change.modifier_local_id as usize;
                        let target_id = head.target_id;

                        match mod_change.action() {
                            ModifierAction::Added => {
                                tracing::debug!(
                                    "[ModifierChange] invoke.entity_id : {}",
                                    invoke.entity_id
                                );
                                tracing::debug!("[ModifierChange] instanced_ability_id : {} instanced_modifier_id : {} modifier_local_id : {} target_id : {}", instanced_ability_id,instanced_modifier_id,modifier_local_id,target_id);

                                let mut instanced_ability_data = None;

                                let mut ability_index = None;
                                let mut target_entity_ref = None;

                                if target_id != 0 {
                                    if let Some(target_entity) = index.0.get(&target_id) {
                                        if let Ok((target_abilities, _)) =
                                            entities.get_mut(*target_entity)
                                        {
                                            match target_abilities.0.get(&instanced_ability_id) {
                                                None => {}
                                                Some(target_ability) => {
                                                    instanced_ability_data =
                                                        target_ability.ability_data;
                                                    if instanced_ability_data.is_none() {
                                                        ability_index = Some(instanced_ability_id);
                                                        target_entity_ref = Some(*target_entity);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                let Ok((this_instanced_abilities, mut this_instanced_modifiers)) =
                                    entities.get_mut(entity)
                                else {
                                    tracing::debug!(
                                        "[ModifierChange] Failed to get entity components for {}",
                                        invoke.entity_id
                                    );
                                    continue;
                                };

                                if instanced_ability_data.is_none() {
                                    match this_instanced_abilities.0.get(&instanced_ability_id) {
                                        None => {}
                                        Some(this_ability) => {
                                            instanced_ability_data = this_ability.ability_data;
                                            if instanced_ability_data.is_none() {
                                                ability_index = Some(instanced_ability_id);
                                            }
                                        }
                                    }
                                }

                                if instanced_ability_data.is_none() {
                                    let parent_ability_name =
                                        get_ability_name(mod_change.parent_ability_name.clone())
                                            .unwrap_or_else(|| "".to_string());
                                    match get_ability_data(&parent_ability_name) {
                                        Some(data) => {
                                            instanced_ability_data = Some(data);
                                        }
                                        None => {
                                            tracing::debug!(
                                                "[ModifierChange] No ability found: {}",
                                                parent_ability_name
                                            );
                                        }
                                    }
                                }

                                if instanced_ability_data.is_none() {
                                    continue;
                                }

                                let ability_data = instanced_ability_data.unwrap();

                                let modifier_data = match ability_data
                                    .modifiers
                                    .get_index(modifier_local_id)
                                {
                                    Some((_, m)) => m,
                                    None => {
                                        tracing::debug!(
                                                "[ModifierChange] Modifier local id {} not found in ability {}",
                                            modifier_local_id,
                                            ability_data.ability_name
                                        );
                                        continue;
                                    }
                                };

                                let is_replacing = this_instanced_modifiers
                                    .0
                                    .contains_key(&instanced_modifier_id);

                                if is_replacing {
                                    tracing::debug!("[ModifierChange] Replacing entity {} instanced_modifier_id : {} with ability {} modifier {}",
                                        invoke.entity_id,
                                        instanced_modifier_id,
                                        ability_data.ability_name,
                                        modifier_data.modifier_name
                                    );
                                } else {
                                    tracing::debug!("[ModifierChange] Adding entity {} instanced_modifier_id : {} with ability {} modifier {}",
                                        invoke.entity_id,
                                        instanced_modifier_id,
                                        ability_data.ability_name,
                                        modifier_data.modifier_name
                                    );
                                }

                                let modifier_controller = AbilityModifierController {
                                    target_entity: target_entity_ref,
                                    ability_index,
                                    ability_data: Some(ability_data),
                                    modifier_data: Some(modifier_data),
                                };

                                this_instanced_modifiers
                                    .0
                                    .insert(instanced_modifier_id, modifier_controller);
                            }
                            ModifierAction::Removed => {
                                let Ok((_, mut this_instanced_modifiers)) =
                                    entities.get_mut(entity)
                                else {
                                    tracing::debug!(
                                        "[ModifierChange] Failed to get entity components for {}",
                                        invoke.entity_id
                                    );
                                    continue;
                                };

                                tracing::debug!(
                                    "[ModifierChange] Removed on entity {} instanced_modifier_id : {}",
                                    invoke.entity_id,
                                    instanced_modifier_id,
                                );

                                this_instanced_modifiers.0.remove(&instanced_modifier_id);
                            }
                        }
                    }
                }
            }
        }
    }
}
