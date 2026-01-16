use crate::{AddNewAbilityEvent, util::get_ability_name};
use bevy_ecs::prelude::*;
use nod_krai_gi_data::ability::get_ability_data;
use nod_krai_gi_entity::common::{
    EntityById, InstancedAbilities, InstancedAbility, ProtocolEntityID,
};

pub fn handle_add_new_ability(
    index: Res<EntityById>,
    mut events: MessageReader<AddNewAbilityEvent>,
    mut entities: Query<(&mut InstancedAbilities, &ProtocolEntityID)>,
) {
    for AddNewAbilityEvent(invoke, version) in events.read() {
        let entity = match index.0.get(&invoke.entity_id) {
            Some(e) => *e,
            None => {
                tracing::debug!("[AddNewAbility] Entity {} not found", invoke.entity_id);
                continue;
            }
        };

        match nod_krai_gi_proto::dy_parser::decode_from_vec_by_name_version::<
            nod_krai_gi_proto::AbilityMetaAddAbility,
        >(version, "AbilityMetaAddAbility", &*invoke.ability_data)
        {
            None => {
                tracing::debug!("[AddNewAbility] Failed to decode AbilityMetaAddAbility");
            }
            Some(add_ability) => match entities.get_mut(entity) {
                Ok((mut instanced_abilities, _)) => match add_ability.ability {
                    None => {
                        continue;
                    }
                    Some(ability) => {
                        let ability_name = match get_ability_name(ability.ability_name) {
                            Some(ability_name) => ability_name,
                            None => {
                                if instanced_abilities
                                    .0
                                    .contains_key(&ability.instanced_ability_id)
                                {
                                    tracing::debug!(
                                        "[AddNewAbility] change ability.instanced_ability_id: {} ability_override",
                                        ability.instanced_ability_id
                                    );
                                }
                                continue;
                            }
                        };

                        tracing::debug!(
                            "[AddNewAbility] instanced_ability_id: {} ability_name: {} invoke.entity_id: {}",
                            ability.instanced_ability_id,
                            ability_name,
                            invoke.entity_id
                        );

                        let ability_data = match get_ability_data(&ability_name) {
                            Some(data) => data,
                            None => {
                                tracing::debug!(
                                    "[AddNewAbility] No ability found: {}",
                                    ability_name
                                );
                                continue;
                            }
                        };

                        let instanced_ability = InstancedAbility::new(Some(ability_data));

                        instanced_abilities
                            .0
                            .insert(ability.instanced_ability_id, instanced_ability);
                    }
                },
                Err(_) => {
                    continue;
                }
            },
        }
    }
}
