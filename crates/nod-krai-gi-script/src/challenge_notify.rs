use bevy_ecs::prelude::*;
use nod_krai_gi_event::lua::{ChallengeFinishEvent, ChallengeProgressEvent, ChallengeStartEvent};
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_proto::normal::{
    ChallengeDataNotify, DungeonChallengeBeginNotify, DungeonChallengeFinishNotify,
    DungeonSettleNotify,
};

use crate::challenge_manager::ChallengeManager;

pub fn handle_challenge_start_event(
    mut ev_reader: MessageReader<ChallengeStartEvent>,
    mut challenge_progress_events: MessageWriter<ChallengeProgressEvent>,
    mut challenge_manager: ResMut<ChallengeManager>,
    message_output: Res<MessageOutput>,
) {
    for event in ev_reader.read() {
        let notify = DungeonChallengeBeginNotify {
            challenge_id: event.challenge_id,
            challenge_index: event.challenge_index,
            group_id: event.group_id,
            param_list: event.param_list.clone(),
            ..Default::default()
        };

        tracing::debug!(
            "[ChallengeNotify] Sending DungeonChallengeBeginNotify: challenge_id={}, index={}, group={}",
            event.challenge_id,
            event.challenge_index,
            event.group_id
        );

        message_output.send_to_all("DungeonChallengeBeginNotify", notify);

        match challenge_manager.update_time_by_index(event.challenge_index, 0) {
            Some(progress_event) => {
                challenge_progress_events.write(progress_event);
            }
            None => {}
        }
    }
}

pub fn handle_challenge_finish_event(
    mut ev_reader: MessageReader<ChallengeFinishEvent>,
    message_output: Res<MessageOutput>,
    players: Res<nod_krai_gi_persistence::Players>,
    world_owner_uid: Res<nod_krai_gi_event::scene::WorldOwnerUID>,
) {
    for event in ev_reader.read() {
        let notify = DungeonChallengeFinishNotify {
            challenge_index: event.challenge_index,
            is_success: event.is_success,
            challenge_record_type: 2,
            time_cost: event.time_cost,
            ..Default::default()
        };

        tracing::debug!(
            "[ChallengeNotify] Sending DungeonChallengeFinishNotify: challenge_id={}, index={}, is_success={}, time_cost={}",
            event.challenge_id,
            event.challenge_index,
            event.is_success,
            event.time_cost
        );

        message_output.send_to_all("DungeonChallengeFinishNotify", notify);

        let Some(player_info) = players.get(world_owner_uid.0) else {
            continue;
        };

        let Some(ref player_dungeon_bin) = player_info.dungeon_bin else {
            continue;
        };

        message_output.send_to_all(
            "DungeonSettleNotify",
            DungeonSettleNotify {
                create_player_uid: world_owner_uid.0,
                result: if event.is_success { 1u32 } else { 0u32 },
                use_time: event.time_cost,
                dungeon_id: player_dungeon_bin.cur_dungeon_id,
                is_success: event.is_success,
                ..Default::default()
            },
        );
    }
}

pub fn handle_challenge_progress_event(
    mut ev_reader: MessageReader<ChallengeProgressEvent>,
    message_output: Res<MessageOutput>,
) {
    for event in ev_reader.read() {
        let notify = ChallengeDataNotify {
            challenge_index: event.challenge_index,
            param_index: event.param_index,
            value: event.value,
        };

        tracing::debug!(
            "[ChallengeNotify] Sending ChallengeDataNotify: index={}, type={:?}, param_index={}, value={}",
            event.challenge_index,
            event.challenge_type,
            event.param_index,
            event.value
        );

        message_output.send_to_all("ChallengeDataNotify", notify);
    }
}
