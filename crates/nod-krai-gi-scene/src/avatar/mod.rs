use bevy_ecs::prelude::*;
use nod_krai_gi_data::prop_type::FightPropType;
use nod_krai_gi_entity::avatar::{CurrentTeam, ReplaceCurrentPlayerAvatarMarker};
use nod_krai_gi_entity::common::{FightProperties, LifeState, Visible};
use nod_krai_gi_entity::transform::Transform;
use nod_krai_gi_entity::{
    avatar::{AvatarQueryReadOnly, CurrentPlayerAvatarMarker},
    EntityDisappearEvent, EntityPropertySeparateUpdateEvent,
};
use nod_krai_gi_event::scene::*;
use nod_krai_gi_message::{event::ClientMessageEvent, get_player_version, output::MessageOutput};
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::dy_parser::{
    replace_in_u32, replace_in_u64, replace_out_i32, replace_out_u32, replace_out_u64,
};
use nod_krai_gi_proto::normal::{
    AvatarDieAnimationEndReq, AvatarDieAnimationEndRsp, AvatarTeam, AvatarTeamUpdateNotify,
    ChangeAvatarReq, ChangeAvatarRsp, SetUpAvatarTeamReq, SetUpAvatarTeamRsp, VisionType,
};
use nod_krai_gi_proto::retcode::Retcode;
use std::collections::HashSet;
use tracing::{debug, instrument};

pub fn change_avatar(
    mut client_messages: MessageReader<ClientMessageEvent>,
    mut commands: Commands,
    mut players: ResMut<Players>,
    avatars: Query<(
        Entity,
        &FightProperties,
        &LifeState,
        AvatarQueryReadOnly,
        Option<&CurrentPlayerAvatarMarker>,
    )>,
    message_output: Res<MessageOutput>,
    mut update_separate_property_entity_events: MessageWriter<EntityPropertySeparateUpdateEvent>,
    mut disappear_events: MessageWriter<EntityDisappearEvent>,
) {
    for message in client_messages.read() {
        match message.message_name() {
            "ChangeAvatarReq" => {
                if let Some(request) = message.decode::<ChangeAvatarReq>() {
                    let Some((cur_entity, _, _, cur_avatar_data, _)) =
                        avatars.iter().find(|(_, _, _, data, is_cur)| {
                            data.owner_player_uid.0 == message.sender_uid() && is_cur.is_some()
                        })
                    else {
                        tracing::error!("ChangeAvatarReq error");
                        continue;
                    };

                    if cur_avatar_data.guid.0 != request.guid {
                        if let Some((new_entity, _, _, _, _)) =
                            avatars.iter().find(|(_, _, _, data, _)| {
                                data.owner_player_uid.0 == message.sender_uid()
                                    && data.guid.0 == request.guid
                            })
                        {
                            commands
                                .entity(cur_entity)
                                .remove::<CurrentPlayerAvatarMarker>()
                                .remove::<Visible>();

                            disappear_events.write(EntityDisappearEvent(
                                cur_avatar_data.entity_id.0,
                                VisionType::VisionReplace.into(),
                            ));

                            commands
                                .entity(new_entity)
                                .insert(CurrentPlayerAvatarMarker)
                                .insert(Visible)
                                .insert(ReplaceCurrentPlayerAvatarMarker(
                                    cur_avatar_data.entity_id.0,
                                ));

                            let Some(player_info) = players.get_mut(message.sender_uid()) else {
                                continue;
                            };
                            if let Some(ref mut avatar_bin) = player_info.avatar_bin {
                                avatar_bin.cur_avatar_guid = request.guid;
                            }

                            message_output.send(
                                message.sender_uid(),
                                "ChangeAvatarRsp",
                                ChangeAvatarRsp {
                                    cur_guid: request.guid,
                                    skill_id: request.skill_id,
                                    retcode: Retcode::RetSucc.into(),
                                },
                            );
                        }
                    }
                }
            }
            "AvatarDieAnimationEndReq" => {
                if let Some(request) = message.decode::<AvatarDieAnimationEndReq>() {
                    let Some(player_info) = players.get_mut(message.sender_uid()) else {
                        continue;
                    };
                    let avatar_guid_list = if let Some(ref avatar_bin) = player_info.avatar_bin {
                        avatar_bin.cur_avatar_guid_list.clone()
                    } else {
                        continue;
                    };

                    let mut all_dead = true;
                    let mut first_alive_avatar_guid = None;
                    let mut first_alive_avatar_entity = None;
                    let mut first_alive_avatar_transform = None;

                    for (avatar_entity, _, life_state, avatar_data, _) in
                        avatars.iter().filter(|(_, _, _, a, _)| {
                            a.owner_player_uid.0 == message.sender_uid()
                                && avatar_guid_list.contains(&a.guid.0)
                        })
                    {
                        if *life_state == LifeState::Alive && first_alive_avatar_guid.is_none() {
                            all_dead = false;
                            first_alive_avatar_guid = Some(avatar_data.guid.0);
                            first_alive_avatar_entity = Some(avatar_entity);
                            first_alive_avatar_transform = Some(avatar_data.transform.clone());
                        }
                    }

                    if let (Some(guid), Some(entity), Some(transform)) = (first_alive_avatar_guid, first_alive_avatar_entity, first_alive_avatar_transform) {
                        if let Some(ref mut avatar_bin) = player_info.avatar_bin {
                            avatar_bin.cur_avatar_guid = guid;
                        }

                        let transform = match request.reborn_pos {
                            Some(move_pos) => Transform {
                                position: move_pos.into(),
                                rotation: transform.rotation,
                            },
                            _ => transform,
                        };

                        debug!("transform:{}", transform);

                        commands
                            .entity(entity)
                            .insert(CurrentPlayerAvatarMarker)
                            .insert(Visible)
                            .insert(ReplaceCurrentPlayerAvatarMarker(0))
                            .insert(transform.clone());
                    }

                    debug!("all_dead:{}", all_dead);
                    if all_dead {
                        let Some(ref avatar_bin) = player_info.avatar_bin else {
                            continue;
                        };
                        for (avatar_entity, fight_props, _, avatar_data, _) in
                            avatars.iter().filter(|(_, _, _, a, _)| {
                                a.owner_player_uid.0 == message.sender_uid()
                                    && avatar_bin
                                        .cur_avatar_guid_list
                                        .contains(&a.guid.0)
                            })
                        {
                            let max_hp = fight_props.get_property(FightPropType::FIGHT_PROP_MAX_HP);
                            update_separate_property_entity_events.write(
                                EntityPropertySeparateUpdateEvent(
                                    avatar_entity,
                                    FightPropType::FIGHT_PROP_CUR_HP,
                                    max_hp,
                                ),
                            );
                            if avatar_bin.cur_avatar_guid == avatar_data.guid.0 {
                                let transform = match request.reborn_pos {
                                    Some(move_pos) => Transform {
                                        position: move_pos.into(),
                                        rotation: avatar_data.transform.rotation,
                                    },
                                    _ => avatar_data.transform.clone(),
                                };

                                debug!("transform:{}", transform);

                                commands
                                    .entity(avatar_entity)
                                    .insert(CurrentPlayerAvatarMarker)
                                    .insert(Visible)
                                    .insert(ReplaceCurrentPlayerAvatarMarker(0))
                                    .insert(transform.clone());
                            }
                        }
                    };

                    message_output.send(
                        message.sender_uid(),
                        "AvatarDieAnimationEndRsp",
                        AvatarDieAnimationEndRsp {
                            die_guid: request.die_guid,
                            retcode: 0,
                            skill_id: request.skill_id,
                        },
                    );
                }
            }
            &_ => {}
        }
    }
}

#[instrument(skip_all)]
pub fn set_up_avatar_team(
    mut client_messages: MessageReader<ClientMessageEvent>,
    out: Res<MessageOutput>,
    mut players: ResMut<Players>,
    mut change_events: MessageWriter<PlayerAvatarTeamChanged>,
) {
    for message in client_messages.read() {
        match message.message_name() {
            "SetUpAvatarTeamReq" => {
                if let Some(request) = message.decode::<SetUpAvatarTeamReq>() {
                    let mut avatar_set =
                        HashSet::with_capacity(request.avatar_team_guid_list.len());
                    for guid in request.avatar_team_guid_list.iter() {
                        if !avatar_set.insert(*guid) {
                            debug!(
                                "duplicate guid {guid} in avatar team {:?}",
                                request.avatar_team_guid_list
                            );

                            out.send(
                                message.sender_uid(),
                                "SetUpAvatarTeamRsp",
                                SetUpAvatarTeamRsp {
                                    retcode: Retcode::RetFail.into(),
                                    ..Default::default()
                                },
                            );
                            continue;
                        }
                    }

                    let Some(player_info) = players.get_mut(message.sender_uid()) else {
                        continue;
                    };
                    let Some(ref mut avatar_bin) = player_info.avatar_bin else {
                        continue;
                    };

                    let version = get_player_version!(&player_info.uid);
                    let protocol_version = version.as_str();

                    let team_id = replace_in_u32(
                        protocol_version,
                        "SetUpAvatarTeamReq.team_id",
                        request.team_id,
                    );

                    if let Some(team) = avatar_bin.team_map.get_mut(&team_id) {
                        let mut cur_avatar_guid = replace_in_u64(
                            protocol_version,
                            "SetUpAvatarTeamReq.cur_avatar_guid",
                            request.cur_avatar_guid,
                        );

                        if !request.avatar_team_guid_list.contains(&cur_avatar_guid) {
                            let Some(temp_cur_avatar_guid) = request.avatar_team_guid_list.first()
                            else {
                                continue;
                            };
                            cur_avatar_guid = *temp_cur_avatar_guid;
                        }

                        team.avatar_guid_list = request.avatar_team_guid_list.clone();

                        if team_id == avatar_bin.cur_team_id {
                            avatar_bin.cur_avatar_guid = cur_avatar_guid;
                            avatar_bin.cur_avatar_guid_list =
                                request.avatar_team_guid_list.clone();

                            change_events.write(PlayerAvatarTeamChanged {
                                uid: message.sender_uid(),
                                avatar_team_guid_list: request.avatar_team_guid_list.clone(),
                                cur_avatar_guid,
                            });
                        }

                        out.send(
                            message.sender_uid(),
                            "SetUpAvatarTeamRsp",
                            SetUpAvatarTeamRsp {
                                retcode: replace_out_i32(
                                    protocol_version,
                                    "SetUpAvatarTeamRsp.retcode",
                                    Retcode::RetSucc.into(),
                                ),
                                team_id: replace_out_u32(
                                    protocol_version,
                                    "SetUpAvatarTeamRsp.team_id",
                                    team_id,
                                ),
                                cur_avatar_guid: replace_out_u64(
                                    protocol_version,
                                    "SetUpAvatarTeamRsp.cur_avatar_guid",
                                    cur_avatar_guid,
                                ),
                                avatar_team_guid_list: request.avatar_team_guid_list.clone(),
                            },
                        );
                    } else {
                        debug!("team_id {} doesn't exist", request.team_id);

                        out.send(
                            message.sender_uid(),
                            "SetUpAvatarTeamRsp",
                            SetUpAvatarTeamRsp {
                                retcode: replace_out_i32(
                                    protocol_version,
                                    "SetUpAvatarTeamRsp.retcode",
                                    Retcode::RetFail.into(),
                                ),
                                ..Default::default()
                            },
                        );
                    }
                }
            }
            &_ => {}
        }
    }
}

pub fn replace_avatar_team(
    mut events: MessageReader<PlayerAvatarTeamChanged>,
    mut commands: Commands,
    avatars: Query<(Entity, AvatarQueryReadOnly)>,
    mut join_team_events: MessageWriter<PlayerJoinTeamEvent>,
    mut disappear_events: MessageWriter<EntityDisappearEvent>,
) {
    for event in events.read() {
        // TODO: multiple teams - check if modified team is active

        for (avatar_entity, avatar_data) in avatars
            .iter()
            .filter(|(_, a)| a.owner_player_uid.0 == event.uid)
        {
            commands
                .entity(avatar_entity)
                .remove::<CurrentTeam>()
                .remove::<CurrentPlayerAvatarMarker>()
                .remove::<Visible>();

            disappear_events.write(EntityDisappearEvent(
                avatar_data.entity_id.0,
                VisionType::VisionMiss.into(),
            ));

            // commands.entity(avatar_entity).insert(ToBeRemovedMarker);
            // commands
            //     .entity(avatar_data.equipment.weapon)
            //     .insert(ToBeRemovedMarker);
        }

        join_team_events.write(PlayerJoinTeamEvent {
            player_uid: event.uid,
            avatar_guid_list: event.avatar_team_guid_list.clone(),
            appear_avatar_guid: event.cur_avatar_guid,
        });
    }
}

#[instrument(skip_all)]
pub fn notify_avatar_team_update(
    mut events: MessageReader<PlayerAvatarTeamChanged>,
    players: Res<Players>,
    out: Res<MessageOutput>,
) {
    for event in events.read() {
        debug!("{event:?}");

        let Some(player_info) = players.get(event.uid) else {
            continue;
        };

        let Some(ref avatar_bin) = player_info.avatar_bin else {
            continue;
        };
        
        out.send(
            event.uid,
            "AvatarTeamUpdateNotify",
            AvatarTeamUpdateNotify {
                temp_avatar_guid_list: Vec::with_capacity(0),
                avatar_team_map: avatar_bin
                    .team_map
                    .iter()
                    .map(|(idx, team)| {
                        (
                            *idx,
                            AvatarTeam {
                                team_name: team.team_name.clone(),
                                avatar_guid_list: team.avatar_guid_list.clone(),
                            },
                        )
                    })
                    .collect(),
            },
        );
    }
}
