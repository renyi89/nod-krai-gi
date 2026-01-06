mod get_player_token;
mod gm_util;

// use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, OnceLock};

use crate::handler::gm_util::execute_gm_cmd;
use crate::{net::Connection, util, AppState};
use anyhow::Result;
use nod_krai_gi_encryption::xor::MhyXorpad;
use nod_krai_gi_message::output::ClientOutput;
use nod_krai_gi_proto::{
    packet_head::PacketHead,
    raw_packet::{make_raw_packet, RawPacket},
    retcode::Retcode,
    GetPlayerTokenReq, GetPlayerTokenRsp, PingReq, PingRsp, PlayerLoginReq, Protobuf,
    UnionCmdNotify,
};
use tokio::sync::mpsc;

enum InputItem {
    NewConnection(Connection),
    DropConnection(u32),
    Packet(u32, Box<[u8]>),
}

#[derive(Clone)]
pub struct PacketHandler(mpsc::UnboundedSender<InputItem>);

pub struct Session {
    pub connection: Connection,
    pub account_uid: OnceLock<String>,
    pub player_uid: OnceLock<u32>,
    pub xor_pad: OnceLock<MhyXorpad>,
    pub version: OnceLock<String>,
}

impl PacketHandler {
    pub fn new(state: &'static AppState) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        tokio::spawn(async move { packet_handler_loop(rx, state).await });

        Self(tx)
    }

    pub fn add_connection(&self, conn: Connection) {
        self.0.send(InputItem::NewConnection(conn)).unwrap();
    }

    pub fn remove_connection(&self, id: u32) {
        self.0.send(InputItem::DropConnection(id)).unwrap();
    }

    pub fn enqueue(&self, conn_id: u32, buf: Box<[u8]>) {
        self.0.send(InputItem::Packet(conn_id, buf)).unwrap();
    }
}

async fn packet_handler_loop(mut rx: mpsc::UnboundedReceiver<InputItem>, state: &'static AppState) {
    while let Some(item) = rx.recv().await {
        match item {
            InputItem::NewConnection(conn) => {
                state.sessions.insert(
                    conn.conv,
                    Arc::new(Session {
                        connection: conn,
                        account_uid: OnceLock::new(),
                        player_uid: OnceLock::new(),
                        xor_pad: OnceLock::new(),
                        version: Default::default(),
                    }),
                );
            }
            InputItem::DropConnection(id) => {
                state.sessions.remove(&id);
            }
            InputItem::Packet(id, buf) => {
                if let Some(session) = state.sessions.get(&id) {
                    if let Err(err) = handle_packet(state, &session, buf, id).await {
                        tracing::warn!("handle_packet(connection_id: {id}) failed, error: {err}");
                    }
                }
            }
        }
    }
}

async fn handle_packet(
    state: &'static AppState,
    session: &Arc<Session>,
    mut data: Box<[u8]>,
    conv: u32,
) -> Result<()> {
    // if session.connection.source_addr.ip() != IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)) {
    //     return Ok(())
    // }
    util::xor_packet(
        session.xor_pad.get(),
        state.initial_xor_pad.as_ref(),
        &mut data,
    );

    let packet = RawPacket::new(&data)?;

    let cmd_id = packet.cmd_id();
    let head = packet.head();
    if head.is_gm_packet && cmd_id == 1 {
        tracing::debug!(
            "gm_packet source_addr ip:{} port:{}",
            session.connection.source_addr.ip(),
            session.connection.source_addr.port()
        );

        let body = packet.body();
        match nod_krai_gi_proto::gm::GmTalkByMuipReq::decode(body) {
            Ok(req) => {
                let rsp = execute_gm_cmd(state, req, head, body).await;
                let head = PacketHead {
                    is_gm_packet: true,
                    ..Default::default()
                };
                let mut data = make_raw_packet(2, head, &rsp.encode_to_vec());
                util::xor_packet(
                    session.xor_pad.get(),
                    state.initial_xor_pad.as_ref(),
                    &mut data,
                );

                session.connection.send(data).await;
            }
            Err(_) => {}
        }

        return Ok(());
    }

    if session.version.get().is_none() {
        tracing::debug!("received packet: {}", hex::encode(&data));
        let version = nod_krai_gi_proto::dy_parser::get_version(cmd_id);
        session.version.set(version).expect("TODO: panic message");
    }
    if session.version.get().unwrap().is_empty() {
        return Err(anyhow::anyhow!("version mismatch"));
    }
    let version = session.version.get().unwrap().as_str();
    let body = packet.body();

    match nod_krai_gi_proto::dy_parser::get_name_by_cmd_id_version(version, cmd_id) {
        Some(message_name) => {
            match message_name.as_str() {
                "GetPlayerTokenReq" => {
                    tracing::debug!("GetPlayerTokenReq received packet: {}", hex::encode(&body));

                    match nod_krai_gi_proto::dy_parser::decode_from_vec_by_name_version::<
                        GetPlayerTokenReq,
                    >(version, "GetPlayerTokenReq", body)
                    {
                        None => {
                            tracing::error!(
                                "version:{} message_name:{} error",
                                version, "GetPlayerTokenReq"
                            );
                        }
                        Some(request) => {
                            tracing::info!(
                                "GetPlayerTokenReq... account_uid:{}",
                                request.account_uid.clone()
                            );
                            let session = session.clone();

                            // Run in separate task because db fetch operation may take a while

                            match state
                                .db_handle
                                .fetch_user_uid(request.account_uid.clone())
                                .await
                            {
                                Ok(uid) => {
                                    let rsp = get_player_token::process_message(
                                        state, &session, request, uid,
                                    );

                                    tracing::debug!("USER_VERSION uid:{} version: {}", uid, version);

                                    nod_krai_gi_message::USER_VERSION
                                        .get()
                                        .unwrap()
                                        .insert(uid, version.to_string());

                                    match nod_krai_gi_proto::dy_parser::get_cmd_id_by_name_version(
                                        version,
                                        "GetPlayerTokenRsp",
                                    ) {
                                        None => {
                                            tracing::error!(
                                        "version:{} message_name:{} error",
                                        version, "GetPlayerTokenRsp"
                                    );
                                        }
                                        Some(cmd_id) => {
                                            match nod_krai_gi_proto::dy_parser::encode_to_vec_by_name_version::<GetPlayerTokenRsp>(
                                                version,
                                                "GetPlayerTokenRsp",
                                                &rsp,
                                            ) {
                                                None => {
                                                    tracing::error!(
                                                        "version:{} message_name:{} error",
                                                        version, "GetPlayerTokenRsp"
                                                    );
                                                }
                                                Some(body) => {
                                                    let mut data = make_raw_packet(
                                                        cmd_id,
                                                        PacketHead::default(),
                                                        &body,
                                                    );
                                                    util::xor_packet(
                                                        None,
                                                        state.initial_xor_pad.as_ref(),
                                                        &mut data,
                                                    );
                                                    session.connection.send(data).await;
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(_) => {
                                    tracing::error!("account_uid:{}  error", request.account_uid.clone());
                                }
                            }
                        }
                    }
                }
                "PlayerLoginReq" => {
                    tracing::debug!("PlayerLoginReq received packet: {}", hex::encode(&body));

                    match nod_krai_gi_proto::dy_parser::decode_from_vec_by_name_version::<PlayerLoginReq>(
                        version,
                        "PlayerLoginReq",
                        body,
                    ) {
                        None => {}
                        Some(_request) => {
                            let user_id = *session.player_uid.get().unwrap();
                            let user_session_id = session.connection.conv;
                            tracing::debug!(
                                "received player login request, session id: {}, player uid: {}",
                                user_session_id, user_id
                            );
                            player_login(state, user_id, user_session_id, conv).await;
                        }
                    }
                }
                "PingReq" => {
                    tracing::trace!("PingReq received packet: {}", hex::encode(&data));

                    match nod_krai_gi_proto::dy_parser::decode_from_vec_by_name_version::<PingReq>(
                        version, "PingReq", body,
                    ) {
                        None => {
                            tracing::error!("version:{} message_name:{} error", version, "PingReq");
                        }
                        Some(ping_req) => {
                            let ping_rsp = &PingRsp {
                                retcode: Retcode::RetSucc.into(),
                                client_time: ping_req.client_time,
                                seq: ping_req.seq,
                            };

                            match nod_krai_gi_proto::dy_parser::get_cmd_id_by_name_version(
                                version, "PingRsp",
                            ) {
                                None => {
                                    tracing::error!("version:{} message_name:{} error", version, "PingRsp");
                                }
                                Some(cmd_id) => {
                                    match nod_krai_gi_proto::dy_parser::encode_to_vec_by_name_version::<
                                        PingRsp,
                                    >(
                                        version, "PingRsp", &ping_rsp
                                    ) {
                                        None => {
                                            tracing::error!(
                                                "version:{} message_name:{} error",
                                                version, "PingRsp"
                                            );
                                        }
                                        Some(body) => {
                                            let mut data = make_raw_packet(
                                                cmd_id,
                                                PacketHead::default(),
                                                &body,
                                            );
                                            util::xor_packet(
                                                session.xor_pad.get(),
                                                state.initial_xor_pad.as_ref(),
                                                &mut data,
                                            );

                                            session.connection.send(data).await;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                "UnionCmdNotify" => {
                    tracing::trace!("UnionCmdNotify received packet: ...");

                    match nod_krai_gi_proto::dy_parser::decode_from_vec_by_name_version::<UnionCmdNotify>(
                        version,
                        "UnionCmdNotify",
                        body,
                    ) {
                        None => {
                            tracing::error!(
                                "version:{} message_name:{} error",
                                version, "UnionCmdNotify"
                            );
                        }
                        Some(union_cmd) => {
                            let user_id = *session.player_uid.get().unwrap();
                            let user_session_id = session.connection.conv;
                            let head = PacketHead {
                                user_id,
                                user_session_id,
                                ..Default::default()
                            };

                            for sub_cmd in union_cmd.cmd_list {
                                tracing::trace!(
                                    "received union subcommand with cmd_id: {}",
                                    sub_cmd.message_id
                                );

                                state.logic_simulator.add_client_packet(
                                    head.clone(),
                                    sub_cmd.message_id as u16,
                                    sub_cmd.body.into(),
                                    false,
                                );
                            }

                            state.logic_simulator.update_world(user_id);
                        }
                    }
                }
                _ => {
                    let user_id = *session.player_uid.get().unwrap();
                    let user_session_id = session.connection.conv;

                    let head = PacketHead {
                        user_id,
                        user_session_id,
                        ..Default::default()
                    };
                    state.logic_simulator.add_client_packet(
                        head,
                        cmd_id,
                        packet.body().into(),
                        true,
                    );
                }
            }
        }
        _ => {}
    }

    Ok(())
}

async fn player_login(state: &'static AppState, user_id: u32, user_session_id: u32, conv: u32) {
    let (tx, rx) = mpsc::channel(32);
    tokio::spawn(packet_sink(state, user_id, user_session_id, rx, conv));

    let Some(player_data) = state.db_handle.fetch(user_id).await else {
        tracing::error!("failed to get player data, uid: {user_id}");
        return;
    };

    state
        .logic_simulator
        .create_world(player_data, ClientOutput::new(tx));
}

async fn packet_sink(
    state: &'static AppState,
    user_id: u32,
    user_session_id: u32,
    mut rx: mpsc::Receiver<(u16, PacketHead, Box<[u8]>)>,
    conv: u32,
) {
    while let Some((cmd_id, head, body)) = rx.recv().await {
        match state.sessions.get(&conv) {
            None => {}
            Some(session) => {
                let mut data = make_raw_packet(
                    cmd_id,
                    PacketHead {
                        user_id,
                        user_session_id,
                        ..head
                    },
                    &body,
                );
                util::xor_packet(
                    session.xor_pad.get(),
                    state.initial_xor_pad.as_ref(),
                    &mut data,
                );

                session.connection.send(data).await;
            }
        }
    }
}
