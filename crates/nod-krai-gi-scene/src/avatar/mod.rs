use crate::player_join_team::PlayerJoinTeamEvent;
use bevy_ecs::prelude::*;
use nod_krai_gi_data::prop_type::FightPropType;
use nod_krai_gi_entity::avatar::{CurrentTeam, ReplaceCurrentPlayerAvatarMarker};
use nod_krai_gi_entity::common::{FightProperties, LifeState, Visible};
use nod_krai_gi_entity::transform::Transform;
use nod_krai_gi_entity::{
    avatar::{AvatarQueryReadOnly, CurrentPlayerAvatarMarker},
    EntityDisappearEvent, EntityPropertySeparateUpdateEvent,
};
use nod_krai_gi_message::{event::ClientMessageEvent, output::MessageOutput, USER_VERSION};
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::dy_parser::{
    replace_in_u32, replace_in_u64, replace_out_i32, replace_out_u32, replace_out_u64,
};
use nod_krai_gi_proto::{
    retcode::Retcode, AvatarDieAnimationEndReq, AvatarDieAnimationEndRsp, AvatarTeam,
    AvatarTeamUpdateNotify, ChangeAvatarReq, ChangeAvatarRsp, SetUpAvatarTeamReq,
    SetUpAvatarTeamRsp, VisionType,
};
use std::collections::HashSet;
use tracing::{debug, instrument};

#[derive(Message, Debug)]
pub struct PlayerAvatarTeamChanged {
    pub uid: u32,
    pub avatar_team_guid_list: Vec<u64>,
    pub cur_avatar_guid: u64,
}

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
                    let player = players.get_mut(message.sender_uid());
                    let team = match player
                        .avatar_module
                        .team_map
                        .get(&player.avatar_module.cur_avatar_team_id)
                    {
                        None => continue,
                        Some(team) => team,
                    };
                    let mut all_dead = true;

                    let mut is_first_avatar = true;
                    for (avatar_entity, _, life_state, avatar_data, _) in
                        avatars.iter().filter(|(_, _, _, a, _)| {
                            a.owner_player_uid.0 == message.sender_uid()
                                && team.avatar_guid_list.contains(&a.guid.0)
                        })
                    {
                        if *life_state == LifeState::Alive && is_first_avatar {
                            all_dead = false;
                            is_first_avatar = false;
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

                    debug!("all_dead:{}", all_dead);
                    if all_dead {
                        let mut is_first_avatar = true;
                        for (avatar_entity, fight_props, _, avatar_data, _) in
                            avatars.iter().filter(|(_, _, _, a, _)| {
                                a.owner_player_uid.0 == message.sender_uid()
                                    && team.avatar_guid_list.contains(&a.guid.0)
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
                            if is_first_avatar {
                                is_first_avatar = false;

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

                    let player = players.get_mut(message.sender_uid());

                    let binding = USER_VERSION.get().unwrap().get(&player.uid).unwrap();
                    let protocol_version = binding.as_str();

                    let team_id = replace_in_u32(
                        protocol_version,
                        "SetUpAvatarTeamReq.team_id",
                        request.team_id,
                    );

                    if let Some(team) = player.avatar_module.team_map.get_mut(&team_id) {
                        let mut cur_avatar_guid = replace_in_u64(
                            protocol_version,
                            "SetUpAvatarTeamReq.cur_avatar_guid",
                            request.cur_avatar_guid,
                        );

                        if !request.avatar_team_guid_list.contains(&cur_avatar_guid) {
                            cur_avatar_guid =
                                request.avatar_team_guid_list.first().unwrap().clone();
                        }

                        team.avatar_guid_list = request.avatar_team_guid_list.clone();

                        change_events.write(PlayerAvatarTeamChanged {
                            uid: message.sender_uid(),
                            avatar_team_guid_list: request.avatar_team_guid_list.clone(),
                            cur_avatar_guid,
                        });

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

        let player = players.get(event.uid);

        out.send(
            event.uid,
            "AvatarTeamUpdateNotify",
            AvatarTeamUpdateNotify {
                temp_avatar_guid_list: Vec::with_capacity(0),
                avatar_team_map: player
                    .avatar_module
                    .team_map
                    .iter()
                    .map(|(idx, team)| {
                        (
                            *idx,
                            AvatarTeam {
                                team_name: team.name.clone(),
                                avatar_guid_list: team.avatar_guid_list.clone(),
                            },
                        )
                    })
                    .collect(),
            },
        );
    }
}
