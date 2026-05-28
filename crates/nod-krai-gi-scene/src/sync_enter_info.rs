use bevy_ecs::prelude::*;
use nod_krai_gi_entity::{
    ability::Ability,
    avatar::{AvatarQueryReadOnly, CurrentPlayerAvatarMarker},
    common::{OwnerPlayerUID, ProtocolEntityID},
    mp_level::{AuthorityPeerId, MpLevelEntityMarker},
    play_team::PlayTeamEntityMarker,
    team::TeamEntityMarker,
    weapon::WeaponQueryReadOnly,
};
use nod_krai_gi_event::scene::*;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_persistence::Players;

use crate::common::{PlayerSceneStates, ScenePeerManager};

pub fn sync_enter_info(
    mut scene_init_events: MessageReader<SceneInitFinishEvent>,
    message_output: Res<MessageOutput>,
    player_scene_states: Res<PlayerSceneStates>,
    team_entity_query: Query<(&ProtocolEntityID, &Ability), With<TeamEntityMarker>>,
    mp_level_entity_query: Query<(&ProtocolEntityID, &AuthorityPeerId), With<MpLevelEntityMarker>>,
    avatars: Query<(AvatarQueryReadOnly, Option<&CurrentPlayerAvatarMarker>)>,
    weapons: Query<WeaponQueryReadOnly>,
    players: Res<Players>,
    peer_mgr: Res<ScenePeerManager>,
) {
    for SceneInitFinishEvent(uid) in scene_init_events.read() {
        let uid = *uid;

        let Ok((team_entity_id, team_ability)) = team_entity_query.single() else {
            tracing::error!("team_entity_query.single() returned None");
            continue;
        };

        let Ok((mp_level_entity_id, authority_peer_id)) = mp_level_entity_query.single() else {
            tracing::error!("mp_level_entity_query.single() returned None");
            continue;
        };

        let Some(cur_avatar_entity_id) = avatars
            .iter()
            .find(|(data, cur)| data.owner_player_uid.0 == uid && cur.is_some())
            .map(|(data, _)| data.entity_id.0)
        else {
            tracing::error!("cur_avatar_entity_id None");
            continue;
        };

        let scalar_value = nod_krai_gi_proto::normal::AbilityScalarValueEntry {
            float_value: 100.0,
            key: Some(nod_krai_gi_proto::normal::AbilityString {
                r#type: Some(nod_krai_gi_proto::normal::ability_string::Type::Str(
                    "SGV_PlayerTeam_Phlogiston".to_string(),
                )),
            }),
        };

        let phlogiston_sync_info = nod_krai_gi_proto::normal::AbilitySyncStateInfo {
            sgv_dynamic_value_map: vec![scalar_value],
            ..Default::default()
        };

        let team_enter_info = nod_krai_gi_proto::normal::TeamEnterSceneInfo {
            team_ability_info: Some(phlogiston_sync_info),
            ability_control_block: Some(team_ability.build_control_block()),
            team_entity_id: team_entity_id.0,
            ..Default::default()
        };

        let mp_level_entity_info = nod_krai_gi_proto::normal::MpLevelEntityInfo {
            ability_info: Some(nod_krai_gi_proto::normal::AbilitySyncStateInfo::default()),
            entity_id: mp_level_entity_id.0,
            authority_peer_id: authority_peer_id.0,
        };

        let Some(player_scene_state) = player_scene_states.get(&uid) else {
            continue;
        };

        message_output.send(
            uid,
            "HostPlayerNotify",
            nod_krai_gi_proto::normal::HostPlayerNotify {
                host_peer_id: peer_mgr.get_peer_id_by_uid(uid),
                host_uid: uid,
            },
        );

        message_output.send(
            uid,
            "PlayerEnterSceneInfoNotify",
            nod_krai_gi_proto::normal::PlayerEnterSceneInfoNotify {
                enter_scene_token: player_scene_state.enter_scene_token(),
                cur_avatar_entity_id,
                team_enter_info: Some(team_enter_info),
                mp_level_entity_info: Some(mp_level_entity_info),
                avatar_enter_info: avatars
                    .iter()
                    .filter(|(data, _)| data.owner_player_uid.0 == uid)
                    .filter_map(|(avatar_data, _)| {
                        match weapons.get(avatar_data.avatar_equipment_weapon.0) {
                            Ok(weapon_data) => {
                                Some(nod_krai_gi_proto::normal::AvatarEnterSceneInfo {
                                    avatar_guid: avatar_data.guid.0,
                                    weapon_guid: weapon_data.guid.0,
                                    avatar_entity_id: avatar_data.entity_id.0,
                                    weapon_entity_id: weapon_data.entity_id.0,
                                    avatar_ability_info: Some(
                                        nod_krai_gi_proto::normal::AbilitySyncStateInfo::default(),
                                    ),
                                    weapon_ability_info: Some(
                                        nod_krai_gi_proto::normal::AbilitySyncStateInfo::default(),
                                    ),
                                    buff_id_list: Vec::with_capacity(0),
                                    server_buff_list: Vec::with_capacity(0),
                                })
                            }
                            Err(_) => None,
                        }
                    })
                    .collect(),
            },
        );

        message_output.send(
            uid,
            "ScenePlayerInfoNotify",
            nod_krai_gi_proto::normal::ScenePlayerInfoNotify {
                player_info_list: players
                    .keys()
                    .filter_map(|player_uid| {
                        let Some(player_info) = players.get(*player_uid) else {
                            return None;
                        };

                        let Some(ref player_basic_bin) = player_info.basic_bin else {
                            return None;
                        };

                        let Some(ref player_scene_bin) = player_info.scene_bin else {
                            return None;
                        };

                        Some(nod_krai_gi_proto::normal::ScenePlayerInfo {
                            uid: *player_uid,
                            peer_id: peer_mgr.get_peer_id_by_uid(*player_uid),
                            name: player_basic_bin.nickname.clone(),
                            scene_id: player_scene_bin.my_cur_scene_id,
                            online_player_info: Some(nod_krai_gi_proto::normal::OnlinePlayerInfo {
                                uid: *player_uid,
                                nickname: player_basic_bin.nickname.clone(),
                                player_level: player_basic_bin.level,
                                mp_setting_type:nod_krai_gi_proto::normal::MpSettingType::MpSettingEnterAfterApply as i32,
                                profile_picture:Some(nod_krai_gi_proto::normal::ProfilePicture{
                                    profile_picture_id: player_basic_bin.profile_picture_id,
                                    profile_frame_id: player_basic_bin.profile_frame_id,
                                    avatar_id: 0,
                                    costume_id: 0,
                                }),
                                ..Default::default()
                            }),
                            is_connected: false,
                        })
                    })
                    .collect(),
            },
        );
    }
}

pub fn sync_play_team_entity(
    mut scene_init_events: MessageReader<SceneInitFinishEvent>,
    message_output: Res<MessageOutput>,
    play_team_entities: Query<(&OwnerPlayerUID, &ProtocolEntityID), With<PlayTeamEntityMarker>>,
    players: Res<Players>,
    peer_mgr: Res<ScenePeerManager>,
) {
    for SceneInitFinishEvent(uid) in scene_init_events.read() {
        let Some(player_info) = players.get(*uid) else {
            continue;
        };

        let Some(ref player_scene_bin) = player_info.scene_bin else {
            continue;
        };

        message_output.send(
            *uid,
            "SyncTeamEntityNotify",
            nod_krai_gi_proto::normal::SyncTeamEntityNotify {
                scene_id: player_scene_bin.my_cur_scene_id,
                team_entity_info_list: vec![],
            },
        );

        message_output.send(
            *uid,
            "SyncScenePlayTeamEntityNotify",
            nod_krai_gi_proto::normal::SyncScenePlayTeamEntityNotify {
                scene_id: player_scene_bin.my_cur_scene_id,
                entity_info_list: play_team_entities
                    .iter()
                    .map(
                        |(owner_uid, id)| nod_krai_gi_proto::normal::PlayTeamEntityInfo {
                            entity_id: id.0,
                            player_uid: owner_uid.0,
                            authority_peer_id: peer_mgr.get_peer_id_by_uid(owner_uid.0),
                            gadget_config_id: 0,
                            ability_info: Some(
                                nod_krai_gi_proto::normal::AbilitySyncStateInfo::default(),
                            ),
                        },
                    )
                    .collect(),
            },
        );
    }
}
