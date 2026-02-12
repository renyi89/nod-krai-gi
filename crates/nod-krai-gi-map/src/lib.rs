use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use nod_krai_gi_data::excel::{
    SceneTagConfig, SceneTagConfigKeyed, BIG_WORLD_MAP_LAYER_CONFIG,
    BIG_WORLD_MAP_LAYER_FLOOR_CONFIG, BIG_WORLD_MAP_LAYER_GROUP_CONFIG,
};
use nod_krai_gi_event::scene::*;
use nod_krai_gi_message::{event::ClientMessageEvent, output::MessageOutput};
use nod_krai_gi_persistence::Players;
use std::sync::Arc;

mod on_map;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, data_request_processor)
            .add_systems(PreUpdate, on_map::message_on_map)
            .add_systems(PreUpdate, sync_scene_info_list_on_scene_init)
            .add_systems(PreUpdate, sync_group_unlimit_point_list_on_post_enter_scene);
    }
}

fn data_request_processor(
    mut events: MessageReader<ClientMessageEvent>,
    message_output: Res<MessageOutput>,
) {
    use nod_krai_gi_proto::normal::*;

    let scene_point_config_collection_clone = Arc::clone(
        nod_krai_gi_data::scene::scene_point_config::SCENE_POINT_CONFIG_COLLECTION
            .get()
            .unwrap(),
    );

    let daily_dungeon_config_collection_clone =
        Arc::clone(nod_krai_gi_data::excel::daily_dungeon_config_collection::get());

    for message in events.read() {
        match message.message_name() {
            "GetSceneAreaReq" => {
                if let Some(request) = message.decode::<GetSceneAreaReq>() {
                    message_output.send(
                        message.sender_uid(),
                        "GetSceneAreaRsp",
                        GetSceneAreaRsp {
                            retcode: nod_krai_gi_proto::retcode::Retcode::RetSucc.into(),
                            city_info_list: vec![
                                CityInfo {
                                    crystal_num: 10,
                                    city_id: 1,
                                    level: 10,
                                },
                                CityInfo {
                                    crystal_num: 10,
                                    city_id: 2,
                                    level: 10,
                                },
                                CityInfo {
                                    crystal_num: 10,
                                    city_id: 3,
                                    level: 10,
                                },
                                CityInfo {
                                    crystal_num: 10,
                                    city_id: 4,
                                    level: 10,
                                },
                                CityInfo {
                                    crystal_num: 10,
                                    city_id: 5,
                                    level: 10,
                                },
                            ],
                            scene_id: request.scene_id,
                            area_id_list: (1..=1000).collect(),
                        },
                    )
                }
            }
            "GetScenePointReq" => {
                if let Some(request) = message.decode::<GetScenePointReq>() {
                    match scene_point_config_collection_clone.get(&request.scene_id) {
                        None => {}
                        Some(scene_config) => message_output.send(
                            message.sender_uid(),
                            "GetScenePointRsp",
                            GetScenePointRsp {
                                retcode: 0,
                                is_relogin: request.is_relogin,
                                belong_uid: request.belong_uid,
                                scene_id: request.scene_id,
                                unlock_area_list: (1..=9).collect(),
                                unlocked_point_list: scene_config.points.keys().cloned().collect(),
                                unhide_point_list: scene_config.points.keys().cloned().collect(),
                                // group_unlimit_point_list: scene_config
                                //     .points
                                //     .iter()
                                //     .filter(|(_point_id, point_data)| {
                                //         point_data.r#point_type == "DungeonEntry"
                                //             && point_data.group_limit
                                //     })
                                //     .map(|(point_id, _point_data)| *point_id)
                                //     .collect(),
                                ..Default::default()
                            },
                        ),
                    }
                }
            }
            "DungeonEntryInfoReq" => {
                if let Some(request) = message.decode::<DungeonEntryInfoReq>() {
                    match scene_point_config_collection_clone.get(&request.scene_id) {
                        None => {}
                        Some(scene_config) => match scene_config.points.get(&request.point_id) {
                            None => {}
                            Some(point_config) => {
                                let mut dungeon_entry_list: Vec<DungeonEntryInfo> = vec![];
                                point_config.dungeon_ids.iter().for_each(|dungeon_id| {
                                    dungeon_entry_list.push(DungeonEntryInfo {
                                        dungeon_id: *dungeon_id,
                                        ..Default::default()
                                    });
                                });
                                point_config
                                    .dungeon_random_list
                                    .iter()
                                    .for_each(|random_id| {
                                        match daily_dungeon_config_collection_clone.get(random_id) {
                                            None => {}
                                            Some(daily_dungeon_config) => {
                                                daily_dungeon_config.sunday.iter().for_each(
                                                    |dungeon_id| {
                                                        dungeon_entry_list.push(DungeonEntryInfo {
                                                            dungeon_id: *dungeon_id,
                                                            ..Default::default()
                                                        });
                                                    },
                                                );
                                            }
                                        }
                                    });
                                message_output.send(
                                    message.sender_uid(),
                                    "DungeonEntryInfoRsp",
                                    DungeonEntryInfoRsp {
                                        point_id: request.point_id,
                                        dungeon_entry_list,
                                        ..Default::default()
                                    },
                                );
                            }
                        },
                    }
                }
            }
            &_ => {}
        }
    }
}

pub fn sync_scene_info_list_on_scene_init(
    mut events: MessageReader<SceneInitFinishEvent>,
    message_output: Res<MessageOutput>,
) {
    use nod_krai_gi_proto::normal::*;

    let scene_tag_entries_clone = Arc::clone(SceneTagConfig::get_scene_tag_entries());

    let mut info_list: Vec<PlayerWorldSceneInfo> = vec![];

    [3, 5, 6, 7, 11, 101].iter().for_each(|scene_id| match scene_tag_entries_clone.get(scene_id) {
        None => {}
        Some(scene_tag_list) => {
            let mut scene_tag_id_list: Vec<u32> = vec![];
            scene_tag_list.iter().for_each(|item| {
                scene_tag_id_list.push(item.id);
            });
            if *scene_id == 3 {
                info_list.push(PlayerWorldSceneInfo {
                    is_locked: false,
                    scene_id: 3,
                    map_layer_info: Some(MapLayerInfo {
                        unlocked_map_layer_floor_id_list: Arc::clone(
                            BIG_WORLD_MAP_LAYER_FLOOR_CONFIG.get().unwrap(),
                        )
                        .to_vec(),
                        unlocked_map_layer_group_id_list: Arc::clone(
                            BIG_WORLD_MAP_LAYER_GROUP_CONFIG.get().unwrap(),
                        )
                        .to_vec(),
                        unlocked_map_layer_id_list: Arc::clone(
                            BIG_WORLD_MAP_LAYER_CONFIG.get().unwrap(),
                        )
                        .to_vec(),
                    }),
                    scene_tag_id_list,
                });
            } else if *scene_id == 101 {
                info_list.push(PlayerWorldSceneInfo {
                    is_locked: false,
                    scene_id: 101,
                    map_layer_info: Some(MapLayerInfo {
                        unlocked_map_layer_id_list: vec![
                            1018000101, 1018000102, 1018000103, 1018000106, 1018000107, 1018000108,
                            1018000109, 1018000110, 1018000111, 1018000112,
                        ],
                        unlocked_map_layer_group_id_list: vec![10180001],
                        unlocked_map_layer_floor_id_list: vec![
                            1018000101, 1018000102, 1018000110, 1018000111, 1018000112,
                        ],
                    }),
                    scene_tag_id_list,
                });
            } else {
                info_list.push(PlayerWorldSceneInfo {
                    is_locked: false,
                    scene_id: *scene_id,
                    map_layer_info: None,
                    scene_tag_id_list,
                });
            }
        }
    });

    for SceneInitFinishEvent(uid) in events.read() {
        message_output.send(
            *uid,
            "PlayerWorldSceneInfoListNotify",
            PlayerWorldSceneInfoListNotify {
                unlocked_area_id_list: vec![4, 5, 6, 11, 14, 19, 22],
                info_list: info_list.clone(),
            },
        );
    }
}

pub fn sync_group_unlimit_point_list_on_post_enter_scene(
    mut reader: MessageReader<PostEnterSceneEvent>,
    players: Res<Players>,
    out: Res<MessageOutput>,
) {
    let scene_point_config_collection_clone = Arc::clone(
        nod_krai_gi_data::scene::scene_point_config::SCENE_POINT_CONFIG_COLLECTION
            .get()
            .unwrap(),
    );

    for PostEnterSceneEvent(uid) in reader.read() {
        let Some(player_info) = players.get(*uid) else {
            continue;
        };
        let scene_id = if let Some(ref player_scene_bin) = player_info.scene_bin {
            player_scene_bin.my_cur_scene_id
        } else {
            continue;
        };
        match scene_point_config_collection_clone.get(&scene_id) {
            None => {}
            Some(scene_config) => {
                scene_config
                    .points
                    .iter()
                    .filter(|(_point_id, point_data)| {
                        point_data.r#point_type == "DungeonEntry" && point_data.group_limit
                    })
                    .for_each(|(point_id, _point_data)| {
                        out.send(
                            *uid,
                            "UnfreezeGroupLimitNotify",
                            nod_krai_gi_proto::normal::UnfreezeGroupLimitNotify {
                                point_id: *point_id,
                                scene_id,
                            },
                        );
                    });
            }
        }
    }
}
