#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PacketHead {
    #[prost(uint32, tag = "1")]
    pub packet_id: u32,
    #[prost(uint32, tag = "2")]
    pub rpc_id: u32,
    #[prost(uint32, tag = "3")]
    pub client_sequence_id: u32,
    #[prost(uint32, tag = "4")]
    pub enet_channel_id: u32,
    #[prost(uint32, tag = "5")]
    pub enet_is_reliable: u32,
    #[prost(uint64, tag = "6")]
    pub sent_ms: u64,
    #[prost(uint32, tag = "11")]
    pub user_id: u32,
    #[prost(uint32, tag = "12")]
    pub user_ip: u32,
    #[prost(uint32, tag = "13")]
    pub user_session_id: u32,
    #[prost(uint32, tag = "14")]
    pub home_user_id: u32,
    #[prost(uint64, tag = "21")]
    pub recv_time_ms: u64,
    #[prost(uint32, tag = "22")]
    pub rpc_begin_time_ms: u32,
    #[prost(map = "uint32, uint32", tag = "23")]
    pub ext_map: ::std::collections::HashMap<u32, u32>,
    #[prost(uint32, tag = "24")]
    pub sender_app_id: u32,
    #[prost(uint32, tag = "25")]
    pub sender_load: u32,
    #[prost(bytes = "vec", tag = "26")]
    pub span_context_str: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "31")]
    pub source_service: u32,
    #[prost(uint32, tag = "32")]
    pub target_service: u32,
    #[prost(map = "uint32, uint32", tag = "33")]
    pub service_app_id_map: ::std::collections::HashMap<u32, u32>,
    #[prost(bool, tag = "34")]
    pub is_set_game_thread: bool,
    #[prost(uint32, tag = "35")]
    pub game_thread_index: u32,
    #[prost(bool, tag = "99")]
    pub is_gm_packet: bool,
}
