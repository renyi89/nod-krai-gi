use crate::common::*;
use bevy_ecs::{prelude::*, query::QueryData};
use nod_krai_gi_message::event::ClientMessageEvent;
use nod_krai_gi_proto::{EvtCreateGadgetNotify, EvtDestroyGadgetNotify};


#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum EntitySystemSet {
    HandleEvtGadgetUpdate,
}

#[derive(Component)]
pub struct ClientGadgetID(pub u32);

#[derive(Bundle)]
pub struct ClientGadgetBundle {
    pub gadget_id: ClientGadgetID,
    pub entity_id: ProtocolEntityID,
    pub owner_entity_id: OwnerProtocolEntityID,
    pub instanced_abilities: InstancedAbilities,
    pub instanced_modifiers: InstancedModifiers,
    pub global_ability_values: GlobalAbilityValues,
}

#[derive(QueryData)]
pub struct ClientGadgetQueryReadOnly {
    pub gadget_id: &'static ClientGadgetID,
    pub entity_id: &'static ProtocolEntityID,
    pub owner_entity_id: &'static OwnerProtocolEntityID,
    pub instanced_abilities: &'static InstancedAbilities,
    pub instanced_modifiers: &'static InstancedModifiers,
    pub global_ability_values: &'static GlobalAbilityValues,
}

pub fn handle_evt_update_gadget(
    index: Res<EntityById>,
    mut events: MessageReader<ClientMessageEvent>,
    mut commands: Commands,
) {
    for message in events.read() {
        match message.message_name() {
            "EvtCreateGadgetNotify" => {
                if let Some(notify) = message.decode::<EvtCreateGadgetNotify>() {
                    let gadget_id = notify.config_id;

                    tracing::debug!(
                        "spawn ClientGadget gadget_id:{} entity_id:{} owner_entity_id:{}",
                        gadget_id,
                        notify.entity_id,
                        notify.owner_entity_id
                    );

                    commands.spawn(ClientGadgetBundle {
                        gadget_id: ClientGadgetID(gadget_id),
                        entity_id: ProtocolEntityID(notify.entity_id),
                        owner_entity_id: OwnerProtocolEntityID(Some(notify.owner_entity_id)),
                        instanced_abilities: InstancedAbilities::default(),
                        instanced_modifiers: InstancedModifiers::default(),
                        global_ability_values: GlobalAbilityValues::default(),
                    });
                }
            }
            "EvtDestroyGadgetNotify" => {
                if let Some(notify) = message.decode::<EvtDestroyGadgetNotify>() {
                    let entity_id = notify.entity_id;
                    let entity = match index.0.get(&entity_id) {
                        Some(e) => *e,
                        None => continue,
                    };
                    commands.entity(entity).insert(ToBeRemovedMarker);
                }
            }
            &_ => {}
        }
    }
}
