use crate::net::udp_server::ConnectionManager;
use anyhow::Result;
use common::data::{EncryptionConfig, RegionConfig};
use common::logging;
use common::player_cache::init_player_cache;
use dashmap::DashMap;
use db_worker::DbWorkerHandle;
use game_server_core::LogicSimulator;
use net::UdpServer;
use nod_krai_gi_data::ability::load_ability_configs_from_bin;
use nod_krai_gi_data::config::load_avatar_talent_configs_from_bin;
use nod_krai_gi_data::quest::quest_config::load_quest_configs_from_bin;
use nod_krai_gi_data::scene::scene_point_config::load_scene_point_configs_from_bin;
use nod_krai_gi_data::scene::script_cache::init_scene_static_templates;
use nod_krai_gi_data::{
    config::load_avatar_configs_from_bin, config::load_gadget_configs_from_bin, excel,
    GAME_SERVER_CONFIG,
};
use nod_krai_gi_encryption::{rsa::RsaKeyPair, xor::MhyXorpad};
use nod_krai_gi_proto::dy_parser::MULTI_VERSION_PROTOCOL;
use std::collections::HashMap;
use std::fs;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, OnceLock};
use tokio::net::UdpSocket;

mod db_worker;
mod player_info_util;

mod handler;
mod net;
mod util;

struct AppState {
    pub socket: Arc<UdpSocket>,
    pub conn_mgr: Arc<ConnectionManager>,
    pub db_handle: DbWorkerHandle,
    pub logic_simulator: LogicSimulator,
    region_config: RegionConfig,
    sessions: DashMap<u32, Arc<handler::Session>>,
    key_pair_map: HashMap<u32, RsaKeyPair>,
    initial_xor_pad: Option<MhyXorpad>,
    stop_flag: AtomicBool,
}

#[tokio::main]
async fn main() -> Result<()> {
    logging::init();

    tokio::spawn(async move {
        nod_krai_gi_proto::dy_parser::init();
    });

    init_player_cache();

    nod_krai_gi_message::PLAYER_VERSION
        .set(Arc::new(DashMap::new()))
        .expect("TODO: panic message");
    static STATE: OnceLock<AppState> = OnceLock::new();

    tokio::spawn(async {
        init_scene_static_templates("assets/lua/scene");
        tracing::info!("init_scene_static_templates end");
    });

    if GAME_SERVER_CONFIG.plugin.ability {
        tokio::spawn(async {
            load_ability_configs_from_bin("assets/BinOutput").unwrap();
            tracing::info!("load_ability_configs_from_bin end");
        });
    }

    tokio::spawn(async {
        load_quest_configs_from_bin("assets/BinOutput");
        tracing::info!("load_quest_configs_from_bin end");
    });

    tokio::spawn(async {
        load_scene_point_configs_from_bin("assets/BinOutput");
        tracing::info!("load_scene_point_configs_from_bin end");
    });

    tokio::spawn(async {
        load_avatar_configs_from_bin("assets/BinOutput").unwrap();
        tracing::info!("load_avatar_configs_from_bin end");
    });

    tokio::spawn(async {
        load_gadget_configs_from_bin("assets/BinOutput").unwrap();
        tracing::info!("load_gadget_configs_from_bin end");
    });

    tokio::spawn(async {
        load_avatar_talent_configs_from_bin("assets/BinOutput").unwrap();
        tracing::info!("load_avatar_talent_configs_from_bin end");
    });

    excel::load_all("assets/ExcelBinOutput")?;

    loop {
        if MULTI_VERSION_PROTOCOL.get().unwrap().is_empty() {
            let _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        } else {
            break;
        }
    }

    let db_connection = nod_krai_gi_database::connect_to(&GAME_SERVER_CONFIG.database)?;
    let (db_handle, save_data_tx) = db_worker::start(db_connection);

    let region_list: Vec<RegionConfig> =
        serde_json::from_str(&fs::read_to_string(&GAME_SERVER_CONFIG.region_list_path)?)?;
    let key_pair_map = serde_json::from_str::<HashMap<u32, EncryptionConfig>>(
        &fs::read_to_string(&GAME_SERVER_CONFIG.encryption_config_path)?,
    )?
    .into_iter()
    .map(|(id, conf)| (id, RsaKeyPair::from_encryption_config(&conf)))
    .collect();

    let cur_region = region_list
        .into_iter()
        .find(|r| r.name == GAME_SERVER_CONFIG.cur_region_name)
        .expect("cur_region not found in region list");

    let initial_xor_pad = if let Some(secret_key_path) = cur_region.secret_key_path.as_ref() {
        Some(MhyXorpad::from_ec2b(&fs::read(secret_key_path)?)?)
    } else {
        None
    };

    let socket = Arc::new(
        UdpSocket::bind(GAME_SERVER_CONFIG.network.udp_host.clone())
            .await
            .unwrap(),
    );

    let state = STATE.get_or_init(move || AppState {
        socket: Arc::clone(&socket),
        conn_mgr: Arc::new(ConnectionManager::default()),
        db_handle,
        logic_simulator: LogicSimulator::spawn(save_data_tx),
        region_config: cur_region,
        sessions: DashMap::new(),
        initial_xor_pad,
        key_pair_map,
        stop_flag: AtomicBool::new(false),
    });

    let udp_server = UdpServer::bind(state).await?;
    udp_server.serve(state).await;

    Ok(())
}
