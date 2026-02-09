use std::collections::HashMap;

use crate::get_player_version;
use bevy_ecs::prelude::Resource;
use common::logging::TRACE_LOG_PACKET;
use nod_krai_gi_data::GAME_SERVER_CONFIG;
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
            let version = get_player_version!(&player_uid);
            let protocol_version = version.as_str();
            out.push(
                PacketHead::default(),
                protocol_version,
                message_name,
                message,
            );
        }
    }

    pub fn send_none(&self, player_uid: u32, message_name: &str) {
        if let Some(out) = self.0.get(&player_uid) {
            let version = get_player_version!(&player_uid);
            let protocol_version = version.as_str();
            out.push_none(PacketHead::default(), protocol_version, message_name);
        }
    }

    pub fn send_to_all<T>(&self, message_name: &str, message: T)
    where
        T: Sized + Serialize + Clone,
    {
        for (player_uid, out) in &self.0 {
            let version = get_player_version!(player_uid);
            let protocol_version = version.as_str();
            out.push(
                PacketHead::default(),
                protocol_version,
                message_name,
                message.clone(),
            );
        }
    }

    pub fn send_to_all_none(&self, message_name: &str) {
        for (player_uid, out) in &self.0 {
            let version = get_player_version!(player_uid);
            let protocol_version = version.as_str();
            out.push_none(PacketHead::default(), protocol_version, message_name);
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
            let version = get_player_version!(player_uid);
            let protocol_version = version.as_str();
            out.push(
                PacketHead::default(),
                protocol_version,
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
            let version = get_player_version!(player_uid);
            let protocol_version = version.as_str();
            out.push_none(PacketHead::default(), protocol_version, message_name);
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
                        if TRACE_LOG_PACKET.contains(&&*message_name) {
                            tracing::trace!(
                                "version:{} cmd_id:{} message_name:{}",
                                version,
                                cmd_id,
                                message_name
                            );
                            if GAME_SERVER_CONFIG.plugin.packet_log {
                                tracing::trace!("send:[{}]", hex::encode(&body));
                            }
                        } else {
                            tracing::debug!(
                                "version:{} cmd_id:{} message_name:{}",
                                version,
                                cmd_id,
                                message_name
                            );
                            if GAME_SERVER_CONFIG.plugin.packet_log {
                                tracing::debug!("send:[{}]", hex::encode(&body));
                            }
                        }

                        let _ = self.0.blocking_send((
                            cmd_id,
                            head,
                            body.into_boxed_slice(), //占位
                        ));
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
                let _ = self.0.blocking_send((
                    cmd_id,
                    head,
                    Vec::new().into_boxed_slice(), //占位
                ));
            }
        }
    }
}
