use dashmap::mapref::one::{Ref, RefMut};
use dashmap::DashMap;
use kcp::KCP_OVERHEAD;
use rand::RngCore;
use std::sync::atomic::AtomicU32;
use std::{fmt, net::SocketAddr, sync::Arc};
use tokio::{net::UdpSocket, sync::mpsc};

use crate::{handler::PacketHandler, AppState};

use super::{
    control_packet::{ControlPacket, ControlPacketType, CONTROL_PACKET_SIZE},
    kcp_connection::{self, NetEvent},
};

pub struct UdpServer {
    socket: Arc<UdpSocket>,
    packet_handler: PacketHandler,
}

impl UdpServer {
    pub async fn bind(state: &'static AppState) -> std::io::Result<Self> {
        let packet_handler = PacketHandler::new(state);
        Ok(Self {
            socket: Arc::clone(&state.socket),
            packet_handler,
        })
    }

    pub async fn serve(self, state: &'static AppState) {
        let conn_mgr = Arc::clone(&state.conn_mgr);
        let conn_mgr_task = Arc::clone(&state.conn_mgr);
        let packet_handler_task = self.packet_handler.clone();
        let mut buf = [0u8; 1400];

        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(tokio::time::Duration::from_secs(10));
            loop {
                ticker.tick().await;
                let remove_list = conn_mgr_task.get_remove_list();
                for (conv, token) in remove_list {
                    if let Some(id) = conn_mgr_task.remove(conv, token) {
                        tracing::info!("remove id: {}", id);
                    }
                    packet_handler_task.remove_connection(conv);
                }
            }
        });

        loop {
            if state.stop_flag.load(std::sync::atomic::Ordering::Relaxed) {
                loop {
                    if state.sessions.len() == 0 {
                        return;
                    }
                    tracing::warn!("stop game server...");
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
            let Ok((len, addr)) = self.socket.recv_from(&mut buf).await else {
                continue;
            };

            match len {
                CONTROL_PACKET_SIZE => {
                    if let Some(rsp_control_packet) = self
                        .handle_control_packet(
                            Arc::clone(&state.conn_mgr),
                            buf[..CONTROL_PACKET_SIZE].try_into().unwrap(),
                            addr,
                        )
                        .await
                    {
                        let _ = self
                            .socket
                            .send_to(rsp_control_packet.as_slice(), addr)
                            .await;
                    }
                }
                KCP_OVERHEAD.. => {
                    let buf = &buf[..len];
                    let (conv, token) = (kcp::get_conv(buf), kcp::get_token(buf));

                    if let Some(connection) = conn_mgr.get(conv, token) {
                        conn_mgr.last_seen.insert(conv, tokio::time::Instant::now());
                        let _ = connection.event_tx.send(NetEvent::Recv(buf.into())).await;
                    }
                }
                _ => (),
            }
        }
    }

    async fn handle_control_packet(
        &self,
        conn_mgr: Arc<ConnectionManager>,
        pk: [u8; CONTROL_PACKET_SIZE],
        addr: SocketAddr,
    ) -> Option<ControlPacket> {
        let conn_mgr = Arc::clone(&conn_mgr);
        let packet = ControlPacket::try_from(pk)
            .inspect_err(|err| tracing::debug!("failed to decode ControlPacket: {err}"))
            .ok()?;

        match packet.get_type() {
            ControlPacketType::Connect => {
                let conn = conn_mgr.create(self.socket.clone(), addr, self.packet_handler.clone());
                self.packet_handler.add_connection(conn.clone());
                tracing::debug!(
                    "new connection from {}, conv:{} token:{}",
                    addr, conn.conv, conn.token
                );

                Some(ControlPacket::build(
                    ControlPacketType::Establish,
                    conn.conv,
                    conn.token,
                    0,
                ))
            }
            ControlPacketType::Disconnect => {
                if let Some(id) = conn_mgr.remove(packet.get_conv(), packet.get_token()) {
                    self.packet_handler.remove_connection(id.conv);
                    tracing::debug!("client from {addr}, id: {id} disconnected");
                    Some(ControlPacket::build(
                        ControlPacketType::Disconnect,
                        id.conv,
                        id.token,
                        packet.get_data(),
                    ))
                } else {
                    None
                }
            }
            ControlPacketType::Establish => None,
        }
    }
}

#[derive(Clone)]
pub struct Connection {
    pub source_addr: SocketAddr,
    pub conv: u32,
    pub token: u32,
    event_tx: mpsc::Sender<NetEvent>,
}

impl Connection {
    pub async fn send(&self, data: Box<[u8]>) {
        let _ = self.event_tx.send(NetEvent::Send(data)).await;
    }

    pub async fn close(&self) {
        let _ = self.event_tx.send(NetEvent::Close()).await;
    }
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}|{}]", self.conv, self.token)
    }
}

#[derive(Default)]
pub struct ConnectionManager {
    connections: DashMap<u32, Connection>,
    last_seen: DashMap<u32, tokio::time::Instant>,
    connection_counter: AtomicU32,
}

impl ConnectionManager {
    fn create(
        &self,
        socket: Arc<UdpSocket>,
        addr: SocketAddr,
        handler: PacketHandler,
    ) -> RefMut<'_, u32, Connection> {
        let connection_counter = self
            .connection_counter
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let (conv, token) = (connection_counter, rand::thread_rng().next_u32());
        let event_tx = kcp_connection::start(conv, token, socket, addr, handler);

        let id = Connection {
            source_addr: addr,
            conv,
            token,
            event_tx,
        };
        self.last_seen
            .entry(id.conv)
            .or_insert(tokio::time::Instant::now());
        self.connections.entry(id.conv).or_insert(id)
    }

    pub fn get(&self, conv: u32, token: u32) -> Option<Ref<'_, u32, Connection>> {
        self.connections
            .get(&conv)
            .and_then(|c| (c.token == token).then_some(c))
    }

    pub fn get_remove_list(&self) -> Vec<(u32, u32)> {
        let mut remove_list: Vec<(u32, u32)> = Vec::new();
        let now = tokio::time::Instant::now();
        self.last_seen.iter().for_each(|x| {
            if now.duration_since(*x.value()) > tokio::time::Duration::from_secs(60) {
                remove_list.push((
                    self.connections.get(x.key()).unwrap().conv,
                    self.connections.get(x.key()).unwrap().token,
                ));
            }
        });
        remove_list
    }

    pub fn remove(&self, conv: u32, token: u32) -> Option<Connection> {
        let mut is_ok = false;
        if let Some(id) = self.connections.get(&conv) {
            if id.token == token {
                is_ok = true;
            }
        }
        if is_ok {
            self.last_seen.remove(&conv);
            match self.connections.remove(&conv) {
                None => None,
                Some((_conv, conn)) => Some(conn),
            }
        } else {
            None
        }
    }
}
