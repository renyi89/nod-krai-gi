use std::collections::HashMap;

use crate::player_data_sync::PlayerDataSyncPlugin;
use bevy_app::prelude::*;
use common::player_cache::cache_get_player_client_data_version;
use nod_krai_gi_ability::AbilityPlugin;
use nod_krai_gi_avatar::AvatarPlugin;
use nod_krai_gi_banner::BannerPlugin;
use nod_krai_gi_combat::CombatPlugin;
use nod_krai_gi_command::CommandPlugin;
use nod_krai_gi_data::{GAME_SERVER_CONFIG, REGION_LIST};
use nod_krai_gi_entity::EntityPlugin;
use nod_krai_gi_environment::EnvironmentPlugin;
use nod_krai_gi_event::EventRegistryPlugin;
use nod_krai_gi_inventory::InventoryPlugin;
use nod_krai_gi_luashell::{LuaShellPlugin, LuaShellSettings};
use nod_krai_gi_map::MapPlugin;
use nod_krai_gi_message::{
    event::ClientMessageEvent,
    get_player_version,
    output::{ClientOutput, MessageOutput},
};
use nod_krai_gi_pathfinding::PathfindingPlugin;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::normal::{PlayerLoginRsp, ResVersionConfig};
use nod_krai_gi_proto::server_only::PlayerDataBin;
use nod_krai_gi_proto::Protobuf;
use nod_krai_gi_quest::QuestPlugin;
use nod_krai_gi_scene::{common::WorldOwnerUID, ScenePlugin};
use nod_krai_gi_script::ScriptPlugin;
use nod_krai_gi_social::SocialPlugin;
use nod_krai_gi_time::TimePlugin;

pub struct PlayerWorld(App);

impl PlayerWorld {
    pub fn new(player_information: PlayerDataBin, output: ClientOutput) -> Self {
        let uid = player_information.uid;

        let message_out = MessageOutput::new(HashMap::from([(uid, output.clone())]));
        let players = Players::from(HashMap::from([(uid, player_information)]));

        let mut app = App::new();
        app.insert_resource(message_out)
            .insert_resource(players)
            .add_message::<ClientMessageEvent>();

        app.add_plugins(EventRegistryPlugin)
            .add_plugins(PlayerDataSyncPlugin)
            .add_plugins(EntityPlugin)
            .add_plugins(ScenePlugin)
            .add_plugins(AvatarPlugin)
            .add_plugins(BannerPlugin)
            .add_plugins(InventoryPlugin)
            .add_plugins(EnvironmentPlugin)
            .add_plugins(PathfindingPlugin)
            .add_plugins(CombatPlugin)
            .add_plugins(TimePlugin)
            .add_plugins(CommandPlugin)
            .add_plugins(MapPlugin)
            .add_plugins(LuaShellPlugin)
            .add_plugins(ScriptPlugin);

        if GAME_SERVER_CONFIG.plugin.ability {
            app.add_plugins(AbilityPlugin);
        }

        if GAME_SERVER_CONFIG.plugin.social {
            app.add_plugins(SocialPlugin);
        }

        if GAME_SERVER_CONFIG.plugin.quest {
            app.add_plugins(QuestPlugin);
        }

        app.world_mut()
            .get_resource_mut::<WorldOwnerUID>()
            .unwrap()
            .0 = uid;

        app.insert_resource(LuaShellSettings {
            startup_payloads: vec![include_bytes!("../../../assets/luashell/wm.bin")
                .to_vec()
                .into_boxed_slice()],
        });

        app.finish();
        app.cleanup();
        app.update();

        let binding = get_player_version!(&uid);
        let version = binding.as_str();
        let client_data_version = cache_get_player_client_data_version(uid).unwrap_or_default();

        let mut cur_hot_fix_data = None;

        REGION_LIST.get().unwrap().iter().for_each(|region| {
            region
                .hot_fix_data
                .iter()
                .for_each(|(_, hot_fix_data)| {
                    if hot_fix_data.client_data_version == client_data_version {
                        cur_hot_fix_data = Some(hot_fix_data.clone());
                    }
                })
        });

        match cur_hot_fix_data {
            None => {
                output.push_none(
                    nod_krai_gi_proto::packet_head::PacketHead::default(),
                    version,
                    "PlayerLoginRsp",
                );
            }
            Some(cur_hot_fix_data) => {
                output.push(
                    nod_krai_gi_proto::packet_head::PacketHead::default(),
                    version,
                    "PlayerLoginRsp",
                    PlayerLoginRsp {
                        client_md5: cur_hot_fix_data.client_data_md5.clone(),
                        client_silence_md5: cur_hot_fix_data.client_silence_data_md5.clone(),
                        client_data_version: cur_hot_fix_data.client_data_version.clone(),
                        client_silence_data_version: cur_hot_fix_data
                            .client_silence_data_version
                            .clone(),
                        client_version_suffix: cur_hot_fix_data.client_version_suffix.clone(),
                        client_silence_version_suffix: cur_hot_fix_data
                            .client_silence_version_suffix
                            .clone(),

                        res_version_config: Some(ResVersionConfig {
                            version: cur_hot_fix_data.res_version_config.version.clone(),
                            md5: cur_hot_fix_data.res_version_config.md5.clone(),
                            release_total_size: cur_hot_fix_data
                                .res_version_config
                                .release_total_size
                                .clone(),
                            version_suffix: cur_hot_fix_data.res_version_config.version_suffix.clone(),
                            branch: cur_hot_fix_data.res_version_config.branch.clone(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                );
            }
        }

        tracing::debug!("created world for player: {uid}");

        Self(app)
    }

    pub fn add_packet(
        &mut self,
        head: nod_krai_gi_proto::packet_head::PacketHead,
        cmd_id: u16,
        data: Box<[u8]>,
        message_name: String,
    ) {
        self.0
            .world_mut()
            .write_message(ClientMessageEvent::new(head, cmd_id, data, message_name));
    }

    pub fn update(&mut self) {
        self.0.update();
    }

    pub fn serialize_player_information(&mut self, uid: u32) -> Vec<u8> {
        let Some(players) = self.0.world_mut().get_resource::<Players>() else {
            return vec![];
        };
        match players.get(uid) {
            None => {
                vec![]
            }
            Some(player) => player.encode_to_vec(),
        }
    }

    pub fn should_save(&mut self, uid: u32) -> bool {
        let Some(players) = self.0.world_mut().get_resource::<Players>() else {
            return false;
        };
        let Some(player_info) = players.get(uid) else {
            return false;
        };
        let scene_id = if let Some(ref player_scene_bin) = player_info.scene_bin {
            player_scene_bin.my_cur_scene_id
        } else {
            return false;
        };
        [3, 5, 6, 7, 11, 101].contains(&scene_id)
    }
}
