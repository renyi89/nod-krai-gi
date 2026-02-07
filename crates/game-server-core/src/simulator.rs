use std::{collections::HashMap, thread};

use crate::command::LOGIC_COMMAND_QUEUE;
use crate::{command::LogicCommand, player_world::PlayerWorld};
use common::logging::TRACE_LOG_PACKET;
use common::time_util;
use nod_krai_gi_message::get_player_version;
use nod_krai_gi_message::output::ClientOutput;
use nod_krai_gi_persistence::player_information::PlayerInformation;
use nod_krai_gi_proto::packet_head::PacketHead;

#[derive(Clone)]
pub struct LogicSimulator;

impl LogicSimulator {
    pub fn spawn(save_data_tx: tokio::sync::mpsc::Sender<(u32, serde_json::Value)>) -> Self {
        thread::spawn(|| simulation_loop(save_data_tx));
        Self
    }

    pub fn create_world(
        &self,
        uid: u32,
        player_information: PlayerInformation,
        output: ClientOutput,
    ) {
        LogicCommand::CreateWorld {
            player_information,
            output,
        }
        .push(uid);
    }

    pub fn add_client_packet(
        &self,
        uid: u32,
        head: PacketHead,
        cmd_id: u16,
        data: Box<[u8]>,
        immediate_mode: bool,
    ) {
        LogicCommand::ClientInput {
            head,
            cmd_id,
            data,
            immediate_mode,
        }
        .push(uid);
    }

    pub fn update_client_time(&self, uid: u32, client_time: u32) {
        LogicCommand::UpdateClientTime(client_time).push(uid);
    }

    pub fn update_world(&self, uid: u32) {
        LogicCommand::WorldUpdate().push(uid);
    }

    pub fn offline(&self, uid: u32) {
        LogicCommand::Offline().push(uid);
    }
}

fn simulation_loop(save_data_tx: tokio::sync::mpsc::Sender<(u32, serde_json::Value)>) {
    // client_player_uid -> world_owner_uid
    let mut player_uid_map: HashMap<u32, u32> = HashMap::new();
    let mut player_world_map: HashMap<u32, PlayerWorld> = HashMap::new();
    let mut player_save_time_map: HashMap<u32, u64> = HashMap::new();

    loop {
        while let Some((uid, command)) = LOGIC_COMMAND_QUEUE.pop() {
            use LogicCommand::*;
            match command {
                CreateWorld {
                    player_information,
                    output,
                } => {
                    player_save_time_map
                        .insert(player_information.uid, time_util::unix_timestamp());
                    player_uid_map.insert(player_information.uid, player_information.uid);
                    player_world_map.insert(
                        player_information.uid,
                        PlayerWorld::new(player_information, output),
                    );
                }
                ClientInput {
                    head,
                    cmd_id,
                    data,
                    immediate_mode,
                } => {
                    let uid = head.user_id;
                    if head.is_gm_packet {
                        if let Some(world_owner_uid) = player_uid_map.get(&uid) {
                            if let Some(world) = player_world_map.get_mut(world_owner_uid) {
                                world.add_packet(head, cmd_id, data, "GmTalkByMuipReq".to_string());
                            }
                        }
                    } else {
                        let binding = get_player_version!(&uid);
                        let version = binding.as_str();
                        match nod_krai_gi_proto::dy_parser::get_name_by_cmd_id_version(
                            version, cmd_id,
                        ) {
                            None => {
                                tracing::warn!(
                                    "UNKNOWN version:{} cmd_id:{} message_name:UNKNOWN",
                                    version,
                                    cmd_id
                                );
                            }
                            Some(message_name) => {
                                if message_name.to_uppercase() == message_name {
                                    tracing::warn!(
                                        "UNKNOWN version:{} cmd_id:{} message_name:{}",
                                        version,
                                        cmd_id,
                                        message_name
                                    );
                                } else {
                                    if TRACE_LOG_PACKET.contains(&&*message_name) {
                                        tracing::trace!(
                                            "version:{} cmd_id: {} message_name:{} \nrecv:[{}]",
                                            version,
                                            cmd_id,
                                            message_name,
                                            hex::encode(&data)
                                        );
                                    } else {
                                        tracing::debug!(
                                            "version:{} cmd_id: {} message_name:{} \nrecv:[{}]",
                                            version,
                                            cmd_id,
                                            message_name,
                                            hex::encode(&data)
                                        );
                                    }

                                    if let Some(world_owner_uid) = player_uid_map.get(&uid) {
                                        if let Some(world) =
                                            player_world_map.get_mut(world_owner_uid)
                                        {
                                            world.add_packet(head, cmd_id, data, message_name);
                                            if immediate_mode {
                                                world.update();
                                            }

                                            let save_time =
                                                player_save_time_map.get_mut(&uid).unwrap();
                                            let cur_time = time_util::unix_timestamp();
                                            if (cur_time - *save_time) >= 30
                                                && world.should_save(uid)
                                            {
                                                *save_time = cur_time;
                                                let _ = save_data_tx.blocking_send((
                                                    uid,
                                                    world.serialize_player_information(uid),
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                WorldUpdate() => {
                    if let Some(world_owner_uid) = player_uid_map.get(&uid) {
                        if let Some(world) = player_world_map.get_mut(world_owner_uid) {
                            world.update();
                        }
                    }
                }
                UpdateClientTime(client_time) => {
                    if let Some(world_owner_uid) = player_uid_map.get(&uid) {
                        if let Some(world) = player_world_map.get_mut(world_owner_uid) {
                            world.update_client_time(uid, client_time);
                        }
                    }
                }
                Offline() => {
                    if let Some(&world_owner_uid) = player_uid_map.get(&uid) {
                        if let Some(world) = player_world_map.get_mut(&world_owner_uid) {
                            let _ = save_data_tx
                                .blocking_send((uid, world.serialize_player_information(uid)));
                        }
                        player_uid_map.remove(&uid);
                        player_world_map.remove(&world_owner_uid);
                        player_save_time_map.remove(&uid);
                        tracing::info!("Player {} offline", uid);
                    }
                }
            }
        }
    }
}
