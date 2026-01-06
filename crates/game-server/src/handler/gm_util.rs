use crate::AppState;
use nod_krai_gi_proto::packet_head::PacketHead;
use std::sync::Arc;

pub async fn execute_gm_cmd(
    state: &'static AppState,
    req: nod_krai_gi_proto::gm::GmTalkByMuipReq,
    head: PacketHead,
    data: &[u8],
) -> nod_krai_gi_proto::gm::GmTalkByMuipRsp {
    let mut rsp = nod_krai_gi_proto::gm::GmTalkByMuipRsp::default();
    rsp.uuid = req.uuid;
    rsp.retcode = 0;
    match req.msg.as_ref() {
        "ping" => {
            let mut player_count = 0;
            state
                .sessions
                .iter()
                .for_each(|x| match x.player_uid.get() {
                    None => {}
                    Some(_) => {
                        player_count = player_count + 1;
                    }
                });
            rsp.msg = format!("{{\"retcode\":0,\"status\":{{\"playerCount\":{},\"maxPlayer\":-1,\"version\":\"all\"}}}}", player_count);
        }
        "server_info" => {
            tracing::debug!("server_info connection len:{}", state.sessions.len());
            let mut player_count = 0;
            state.sessions.iter().for_each(|x| {
                tracing::debug!("server_info connection:{}", x.connection.source_addr);
                match x.player_uid.get() {
                    None => {
                        tracing::debug!("server_info no login connection");
                    }
                    Some(player_uid) => {
                        player_count = player_count + 1;
                        tracing::debug!("server_info player_uid:{}", player_uid);
                    }
                }
            });
            tracing::debug!("server_info end ...");
            rsp.msg = format!("{{\"retcode\":0,\"status\":{{\"playerCount\":{},\"maxPlayer\":-1,\"version\":\"all\"}}}}", player_count);
        }
        "stop" => {
            state
                .stop_flag
                .store(true, std::sync::atomic::Ordering::Relaxed);
            state.sessions.iter().for_each(|x| {
                tokio::spawn(async move {
                    x.connection.close().await;
                });
            });
            rsp.msg = "ok".to_string();
        }
        "kick" => {
            rsp.msg = "set_target_offline".to_string();
            match state.sessions.iter().find(|x| match x.player_uid.get() {
                None => false,
                Some(player_uid) => player_uid.eq(&req.player_uid),
            }) {
                None => {}
                Some(session) => {
                    tracing::info!("kick player_uid:{}", req.player_uid);
                    let socket_clone = Arc::clone(&state.socket);
                    let data = Some(crate::net::control_packet::ControlPacket::build(
                        crate::net::control_packet::ControlPacketType::Disconnect,
                        session.connection.conv,
                        session.connection.token,
                        5,
                    ))
                    .unwrap();
                    let _ = socket_clone
                        .send_to(data.as_slice(), session.connection.source_addr)
                        .await;
                    session.connection.close().await;
                    rsp.msg = "ok".to_string();
                }
            }
        }
        _ => {
            tracing::info!("add gm event: {}", req.msg);
            state
                .logic_simulator
                .add_client_packet(head, 1, data.into(), true);
        }
    }

    rsp
}
