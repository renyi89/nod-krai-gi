use bevy_ecs::prelude::*;
use nod_krai_gi_data::prop_type::FightPropType;
use nod_krai_gi_event::ability::*;

pub fn ability_action_get_hp_paid_debts_event(
    mut events: MessageReader<AbilityActionGetHPPaidDebtsEvent>,
    mut fight_props_query: Query<&mut nod_krai_gi_entity::common::FightProperties>,
    mut abilities_query: Query<&mut nod_krai_gi_entity::common::InstancedAbilities>,
) {
    for AbilityActionGetHPPaidDebtsEvent(
        ability_index,
        ability_entity,
        action,
        _ability_data,
        target_entity,
    ) in events.read()
    {
        let override_map_key = action.override_map_key.unwrap_or("".into());

        if override_map_key.is_empty() {
            tracing::debug!("[AbilityActionGetHPPaidDebtsEvent] Missing override_map_key");
            continue;
        }

        // Get fight properties from target entity
        let Ok(mut fight_props) = fight_props_query.get_mut(*target_entity) else {
            tracing::debug!(
                "[AbilityActionGetHPPaidDebtsEvent] Failed to get fight properties for entity {}",
                target_entity
            );
            continue;
        };

        // Get abilities from ability entity (ability_index only applies to ability_entity)
        let Ok(mut abilities) = abilities_query.get_mut(*ability_entity) else {
            tracing::debug!(
                "[AbilityActionGetHPPaidDebtsEvent] Failed to get abilities for entity {}",
                ability_entity
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
        if let Some(ability) = abilities.list.get_mut(*ability_index as usize) {
            ability.ability_specials.insert(override_map_key, paid_debt);
            tracing::debug!(
                "[AbilityActionGetHPPaidDebtsEvent] Setting override map value {} to {}",
                override_map_key,
                paid_debt
            );
        }

        // Update the entity's fight property
        fight_props.set_property(FightPropType::FIGHT_PROP_CUR_HP_PAID_DEBTS, paid_debt);
    }
}
