use bevy_ecs::prelude::*;

use crate::common::{
    GlobalAbilityValues, InstancedAbilities, InstancedModifiers, ProtocolEntityID,
};

use super::ability::Ability;

#[derive(Bundle)]
pub struct TeamEntityBundle {
    pub marker: TeamEntityMarker,
    pub entity_id: ProtocolEntityID,
    pub ability: Ability,
    pub instanced_abilities: InstancedAbilities,
    pub instanced_modifiers: InstancedModifiers,
    pub global_ability_values: GlobalAbilityValues,
}

#[derive(Component)]
pub struct TeamEntityMarker;
