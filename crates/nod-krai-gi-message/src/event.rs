use crate::get_player_version;
use bevy_ecs::prelude::*;
use nod_krai_gi_proto::packet_head::PacketHead;
use serde::{de, Serialize};

#[derive(Message)]
pub struct ClientMessageEvent(PacketHead, u16, Box<[u8]>, String);

impl ClientMessageEvent {
    pub fn new(head: PacketHead, cmd_id: u16, data: Box<[u8]>, message_name: String) -> Self {
        Self(head, cmd_id, data, message_name)
    }

    pub const fn sender_uid(&self) -> u32 {
        self.0.user_id
    }

    pub const fn head(&self) -> &PacketHead {
        &self.0
    }

    pub const fn body(&self) -> &Box<[u8]> {
        &self.2
    }

    pub const fn cmd_id(&self) -> u16 {
        self.1
    }

    pub const fn message_name(&self) -> &str {
        self.3.as_str()
    }

    pub fn version(&self) -> String {
        get_player_version!(&self.0.user_id)
    }

    pub fn decode<T: Sized + Serialize + Default>(&self) -> Option<T>
    where
        T: for<'a> de::Deserialize<'a>,
    {
        let binding = get_player_version!(&self.0.user_id);
        let version = binding.as_str();

        let Some(binding) = nod_krai_gi_proto::dy_parser::get_name_by_cmd_id_version(version, self.1) else {
            return None;
        };
        let message_name = binding.as_str();
        match nod_krai_gi_proto::dy_parser::decode_from_vec_by_name_version::<T>(
            version,
            message_name,
            &*self.2.clone(),
        ) {
            None => None,
            Some(value) => Some(value),
        }
    }
}
