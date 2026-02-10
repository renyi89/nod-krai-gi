use avatar::{change_avatar, notify_avatar_team_update, replace_avatar_team, set_up_avatar_team};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ::common::player_cache::cache_set_is_tp;
use ::common::time_util::unix_timestamp_ms;
use common::{PlayerSceneState, PlayerSceneStates, ScenePeerManager, WorldOwnerUID};
use enter::EnterSceneStateSystems;
use nod_krai_gi_data::excel::{SceneTagConfig, SceneTagConfigKeyed};
use nod_krai_gi_entity::avatar::{CurrentPlayerAvatarMarker, CurrentTeam};
use nod_krai_gi_entity::common::Visible;
use nod_krai_gi_entity::{
    ability::Ability,
    avatar::AvatarQueryReadOnly,
    common::{EntityCounter, OwnerPlayerUID},
    mp_level::{AuthorityPeerId, MpLevelBundle, MpLevelEntityMarker},
    play_team::{PlayTeamEntityBundle, PlayTeamEntityMarker},
    team::{TeamEntityBundle, TeamEntityMarker},
    util::to_protocol_entity_id,
    EntityDisappearEvent,
};
use nod_krai_gi_event::scene::*;
use nod_krai_gi_message::get_player_version;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::dy_parser::{replace_out_u32, replace_out_u64};
use nod_krai_gi_proto::normal::{EnterType, ProtEntityType, VisionType};
use std::sync::Arc;

mod avatar;
mod enter;
mod player_join_team;
mod player_jump;
mod scene_team_update;
mod sync_enter_info;

pub mod common;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnterSceneStateSystems>()
            .insert_resource(WorldOwnerUID(0))
            .insert_resource(PlayerSceneStates::default())
            .insert_resource(ScenePeerManager::default())
            .add_systems(PostStartup, init_scene)
            .add_systems(PreUpdate, enter::handle_enter_scene_state_change)
            .add_systems(PreUpdate, (set_up_avatar_team, replace_avatar_team).chain())
            .add_systems(PreUpdate, change_avatar)
            .add_systems(Update, player_join_team::player_join_team)
            .add_systems(Update, player_jump::player_jump)
            .add_systems(Update, player_jump::player_jump_by_point)
            .add_systems(
                PostUpdate,
                (
                    begin_enter_scene,
                    create_play_team_entity,
                    notify_player_enter_scene,
                )
                    .chain(),
            )
            .add_systems(
                PostUpdate,
                (
                    sync_enter_info::sync_enter_info,
                    sync_enter_info::sync_play_team_entity,
                    scene_team_update::notify_scene_team_update,
                    enter::scene_init_finish_send_rsp,
                    enter::enter_scene_done_send_rsp,
                    notify_avatar_team_update,
                )
                    .chain(),
            );
    }
}

fn init_scene(
    mut commands: Commands,
    mut players: ResMut<Players>,
    mut entity_counter: ResMut<EntityCounter>,
    mut enter_events: MessageWriter<BeginEnterSceneEvent>,
) {
    commands.spawn(TeamEntityBundle {
        marker: TeamEntityMarker,
        entity_id: to_protocol_entity_id(ProtEntityType::ProtEntityTeam, entity_counter.inc()),
        ability: Ability::new_for_team(),
        instanced_abilities: Default::default(),
        instanced_modifiers: Default::default(),
        global_ability_values: Default::default(),
    });

    commands.spawn(MpLevelBundle {
        authority_peer_id: AuthorityPeerId(1),
        entity_id: to_protocol_entity_id(ProtEntityType::ProtEntityMpLevel, entity_counter.inc()),
        marker: MpLevelEntityMarker,
    });

    let mut uids = vec![];
    for uid in players.keys() {
        uids.push(uid.clone());
    }

    for uid in uids {
        let Some(player_info) = players.get_mut(uid) else {
            continue;
        };
        let Some(ref mut scene_bin) = player_info.scene_bin else {
            continue;
        };

        if scene_bin.my_cur_scene_id == 0 {
            scene_bin.my_cur_scene_id = 3;
        }

        enter_events.write(BeginEnterSceneEvent {
            uid,
            scene_id: scene_bin.my_cur_scene_id,
            enter_type: EnterType::EnterSelf,
            enter_reason: EnterReason::Login,
            position: scene_bin.my_prev_pos.unwrap_or_default().into(),
        });
    }
}

fn begin_enter_scene(
    mut events: MessageReader<BeginEnterSceneEvent>,
    mut commands: Commands,
    mut player_scene_states: ResMut<PlayerSceneStates>,
    player_avatar_entities: Query<(Entity, AvatarQueryReadOnly)>,
    mut disappear_events: MessageWriter<EntityDisappearEvent>,
) {
    for event in events.read() {
        cache_set_is_tp(event.uid, true);
        for (avatar_entity, avatar_data) in player_avatar_entities
            .iter()
            .filter(|(_, data)| data.owner_player_uid.0 == event.uid)
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

            // commands.entity(entity).insert(ToBeRemovedMarker);
            // commands
            //     .entity(avatar_data.equipment.weapon)
            //     .insert(ToBeRemovedMarker);
        }

        player_scene_states.insert(event.uid, PlayerSceneState::new());
    }
}

fn create_play_team_entity(
    mut events: MessageReader<BeginEnterSceneEvent>,
    mut commands: Commands,
    mut entity_counter: ResMut<EntityCounter>,
    play_team_entities: Query<(Entity, &OwnerPlayerUID), With<PlayTeamEntityMarker>>,
) {
    for event in events.read() {
        if !play_team_entities
            .iter()
            .any(|(_, owner_uid)| owner_uid.0 == event.uid)
        {
            commands.spawn(PlayTeamEntityBundle {
                player_uid: OwnerPlayerUID(event.uid),
                entity_id: to_protocol_entity_id(
                    ProtEntityType::ProtEntityPlayTeamEntity,
                    entity_counter.inc(),
                ),
                ability: Ability::default(),
                marker: PlayTeamEntityMarker,
            });
        }
    }
}

fn notify_player_enter_scene(
    mut events: MessageReader<BeginEnterSceneEvent>,
    message_output: Res<MessageOutput>,
    player_scene_states: Res<PlayerSceneStates>,
) {
    for event in events.read() {
        let version = get_player_version!(&event.uid);
        let protocol_version = version.as_str();
        let Some(player_scene_state) = player_scene_states.get(&event.uid) else {
            continue;
        };

        let enter_scene_token = player_scene_state.enter_scene_token();
        tracing::debug!("Player enter scene: {:?}", enter_scene_token);
        let mut scene_tag_id_list = vec![];
        if [3, 5, 6, 7, 11, 101].contains(&event.scene_id) {
            let scene_tag_entries_clone = Arc::clone(SceneTagConfig::get_scene_tag_entries());
            match scene_tag_entries_clone.get(&event.scene_id) {
                None => {}
                Some(scene_tag_list) => {
                    scene_tag_list.iter().for_each(|item| {
                        scene_tag_id_list.push(item.id);
                    });
                }
            }
        }
        message_output.send(
            event.uid,
            "PlayerEnterSceneNotify",
            nod_krai_gi_proto::normal::PlayerEnterSceneNotify {
                scene_id: replace_out_u32(
                    protocol_version,
                    "PlayerEnterSceneNotify.scene_id",
                    event.scene_id,
                ),
                enter_scene_token: replace_out_u32(
                    protocol_version,
                    "PlayerEnterSceneNotify.enter_scene_token",
                    enter_scene_token,
                ),
                target_uid: replace_out_u32(
                    protocol_version,
                    "PlayerEnterSceneNotify.target_uid",
                    event.uid,
                ),
                pos: Some(event.position.into()),
                prev_pos: Some(Default::default()),
                scene_transaction: format!(
                    "{}-{}-{}-{}",
                    event.scene_id,
                    event.uid,
                    ::common::time_util::unix_timestamp(),
                    179398
                ),
                r#type: event.enter_type.into(),
                world_level: replace_out_u32(
                    protocol_version,
                    "PlayerEnterSceneNotify.world_level",
                    9,
                ),
                enter_reason: replace_out_u32(
                    protocol_version,
                    "PlayerEnterSceneNotify.enter_reason",
                    event.enter_reason as u32,
                ),
                scene_begin_time: replace_out_u64(
                    protocol_version,
                    "PlayerEnterSceneNotify.scene_begin_time",
                    unix_timestamp_ms(),
                ),
                scene_tag_id_list,
                ..Default::default()
            },
        );
    }
}
