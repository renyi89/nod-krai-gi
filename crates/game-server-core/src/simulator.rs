use std::{collections::HashMap, thread};

use crate::{command::LogicCommand, player_world::PlayerWorld};
use common::time_util;
use nod_krai_gi_message::output::ClientOutput;
use nod_krai_gi_persistence::player_information::PlayerInformation;
use nod_krai_gi_proto::packet_head::PacketHead;
use std::sync::mpsc;

#[derive(Clone)]
pub struct LogicSimulator(mpsc::Sender<LogicCommand>);

impl LogicSimulator {
    pub fn spawn(save_data_tx: tokio::sync::mpsc::Sender<(u32, serde_json::Value)>) -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(|| simulation_loop(rx, save_data_tx));
        Self(tx)
    }

    pub fn create_world(&self, player_information: PlayerInformation, output: ClientOutput) {
        self.0
            .send(LogicCommand::CreateWorld {
                player_information,
                output,
            })
            .unwrap();
    }

    pub fn add_client_packet(
        &self,
        head: PacketHead,
        cmd_id: u16,
        data: Box<[u8]>,
        immediate_mode: bool,
    ) {
        self.0
            .send(LogicCommand::ClientInput {
                head,
                cmd_id,
                data,
                immediate_mode,
            })
            .unwrap();
    }

    pub fn update_world(&self, uid: u32) {
        self.0.send(LogicCommand::WorldUpdate(uid)).unwrap();
    }
}

fn simulation_loop(
    command_receiver: mpsc::Receiver<LogicCommand>,
    save_data_tx: tokio::sync::mpsc::Sender<(u32, serde_json::Value)>,
) {
    // client_player_uid -> world_owner_uid
    let mut player_uid_map: HashMap<u32, u32> = HashMap::new();
    let mut player_world_map: HashMap<u32, PlayerWorld> = HashMap::new();
    let mut player_save_time_map: HashMap<u32, u64> = HashMap::new();

    while let Ok(command) = command_receiver.recv() {
        use LogicCommand::*;
        match command {
            CreateWorld {
                player_information,
                output,
            } => {
                player_save_time_map.insert(player_information.uid, time_util::unix_timestamp());
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
                    let binding = nod_krai_gi_message::USER_VERSION
                        .get()
                        .unwrap()
                        .get(&uid)
                        .unwrap();
                    let version = binding.as_str();
                    match nod_krai_gi_proto::dy_parser::get_name_by_cmd_id_version(version, cmd_id)
                    {
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
                                match message_name.as_str() {
                                    "ClientAbilityInitFinishNotify"
                                    | "AbilityInvocationsNotify"
                                    | "CombatInvocationsNotify" => {
                                        tracing::trace!(
                                            "version:{} cmd_id: {} message_name:{} \nrecv:[{}]",
                                            version,
                                            cmd_id,
                                            message_name,
                                            hex::encode(&data)
                                        );
                                    }
                                    &_ => {
                                        tracing::debug!(
                                            "version:{} cmd_id: {} message_name:{} \nrecv:[{}]",
                                            version,
                                            cmd_id,
                                            message_name,
                                            hex::encode(&data)
                                        );
                                    }
                                }

                                if let Some(world_owner_uid) = player_uid_map.get(&uid) {
                                    if let Some(world) = player_world_map.get_mut(world_owner_uid) {
                                        world.add_packet(head, cmd_id, data, message_name);
                                        if immediate_mode {
                                            world.update();
                                        }

                                        let save_time = player_save_time_map.get_mut(&uid).unwrap();
                                        let cur_time = time_util::unix_timestamp();
                                        if (cur_time - *save_time) >= 30 && world.should_save(uid) {
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
            WorldUpdate(uid) => {
                if let Some(world_owner_uid) = player_uid_map.get(&uid) {
                    if let Some(world) = player_world_map.get_mut(world_owner_uid) {
                        world.update();
                    }
                }
            }
        }
    }
}
