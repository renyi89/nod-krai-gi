mod enter_scene_done;
mod enter_scene_ready;
mod post_enter_scene;
mod scene_init_finish;

pub use enter_scene_done::EnterSceneDoneEvent;
pub use enter_scene_ready::EnterSceneReadyEvent;
pub use post_enter_scene::PostEnterSceneEvent;
pub use scene_init_finish::SceneInitFinishEvent;

pub use enter_scene_done::enter_scene_done_send_rsp;
pub use scene_init_finish::scene_init_finish_send_rsp;

use crate::common::PlayerSceneStates;

use bevy_ecs::{prelude::*, system::SystemId};
use nod_krai_gi_message::event::ClientMessageEvent;
use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub enum EnterSceneState {
    Ready,
    InitFinish,
    Done,
    Post,
}

#[derive(Resource)]
pub struct EnterSceneStateSystems(HashMap<EnterSceneState, SystemId>);

impl FromWorld for EnterSceneStateSystems {
    fn from_world(world: &mut World) -> Self {
        let mut systems = HashMap::new();

        systems.insert(
            EnterSceneState::Ready,
            world.register_system(enter_scene_ready::on_enter_scene_ready),
        );

        systems.insert(
            EnterSceneState::InitFinish,
            world.register_system(scene_init_finish::on_scene_init_finish),
        );

        systems.insert(
            EnterSceneState::Done,
            world.register_system(enter_scene_done::on_enter_scene_done),
        );

        systems.insert(
            EnterSceneState::Post,
            world.register_system(post_enter_scene::on_post_enter_scene),
        );

        Self(systems)
    }
}

pub fn handle_enter_scene_state_change(
    mut messages: MessageReader<ClientMessageEvent>,
    mut commands: Commands,
    systems: Res<EnterSceneStateSystems>,
    mut player_scene_states: ResMut<PlayerSceneStates>,
    mut ready_events: MessageWriter<EnterSceneReadyEvent>,
    mut init_finish_events: MessageWriter<SceneInitFinishEvent>,
    mut done_events: MessageWriter<EnterSceneDoneEvent>,
    mut post_events: MessageWriter<PostEnterSceneEvent>,
) {
    for msg in messages.read() {
        let next_enter_state;
        match msg.message_name() {
            "EnterSceneReadyReq" => next_enter_state = EnterSceneState::Ready,
            "SceneInitFinishReq" => next_enter_state = EnterSceneState::InitFinish,
            "EnterSceneDoneReq" => next_enter_state = EnterSceneState::Done,
            "PostEnterSceneReq" => next_enter_state = EnterSceneState::Post,
            &_ => {
                continue;
            }
        }

        let player_scene_state = player_scene_states.get_mut(&msg.sender_uid()).unwrap();
        let prev_enter_state = player_scene_state.enter_state();

        if player_scene_state.change_enter_state(next_enter_state) {
            tracing::debug!(
                "EnterScene: changing state {:?} -> {:?}",
                prev_enter_state, next_enter_state
            );

            let uid = msg.sender_uid();
            match next_enter_state {
                EnterSceneState::Ready => {
                    ready_events.write(EnterSceneReadyEvent(uid));
                }
                EnterSceneState::InitFinish => {
                    init_finish_events.write(SceneInitFinishEvent(uid));
                }
                EnterSceneState::Done => {
                    done_events.write(EnterSceneDoneEvent(uid));
                }
                EnterSceneState::Post => {
                    post_events.write(PostEnterSceneEvent(uid));
                }
            }

            commands.run_system(*systems.0.get(&next_enter_state).unwrap());
        } else {
            tracing::debug!(
                "EnterScene: state transition not allowed: {:?} -> {:?}",
                prev_enter_state, next_enter_state
            );
        }
    }
}
