mod gacha;

use crate::gacha::roll_gacha_once_readonly;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use common::time_util::unix_timestamp;
use nod_krai_gi_data::custom::gacha_banner_collection;
use nod_krai_gi_message::event::ClientMessageEvent;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::normal::{
    DoGachaReq, DoGachaRsp, GachaInfo, GachaItem, GachaUpInfo, GachaWishReq, GachaWishRsp,
    GetGachaInfoRsp, ItemParam,
};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::sync::Arc;

pub struct BannerPlugin;

impl Plugin for BannerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, on_banner_notify);
    }
}

fn on_banner_notify(
    mut events: MessageReader<ClientMessageEvent>,
    message_output: Res<MessageOutput>,
    mut players: ResMut<Players>,
) {
    for message in events.read() {
        match message.message_name() {
            "GetGachaInfoReq" => {
                let Some(player_info) = players.get(message.sender_uid()) else {
                    continue;
                };
                let Some(ref player_gacha_bin) = player_info.gacha_bin else {
                    continue;
                };

                if player_gacha_bin.gacha_map.is_empty() {
                    continue;
                }

                let gacha_banner_collection_clone = Arc::clone(gacha_banner_collection::get());

                let rsp = GetGachaInfoRsp {
                    gacha_info_list: player_gacha_bin
                        .gacha_map
                        .iter()
                        .flat_map(|(schedule_id, schedule_gacha_bin)| {
                            match gacha_banner_collection_clone.get(schedule_id) {
                                None => None,
                                Some(gacha_banner) => {
                                    let mut gacha_up_info_list = vec![];
                                    if !gacha_banner.rate_up_items_5.is_empty() {
                                        gacha_up_info_list.push(GachaUpInfo {
                                            item_id_list: gacha_banner.rate_up_items_5.clone(),
                                            item_parent_type: 1,
                                        });
                                    }
                                    if !gacha_banner.rate_up_items_4.is_empty() {
                                        gacha_up_info_list.push(GachaUpInfo {
                                            item_id_list: gacha_banner.rate_up_items_4.clone(),
                                            item_parent_type: 2,
                                        });
                                    }
                                    Some(GachaInfo {
                                        gacha_type: gacha_banner.gacha_type,
                                        schedule_id: gacha_banner.schedule_id,
                                        begin_time: (unix_timestamp() - 3600 * 24 * 30) as u32,
                                        end_time: (unix_timestamp() + 3600 * 24 * 30) as u32,
                                        cost_item_id: gacha_banner.cost_item_id,
                                        cost_item_num: 1,
                                        ten_cost_item_id: gacha_banner.cost_item_id,
                                        ten_cost_item_num: gacha_banner
                                            .cost_item_amount_10
                                            .unwrap_or(10),

                                        gacha_prefab_path: gacha_banner.prefab_path.to_string(),
                                        gacha_preview_prefab_path: gacha_banner.preview_prefab_path.unwrap_or(format!(
                                            "UI_Tab_{}",
                                            gacha_banner.prefab_path
                                        ).into()).to_string() ,
                                        title_textmap: gacha_banner.title_path.to_string(),

                                        gacha_prob_url: "https://webstatic.mihoyo.com/hk4e/event/e20190909gacha-v3/index.html".to_string(),
                                        gacha_prob_url_oversea: "https://webstatic.mihoyo.com/hk4e/event/e20190909gacha-v3/index.html".to_string(),
                                        gacha_record_url: "https://webstatic.mihoyo.com/hk4e/event/e20190909gacha-v3/index.html".to_string(),
                                        gacha_record_url_oversea: "https://webstatic.mihoyo.com/hk4e/event/e20190909gacha-v3/index.html".to_string(),

                                        display_chronicle_5_item_list: {
                                            if gacha_banner.gacha_type == 302 || gacha_banner.gacha_type == 500{
                                                gacha_banner.rate_up_items_5.clone()
                                            } else {
                                                vec![]
                                            }
                                        },
                                        display_up5_item_list: {
                                            if gacha_banner.gacha_type == 301
                                                || gacha_banner.gacha_type == 302
                                            {
                                                gacha_banner.rate_up_items_5.clone()
                                            } else {
                                                vec![]
                                            }
                                        },
                                        display_up4_item_list: {
                                            if gacha_banner.gacha_type == 302 {
                                                gacha_banner.rate_up_items_4.clone()
                                            } else {
                                                vec![]
                                            }
                                        },
                                        gacha_up_info_list,

                                        gacha_times_limit: u32::MAX,
                                        left_gacha_times: u32::MAX,
                                        cur_schedule_daily_gacha_times: 0,

                                        gacha_sort_id: gacha_banner.sort_id,

                                        is_new_wish: schedule_gacha_bin.wish_item_id == 0,
                                        wish_item_id: schedule_gacha_bin.wish_item_id,
                                        wish_progress: {
                                            if schedule_gacha_bin.item5_is_up_list.is_empty() {
                                                0
                                            } else {
                                               if *schedule_gacha_bin.item5_is_up_list.last().unwrap_or(&false) {
                                                   0
                                               }else {
                                                   1
                                               }
                                            }
                                        },
                                        wish_max_progress: {
                                            if schedule_gacha_bin.wish_item_id == 0 {
                                                0
                                            } else {
                                                1
                                            }
                                        },
                                        ..Default::default()
                                    })
                                }
                            }
                        })
                        .collect(),
                    ..Default::default()
                };

                message_output.send(message.sender_uid(), "GetGachaInfoRsp", rsp);
            }
            "DoGachaReq" => {
                let Some(player_info) = players.get_mut(message.sender_uid()) else {
                    continue;
                };
                let Some(ref mut player_gacha_bin) = player_info.gacha_bin else {
                    continue;
                };

                if player_gacha_bin.gacha_map.is_empty() {
                    continue;
                }

                let gacha_banner_collection_clone = Arc::clone(gacha_banner_collection::get());

                if let Some(request) = message.decode::<DoGachaReq>() {
                    match player_gacha_bin
                        .gacha_map
                        .get_mut(&request.gacha_schedule_id)
                    {
                        None => {}
                        Some(schedule_gacha_bin) => {
                            match gacha_banner_collection_clone.get(&request.gacha_schedule_id) {
                                None => {}
                                Some(gacha_banner) => {
                                    let mut rng = SmallRng::from_entropy();

                                    let mut gacha_item_list = vec![];
                                    for _ in 0..request.gacha_times.clamp(1, 10) {
                                        schedule_gacha_bin.fail_4_count += 1;
                                        schedule_gacha_bin.fail_5_count += 1;

                                        let (item_id, is_4, is_5, is_up_4, is_up_5) =
                                            roll_gacha_once_readonly(
                                                &mut rng,
                                                gacha_banner,
                                                schedule_gacha_bin,
                                            );
                                        if is_4 {
                                            schedule_gacha_bin.item4_is_up_list.push(is_up_4);
                                            schedule_gacha_bin.fail_4_count = 0;
                                        }
                                        if is_5 {
                                            schedule_gacha_bin.item4_is_up_list.push(is_up_5);
                                            schedule_gacha_bin.fail_5_count = 0;
                                        }
                                        gacha_item_list.push(GachaItem {
                                            gacha_item: Some(ItemParam { count: 1, item_id }),
                                            is_gacha_item_new: true,
                                            ..Default::default()
                                        });
                                    }

                                    message_output.send(
                                        message.sender_uid(),
                                        "DoGachaRsp",
                                        DoGachaRsp {
                                            retcode: 0,

                                            gacha_type: gacha_banner.gacha_type,
                                            gacha_schedule_id: gacha_banner.schedule_id,

                                            cost_item_id: gacha_banner.cost_item_id,
                                            cost_item_num: 1,
                                            ten_cost_item_id: gacha_banner.cost_item_id,
                                            ten_cost_item_num: gacha_banner
                                                .cost_item_amount_10
                                                .unwrap_or(10),

                                            gacha_times_limit: u32::MAX,
                                            left_gacha_times: u32::MAX,
                                            cur_schedule_daily_gacha_times: 0,
                                            daily_gacha_times: 10,
                                            gacha_times: 0,

                                            new_gacha_random: 12345,
                                            gacha_item_list,

                                            wish_item_id: schedule_gacha_bin.wish_item_id,
                                            wish_progress: {
                                                if schedule_gacha_bin.item5_is_up_list.is_empty() {
                                                    0
                                                } else {
                                                    if *schedule_gacha_bin
                                                        .item5_is_up_list
                                                        .last()
                                                        .unwrap_or(&false)
                                                    {
                                                        0
                                                    } else {
                                                        1
                                                    }
                                                }
                                            },
                                            wish_max_progress: {
                                                if schedule_gacha_bin.wish_item_id == 0 {
                                                    0
                                                } else {
                                                    1
                                                }
                                            },

                                            ..Default::default()
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
            }
            "GachaWishReq" => {
                let Some(player_info) = players.get_mut(message.sender_uid()) else {
                    continue;
                };
                let Some(ref mut player_gacha_bin) = player_info.gacha_bin else {
                    continue;
                };

                if player_gacha_bin.gacha_map.is_empty() {
                    continue;
                }

                if let Some(request) = message.decode::<GachaWishReq>() {
                    match player_gacha_bin
                        .gacha_map
                        .get_mut(&request.gacha_schedule_id)
                    {
                        None => {}
                        Some(schedule_gacha_bin) => {
                            schedule_gacha_bin.wish_item_id = request.item_id;
                            message_output.send(
                                message.sender_uid(),
                                "GachaWishRsp",
                                GachaWishRsp {
                                    gacha_schedule_id: request.gacha_schedule_id,
                                    retcode: 0,
                                    gacha_type: request.gacha_type,
                                    wish_item_id: request.item_id,
                                    wish_progress: 0,
                                    wish_max_progress: 1,
                                },
                            );
                        }
                    }
                }
            }
            &_ => {}
        }
    }
}
