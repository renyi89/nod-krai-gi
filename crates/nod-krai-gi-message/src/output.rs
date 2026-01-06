use std::collections::HashMap;

use bevy_ecs::prelude::Resource;
use nod_krai_gi_proto::packet_head::PacketHead;
use serde::Serialize;
use tokio::sync::mpsc;
#[derive(Clone)]
pub struct ClientOutput(mpsc::Sender<(u16, PacketHead, Box<[u8]>)>);

#[derive(Resource)]
pub struct MessageOutput(HashMap<u32, ClientOutput>);

impl MessageOutput {
    pub fn new(client_map: HashMap<u32, ClientOutput>) -> Self {
        Self(client_map)
    }

    pub fn send<T>(&self, player_uid: u32, message_name: &str, message: T)
    where
        T: Sized + Serialize,
    {
        if let Some(out) = self.0.get(&player_uid) {
            let binding = crate::USER_VERSION.get().unwrap().get(&player_uid).unwrap();
            let version = binding.as_str();
            out.push(PacketHead::default(), version, message_name, message);
        }
    }

    pub fn send_none(&self, player_uid: u32, message_name: &str) {
        if let Some(out) = self.0.get(&player_uid) {
            let binding = crate::USER_VERSION.get().unwrap().get(&player_uid).unwrap();
            let version = binding.as_str();
            out.push_none(PacketHead::default(), version, message_name);
        }
    }

    pub fn send_to_all<T>(&self, message_name: &str, message: T)
    where
        T: Sized + Serialize + Clone,
    {
        for (player_uid, out) in &self.0 {
            let binding = crate::USER_VERSION.get().unwrap().get(&player_uid).unwrap();
            let version = binding.as_str();
            out.push(
                PacketHead::default(),
                version,
                message_name,
                message.clone(),
            );
        }
    }

    pub fn send_to_all_none(&self, message_name: &str) {
        for (player_uid, out) in &self.0 {
            let binding = crate::USER_VERSION.get().unwrap().get(&player_uid).unwrap();
            let version = binding.as_str();
            out.push_none(PacketHead::default(), version, message_name);
        }
    }

    pub fn send_to_others<T>(&self, host_player_uid: u32, message_name: &str, message: T)
    where
        T: Sized + Serialize + Clone,
    {
        for (player_uid, out) in &self.0 {
            if *player_uid == host_player_uid {
                continue;
            }
            let binding = crate::USER_VERSION.get().unwrap().get(&player_uid).unwrap();
            let version = binding.as_str();
            out.push(
                PacketHead::default(),
                version,
                message_name,
                message.clone(),
            );
        }
    }

    pub fn send_to_others_none(&self, host_player_uid: u32, message_name: &str) {
        for (player_uid, out) in &self.0 {
            if *player_uid == host_player_uid {
                continue;
            }
            let binding = crate::USER_VERSION.get().unwrap().get(&player_uid).unwrap();
            let version = binding.as_str();
            out.push_none(PacketHead::default(), version, message_name);
        }
    }
}

impl ClientOutput {
    pub fn new(tx: mpsc::Sender<(u16, PacketHead, Box<[u8]>)>) -> Self {
        Self(tx)
    }

    pub fn push<T>(&self, head: PacketHead, version: &str, message_name: &str, message: T)
    where
        T: Sized + Serialize,
    {
        match nod_krai_gi_proto::dy_parser::get_cmd_id_by_name_version(version, message_name) {
            None => {
                tracing::warn!(
                    "cmd_id not found version:{} message_name:{}",
                    version,
                    message_name
                );
            }
            Some(cmd_id) => {
                match nod_krai_gi_proto::dy_parser::encode_to_vec_by_name_version::<T>(
                    version,
                    message_name,
                    &message,
                ) {
                    None => {
                        tracing::warn!(
                            "message not found version:{} message_name:{}",
                            version,
                            message_name
                        );
                    }
                    Some(body) => {
                        tracing::debug!(
                            "version:{} cmd_id:{} message_name:{} \nsend:[{}]",
                            version,
                            cmd_id,
                            message_name,
                            hex::encode(&body)
                        );
                        self.0
                            .blocking_send((
                                cmd_id,
                                head,
                                body.into_boxed_slice(), //占位
                            ))
                            .unwrap()
                    }
                }
            }
        }
    }

    pub fn push_none(&self, head: PacketHead, version: &str, message_name: &str) {
        match nod_krai_gi_proto::dy_parser::get_cmd_id_by_name_version(version, message_name) {
            None => {
                tracing::warn!(
                    "cmd_id not found version:{} message_name:{}",
                    version,
                    message_name
                );
            }
            Some(cmd_id) => {
                tracing::debug!(
                    "version:{} cmd_id:{} message_name:{} \nsend:[]",
                    version,
                    cmd_id,
                    message_name
                );
                self.0
                    .blocking_send((
                        cmd_id,
                        head,
                        Vec::new().into_boxed_slice(), //占位
                    ))
                    .unwrap()
            }
        }
    }
}
