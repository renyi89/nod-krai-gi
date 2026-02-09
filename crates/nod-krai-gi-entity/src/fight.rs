use crate::common::{FightProperties, Guid, ProtocolEntityID};
use bevy_ecs::change_detection::Res;
use bevy_ecs::message::{Message, MessageReader};
use bevy_ecs::prelude::{Changed, Query};
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_proto::normal::{
    AvatarFightPropUpdateNotify, ChangeEnergyReason, ChangeHpDebtsReason, ChangeHpReason,
    EntityFightPropChangeReasonNotify, EntityFightPropUpdateNotify, PropChangeReason,
    ProtEntityType,
};

pub fn notify_fight_properties_to_clients(
    mut changed_properties: Query<
        (&mut FightProperties, &ProtocolEntityID, Option<&Guid>),
        Changed<FightProperties>,
    >,
    message_output: Res<MessageOutput>,
) {
    for (mut properties, entity_id, guid) in changed_properties.iter_mut() {
        if properties.1.is_empty() {
            continue;
        }
        let fight_prop_map = properties
            .0
            .iter()
            .filter(|(ty, _)| properties.1.contains(ty))
            .map(|(ty, val)| (*ty as u32, *val))
            .collect();

        properties.1.clear();

        if entity_id.entity_type() == ProtEntityType::ProtEntityAvatar {
            message_output.send_to_all(
                "AvatarFightPropUpdateNotify",
                AvatarFightPropUpdateNotify {
                    avatar_guid: guid.map(|g| g.0).unwrap_or_default(),
                    fight_prop_map,
                },
            );
        } else {
            message_output.send_to_all(
                "EntityFightPropUpdateNotify",
                EntityFightPropUpdateNotify {
                    entity_id: entity_id.0,
                    fight_prop_map,
                },
            );
        }
    }
}

pub enum ChangeReason {
    ChangeHpReason(ChangeHpReason),
    ChangeHpDebtsReason(ChangeHpDebtsReason),
    ChangeEnergyReason(ChangeEnergyReason),
}
#[derive(Message)]
pub struct EntityFightPropChangeReasonNotifyEvent {
    pub entity_id: u32,
    pub prop_type: nod_krai_gi_data::prop_type::FightPropType,
    pub value: f32,
    pub param_list: Option<Vec<u32>>,
    pub reason: PropChangeReason,
    pub change_reason: ChangeReason,
}

pub fn notify_fight_properties_change_reason_to_clients(
    mut events: MessageReader<EntityFightPropChangeReasonNotifyEvent>,
    message_output: Res<MessageOutput>,
) {
    for event in events.read() {
        match event.change_reason {
            ChangeReason::ChangeHpReason(_) => {}
            ChangeReason::ChangeHpDebtsReason(change_hp_debts_reason) => {
                message_output.send_to_all(
                    "EntityFightPropChangeReasonNotify",
                    EntityFightPropChangeReasonNotify {
                        entity_id: event.entity_id,
                        prop_type: event.prop_type as u32,
                        prop_delta: event.value,
                        paid_hp_debts: event.value,
                        reason: event.reason as i32,
                        change_hp_debts_reason: change_hp_debts_reason as i32,
                        ..Default::default()
                    },
                );
            }
            ChangeReason::ChangeEnergyReason(_) => {}
        }
    }
}
