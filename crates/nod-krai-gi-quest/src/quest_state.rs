use common::time_util::unix_timestamp;
use nod_krai_gi_data::excel::common::QuestState;
use nod_krai_gi_proto::server_only::{
    ParentQuestBin, PlayerParentQuestBin, PlayerQuestBin, PlayerQuestCompBin, QuestBin,
    Uint32PairBin,
};

/// 检查任务状态是否可以转换
pub fn can_transition_to(current: QuestState, target: QuestState) -> bool {
    match current {
        s if s == QuestState::None => target == QuestState::Unstarted,
        s if s == QuestState::Unstarted => {
            target == QuestState::Unfinished || target == QuestState::Failed
        }
        s if s == QuestState::Unfinished => {
            target == QuestState::Finished || target == QuestState::Failed
        }
        s if s == QuestState::Finished => {
            // 已完成的任务可以回滚到未完成（GM命令等场景）
            target == QuestState::Unfinished
        }
        s if s == QuestState::Failed => {
            // 失败的任务可以重新开始
            target == QuestState::Unfinished || target == QuestState::Unstarted
        }
        _ => false,
    }
}

/// 获取状态转换的合法性描述
pub fn transition_reason(current: QuestState, target: QuestState) -> &'static str {
    if can_transition_to(current, target) {
        return "valid";
    }
    match current {
        s if s == QuestState::None => "cannot transition from None",
        s if s == QuestState::Unstarted => "cannot transition from Unstarted to target state",
        s if s == QuestState::Unfinished => "cannot transition from Unfinished to target state",
        s if s == QuestState::Finished => {
            "cannot transition from Finished (except to Unfinished via GM)"
        }
        s if s == QuestState::Failed => "cannot transition from Failed",
        _ => "unknown state",
    }
}

/// 确保玩家任务组件存在
pub fn ensure_player_quest_comp(
    player_data: &mut nod_krai_gi_proto::server_only::PlayerDataBin,
) -> &mut PlayerQuestCompBin {
    if player_data.quest_bin.is_none() {
        player_data.quest_bin = Some(PlayerQuestCompBin {
            quest_bin: Some(PlayerQuestBin {
                quest_map: Default::default(),
            }),
            parent_quest_bin: Some(PlayerParentQuestBin {
                parent_quest_map: Default::default(),
            }),
            ..Default::default()
        });
    }
    player_data
        .quest_bin
        .as_mut()
        .expect("quest_bin should be initialized above")
}

/// 确保父任务存在
pub fn ensure_parent_quest(
    player_quest_bin: &mut PlayerQuestCompBin,
    parent_quest_id: u32,
) -> &mut ParentQuestBin {
    let parent_quest_bin =
        player_quest_bin
            .parent_quest_bin
            .get_or_insert_with(|| PlayerParentQuestBin {
                parent_quest_map: Default::default(),
            });

    parent_quest_bin
        .parent_quest_map
        .entry(parent_quest_id)
        .or_insert_with(|| ParentQuestBin {
            parent_quest_id,
            accept_time: unix_timestamp() as u32,
            ..Default::default()
        })
}

/// 获取父任务（如果不存在返回 None）
pub fn get_parent_quest(
    player_quest_bin: &PlayerQuestCompBin,
    parent_quest_id: u32,
) -> Option<&ParentQuestBin> {
    player_quest_bin
        .parent_quest_bin
        .as_ref()
        .and_then(|bin| bin.parent_quest_map.get(&parent_quest_id))
}

/// 获取父任务的可变引用（如果不存在返回 None）
pub fn get_parent_quest_mut(
    player_quest_bin: &mut PlayerQuestCompBin,
    parent_quest_id: u32,
) -> Option<&mut ParentQuestBin> {
    player_quest_bin
        .parent_quest_bin
        .as_mut()
        .and_then(|bin| bin.parent_quest_map.get_mut(&parent_quest_id))
}

/// 更新子任务在父任务中的状态
pub fn update_child_quest_state_in_parent(
    parent_quest_bin: &mut ParentQuestBin,
    quest_id: u32,
    state: u32,
) {
    let mut found = false;
    for pair in &mut parent_quest_bin.child_quest_state_list {
        if pair.key == quest_id {
            pair.value = state;
            found = true;
            break;
        }
    }
    if !found {
        parent_quest_bin.child_quest_state_list.push(Uint32PairBin {
            key: quest_id,
            value: state,
        });
    }
}

/// 从父任务中移除子任务状态
pub fn remove_child_quest_state_from_parent(parent_quest_bin: &mut ParentQuestBin, quest_id: u32) {
    parent_quest_bin
        .child_quest_state_list
        .retain(|pair| pair.key != quest_id);
}

/// 检查父任务是否完成
pub fn check_parent_quest_finished(parent_quest_bin: &mut ParentQuestBin) -> bool {
    let all_finished = !parent_quest_bin.child_quest_state_list.is_empty()
        && parent_quest_bin
            .child_quest_state_list
            .iter()
            .all(|pair| pair.value == QuestState::Finished as u32);

    if all_finished && parent_quest_bin.state == QuestState::None as u32 {
        parent_quest_bin.state = QuestState::Finished as u32;
        parent_quest_bin.total_finish_count += 1;
        parent_quest_bin.last_finish_time = unix_timestamp() as u32;
        return true;
    }
    false
}

/// 检查父任务是否失败
pub fn check_parent_quest_failed(parent_quest_bin: &mut ParentQuestBin) -> bool {
    let any_failed = parent_quest_bin
        .child_quest_state_list
        .iter()
        .any(|pair| pair.value == QuestState::Failed as u32);

    if any_failed && parent_quest_bin.state == QuestState::None as u32 {
        parent_quest_bin.state = QuestState::Failed as u32;
        return true;
    }
    false
}

/// 重置父任务状态
pub fn reset_parent_quest_state(parent_quest_bin: &mut ParentQuestBin) {
    parent_quest_bin.state = QuestState::None as u32;
}

/// 创建任务 Bin
///
/// C++ 行为：
/// - onAccept: 设置 accept_time，状态为 QUEST_STATE_UNSTARTED
/// - onStart: 设置 start_time 和 start_game_time，状态变为 QUEST_STATE_UNFINISHED
///
/// Rust 版本简化为直接创建 Unfinished 状态的任务，
/// accept_time 和 start_time 都设为当前时间。
pub fn create_quest_bin(
    quest_id: u32,
    parent_quest_id: u32,
    state: u32,
    finish_cond_count: usize,
    fail_cond_count: usize,
) -> QuestBin {
    let now = unix_timestamp() as u32;
    let is_started = state != QuestState::Unstarted as u32;
    QuestBin {
        quest_id,
        parent_quest_id,
        state,
        start_time: if is_started { now } else { 0 },
        accept_time: now, // accept_time 在任务被接受时即设置
        start_game_time: if is_started { now } else { 0 },
        finish_progress_list: if finish_cond_count == 0 {
            vec![]
        } else {
            vec![0u32; finish_cond_count]
        },
        fail_progress_list: if fail_cond_count == 0 {
            vec![]
        } else {
            vec![0u32; fail_cond_count]
        },
        ..Default::default()
    }
}

/// 重置任务进度
pub fn reset_quest_progress(quest_bin: &mut QuestBin) {
    for progress in quest_bin.finish_progress_list.iter_mut() {
        *progress = 0;
    }
    for progress in quest_bin.fail_progress_list.iter_mut() {
        *progress = 0;
    }
    let now = unix_timestamp() as u32;
    quest_bin.start_time = now;
    quest_bin.start_game_time = now;
}

/// 设置任务状态（带状态转换检查）
pub fn set_quest_state(
    quest_bin: &mut QuestBin,
    new_state: QuestState,
) -> Result<(), &'static str> {
    let current_state = QuestState::from(quest_bin.state);
    if can_transition_to(current_state, new_state) {
        quest_bin.state = new_state as u32;
        Ok(())
    } else {
        Err(transition_reason(current_state, new_state))
    }
}

/// 设置任务失败
pub fn fail_quest(quest_bin: &mut QuestBin) -> Result<(), &'static str> {
    set_quest_state(quest_bin, QuestState::Failed)
}

/// 设置任务完成
pub fn finish_quest(quest_bin: &mut QuestBin) -> Result<(), &'static str> {
    set_quest_state(quest_bin, QuestState::Finished)
}

/// 设置任务开始
pub fn start_quest(quest_bin: &mut QuestBin) -> Result<(), &'static str> {
    set_quest_state(quest_bin, QuestState::Unfinished)
}

/// 检查任务是否处于活动状态（未完成）
pub fn is_quest_active(quest_bin: &QuestBin) -> bool {
    quest_bin.state == QuestState::Unfinished as u32
}

/// 检查任务是否已完成
pub fn is_quest_finished(quest_bin: &QuestBin) -> bool {
    quest_bin.state == QuestState::Finished as u32
}

/// 检查任务是否已失败
pub fn is_quest_failed(quest_bin: &QuestBin) -> bool {
    quest_bin.state == QuestState::Failed as u32
}

/// 更新任务失败进度
pub fn update_fail_progress(
    quest_bin: &mut QuestBin,
    cond_index: usize,
    add_progress: u32,
    max_count: u32,
) -> bool {
    if cond_index >= quest_bin.fail_progress_list.len() {
        return false;
    }

    let current = quest_bin.fail_progress_list[cond_index];
    quest_bin.fail_progress_list[cond_index] = current.saturating_add(add_progress);

    // 检查是否达到失败条件
    quest_bin.fail_progress_list[cond_index] >= max_count
}

/// 检查所有失败条件是否满足
pub fn check_all_fail_conditions(quest_bin: &QuestBin, fail_counts: &[u32]) -> bool {
    if quest_bin.fail_progress_list.is_empty() {
        return false;
    }

    quest_bin
        .fail_progress_list
        .iter()
        .zip(fail_counts.iter())
        .all(|(progress, count)| progress >= count)
}

/// 检查所有完成条件是否满足
pub fn check_all_finish_conditions(quest_bin: &QuestBin, finish_counts: &[u32]) -> bool {
    if quest_bin.finish_progress_list.is_empty() {
        return true;
    }

    quest_bin
        .finish_progress_list
        .iter()
        .zip(finish_counts.iter())
        .all(|(progress, count)| progress >= count)
}
