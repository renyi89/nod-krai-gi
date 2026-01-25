use bevy_ecs::prelude::*;
use nod_krai_gi_data::ability::AbilityModifierAction;
use nod_krai_gi_entity::common::{InstancedAbility};
use nod_krai_gi_data::prop_type::FightPropType;

#[derive(Message)]
pub struct AbilityActionGetHPPaidDebtsEvent(
    pub InstancedAbility,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Entity,
    pub Entity,
);
pub fn ability_action_get_hp_paid_debts_event(
    mut events: MessageReader<AbilityActionGetHPPaidDebtsEvent>,
    mut entities: Query<&mut nod_krai_gi_entity::common::FightProperties>,
) {
    for AbilityActionGetHPPaidDebtsEvent(_ability, action, _ability_data, _entity, target_entity) in
        events.read()
    {
        let override_map_key = action.override_map_key.as_deref().unwrap_or("");

        if override_map_key.is_empty() {
            tracing::debug!(
                "[AbilityActionGetHPPaidDebtsEvent] Missing override_map_key"
            );
            continue;
        }

        // Get fight properties from target entity
        let Ok(mut fight_props) = entities.get_mut(*target_entity) else {
            tracing::debug!(
                "[AbilityActionGetHPPaidDebtsEvent] Failed to get fight properties for entity {}",
                target_entity
            );
            continue;
        };

        // Get current HP paid debts
        let mut paid_debt = fight_props.get_property(FightPropType::FIGHT_PROP_CUR_HP_PAID_DEBTS);

        // Ensure paid debt is non-negative
        if paid_debt < 0.0 {
            paid_debt = 0.0;
        }

        // Store the value in the ability's specials
        // TODO: Implement storing value in ability's specials
        tracing::debug!(
            "[AbilityActionGetHPPaidDebtsEvent] Setting override map value {} to {}",
            override_map_key,
            paid_debt
        );

        // Update the entity's fight property
        fight_props.set_property(FightPropType::FIGHT_PROP_CUR_HP_PAID_DEBTS, paid_debt);

        // TODO: Implement broadcasting update notifications
    }
}
