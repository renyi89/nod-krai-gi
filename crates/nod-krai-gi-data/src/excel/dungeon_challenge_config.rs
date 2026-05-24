use std::collections::HashMap;

#[repr(u32)]
#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum ChallengeType {
    #[default]
    #[serde(alias = "CHALLENGE_NONE")]
    ChallengeNone = 0,
    #[serde(alias = "CHALLENGE_KILL_COUNT")]
    ChallengeKillCount = 1,
    #[serde(alias = "CHALLENGE_KILL_COUNT_IN_TIME")]
    ChallengeKillCountInTime = 2,
    #[serde(alias = "CHALLENGE_SURVIVE")]
    ChallengeSurvive = 3,
    #[serde(alias = "CHALLENGE_TIME_FLY")]
    ChallengeTimeFly = 4,
    #[serde(alias = "CHALLENGE_KILL_COUNT_FAST")]
    ChallengeKillCountFast = 5,
    #[serde(alias = "CHALLENGE_KILL_COUNT_FROZEN_LESS")]
    ChallengeKillCountFrozenLess = 6,
    #[serde(alias = "CHALLENGE_KILL_MONSTER_IN_TIME")]
    ChallengeKillMonsterInTime = 7,
    #[serde(alias = "CHALLENGE_TRIGGER_IN_TIME")]
    ChallengeTriggerInTime = 8,
    #[serde(alias = "CHALLENGE_GUARD_HP")]
    ChallengeGuardHp = 9,
    #[serde(alias = "CHALLENGE_KILL_COUNT_GUARD_HP")]
    ChallengeKillCountGuardHp = 10,
    #[serde(alias = "CHALLENGE_TRIGGER_IN_TIME_FLY")]
    ChallengeTriggerInTimeFly = 11,
    #[serde(alias = "CHALLENGE_TRIGGER2_AVOID_TRIGGER1")]
    ChallengeTrigger2AvoidTrigger1 = 12,
    #[serde(alias = "CHALLENGE_FATHER_SUCC_IN_TIME")]
    ChallengeFatherSuccInTime = 13,
    #[serde(alias = "CHALLENGE_MONSTER_DAMAGE_COUNT")]
    ChallengeMonsterDamageCount = 14,
    #[serde(alias = "CHALLENGE_ELEMENT_REACTION_COUNT")]
    ChallengeElementReactionCount = 15,
    #[serde(alias = "CHALLENGE_FREEZE_ENEMY_IN_TIME")]
    ChallengeFreezeEnemyInTime = 16,
    #[serde(alias = "CHALLENGE_CRYSTAL_ELEMENT_REACTION_COUNT")]
    ChallengeCrystalElementReactionCount = 17,
    #[serde(alias = "CHALLENGE_SHEILD_ABSORB_DAMAGE_COUNT")]
    ChallengeShieldAbsorbDamageCount = 18,
    #[serde(alias = "CHALLENGE_ELEMENT_TRIAL")]
    ChallengeElementTrial = 19,
    #[serde(alias = "CHALLENGE_SWIRL_ELEMENT_REACTION_COUNT")]
    ChallengeSwirlElementReactionCount = 20,
    #[serde(alias = "CHALLENGE_DIE_LESS_IN_TIME")]
    ChallengeDieLessInTime = 21,
    #[serde(alias = "CHALLENGE_SURVIVE_IN_TIME")]
    ChallengeSurviveInTime = 22,
    #[serde(alias = "CHALLENGE_TRIGGER_COUNT")]
    ChallengeTriggerCount = 23,
    #[serde(alias = "CHALLENGE_LUA_COUNT")]
    ChallengeLuaCount = 24,
    #[serde(alias = "CHALLENGE_LUA_IN_TIME")]
    ChallengeLuaInTime = 25,
    #[serde(alias = "CHALLENGE_COST_STAMINA")]
    ChallengeCostStamina = 26,
    #[serde(alias = "CHALLENGE_ELEMENT_BALL")]
    ChallengeElementBall = 27,
    #[serde(alias = "CHALLENGE_MOVE_SPEED_TIME")]
    ChallengeMoveSpeedTime = 28,
    #[serde(alias = "CHALLENGE_FALLING_STONE")]
    ChallengeFallingStone = 29,
    #[serde(alias = "CHALLENGE_DASH_TIME")]
    ChallengeDashTime = 30,
    #[serde(alias = "CHALLENGE_JUMP_TIME")]
    ChallengeJumpTime = 31,
    #[serde(alias = "CHALLENGE_CLIMB_TIME")]
    ChallengeClimbTime = 32,
    #[serde(alias = "CHALLENGE_FLY_TIME")]
    ChallengeFlyTime = 33,
    #[serde(alias = "CHALLENGE_SWIM_TIME")]
    ChallengeSwimTime = 34,
    #[serde(alias = "CHALLENGE_GATHER_NUM")]
    ChallengeGatherNum = 35,
    #[serde(alias = "CHALLENGE_COLLECT_ENERGY")]
    ChallengeCollectEnergy = 36,
    #[serde(alias = "CHALLENGE_PAIMON_GAME")]
    ChallengePaimonGame = 37,
    #[serde(alias = "CHALLENGE_TEAM_CHAIN_MONSTER_KILL")]
    ChallengeTeamChainMonsterKill = 38,
    #[serde(alias = "CHALLENGE_ABANDON_REGION_TIME")]
    ChallengeAbandonRegionTime = 39,
    #[serde(alias = "CHALLENGE_MONSTER_DIE_IN_TIME")]
    ChallengeMonsterDieInTime = 40,
    #[serde(alias = "CHALLENGE_KILL_FAST")]
    ChallengeKillFast = 41,
    #[serde(alias = "CHALLENGE_DIE_IN_TIME")]
    ChallengeDieInTime = 42,
    #[serde(alias = "CHALLENGE_KEEP_MONSTER_NUM")]
    ChallengeKeepMonsterNum = 43,
    #[serde(alias = "CHALLENGE_NOT_USE_ULTRA")]
    ChallengeNotUseUltra = 44,
    #[serde(alias = "CHALLENGE_NOT_USE_SKILL")]
    ChallengeNotUseSkill = 45,
    #[serde(alias = "CHALLENGE_NOT_USE_ELEMENTAL_BURST")]
    ChallengeNotUseElementalBurst = 46,
    #[serde(alias = "CHALLENGE_FIGHT_OVER_TIME")]
    ChallengeFightOverTime = 47,
    #[serde(alias = "CHALLENGE_NO_DIE_TIME")]
    ChallengeNoDieTime = 48,
    #[serde(alias = "CHALLENGE_COST_STAMINA_IN_TIME")]
    ChallengeCostStaminaInTime = 49,
    #[serde(alias = "CHALLENGE_OTHERS")]
    ChallengeOthers = 50,
}

#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum ChallengeRecordType {
    #[default]
    #[serde(alias = "CHALLENGE_RECORD_TYPE_NONE")]
    ChallengeRecordTypeNone,
    #[serde(alias = "CHALLENGE_RECORD_TYPE_MAX")]
    ChallengeRecordTypeMax,
    #[serde(alias = "CHALLENGE_RECORD_TYPE_MIN")]
    ChallengeRecordTypeMin,
    #[serde(alias = "CHALLENGE_RECORD_TYPE_IN_TIME")]
    ChallengeRecordTypeInTime,
}

#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum InterruptButtonType {
    #[default]
    #[serde(alias = "INTERRUPT_BUTTON_TYPE_NONE")]
    InterruptButtonTypeNone,
    #[serde(alias = "INTERRUPT_BUTTON_TYPE_STOP")]
    InterruptButtonTypeStop,
    #[serde(alias = "INTERRUPT_BUTTON_TYPE_EXIT")]
    InterruptButtonTypeExit,
    #[serde(alias = "INTERRUPT_BUTTON_TYPE_HOST")]
    InterruptButtonTypeHost,
    #[serde(alias = "INTERRUPT_BUTTON_TYPE_ALL")]
    InterruptButtonTypeAll,
}

#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum SubChallengeBannerType {
    #[default]
    #[serde(alias = "SUBCHALLENGE_BANNER_TYPE_NONE")]
    SubchallengeBannerTypeNone,
    #[serde(alias = "SUBCHALLENGE_BANNER_TYPE_SHOW")]
    SubchallengeBannerTypeShow,
    #[serde(alias = "SUBCHALLENGE_BANNER_TYPE_HIDE_FINAL")]
    SubchallengeBannerTypeHideFinal,
    #[serde(alias = "SUBCHALLENGE_BANNER_TYPE_FAIL")]
    SubchallengeBannerTypeFail,
}

#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum SubChallengeFadeOutType {
    #[default]
    #[serde(alias = "SUBCHALLENGE_FADEOUT_TYPE_NONE")]
    SubchallengeFadeoutTypeNone,
    #[serde(alias = "SUBCHALLENGE_FADEOUT_TYPE_FINISH")]
    SubchallengeFadeoutTypeFinish,
    #[serde(alias = "SUBCHALLENGE_FADEOUT_TYPE_FAIL")]
    SubchallengeFadeoutTypeFail,
    #[serde(alias = "SUBCHALLENGE_FADEOUT_TYPE_ALL")]
    SubchallengeFadeoutTypeAll,
    #[serde(alias = "SUBCHALLENGE_FADEOUT_TYPE_SUCCESS")]
    SubchallengeFadeoutTypeSuccess,
}

#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum SubChallengeAnimType {
    #[default]
    #[serde(alias = "SUB_CHALLENGE_ANIM_TYPE_DEFAULT")]
    SubChallengeAnimTypeDefault,
    #[serde(alias = "SUB_CHALLENGE_ANIM_TYPE_SPECIAL")]
    SubChallengeAnimTypeSpecial,
    #[serde(alias = "SUB_CHALLENGE_ANIM_TYPE_FORBID")]
    SubChallengeAnimTypeForbid,
    #[serde(alias = "SUB_CHALLENGE_ANIM_TYPE_SUCCESS")]
    SubChallengeAnimTypeSuccess,
}

#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum SubChallengeSortType {
    #[default]
    #[serde(alias = "SUB_CHALLENGE_SORT_TYPE_DEFAULT")]
    SubchallengeSortTypeDefault,
    #[serde(alias = "SUB_CHALLENGE_SORT_TYPE_PROGRESS")]
    SubchallengeSortTypeProgress,
    #[serde(alias = "SUB_CHALLENGE_SORT_TYPE_CHALLENGEINDEX")]
    SubchallengeSortTypeChallengeIndex,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DungeonChallengeConfig {
    pub id: u32,
    #[serde(default)]
    pub challenge_type: ChallengeType,
    #[serde(default)]
    pub activity_skill_id: u32,
    #[serde(default)]
    pub icon_path: String,
    #[serde(default)]
    pub is_block_top_timer: bool,
    #[serde(default)]
    pub is_success_when_not_settled: bool,
    #[serde(default)]
    pub is_trans_back_when_interrupt: bool,
    #[serde(default)]
    pub no_fail_hint: bool,
    #[serde(default)]
    pub no_success_hint: bool,
    #[serde(default)]
    pub interrupt_button_type: InterruptButtonType,
    #[serde(default)]
    pub record_type: ChallengeRecordType,
    #[serde(default)]
    pub sub_challenge_banner_rule: SubChallengeBannerType,
    #[serde(default)]
    pub sub_challenge_fade_out_delay_time: u32,
    #[serde(default)]
    pub sub_challenge_fade_out_rule: SubChallengeFadeOutType,
    #[serde(default)]
    pub sub_challenge_fail_anim: SubChallengeAnimType,
    #[serde(default)]
    pub sub_challenge_sort_type: SubChallengeSortType,
    #[serde(default)]
    pub sub_challenge_start_anim: SubChallengeAnimType,
    #[serde(default)]
    pub sub_challenge_success_anim: SubChallengeAnimType,
    #[serde(default)]
    pub progress_text_template_text_map_hash: u32,
    #[serde(default)]
    pub sub_progress_text_template_text_map_hash: u32,
    #[serde(default)]
    pub sub_target_text_template_text_map_hash: u32,
    #[serde(default)]
    pub target_text_template_text_map_hash: u32,
    #[serde(default)]
    pub team_ability_group_list: Vec<String>,
}

pub trait DungeonChallengeConfigKeyed<K> {
    fn key(&self) -> K;
    fn load(excel_bin_output_path: &str) -> HashMap<K, DungeonChallengeConfig>;
}

impl DungeonChallengeConfigKeyed<u32> for DungeonChallengeConfig {
    fn key(&self) -> u32 {
        self.id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, DungeonChallengeConfig> {
        let json = std::fs::read(&format!(
            "{excel_bin_output_path}/DungeonChallengeConfigData.json"
        ))
        .unwrap();
        let list: Vec<DungeonChallengeConfig> = serde_json::from_slice(&*json).unwrap();
        list.iter()
            .filter(|item| item.challenge_type != ChallengeType::ChallengeNone)
            .map(|item| (item.key(), item.clone()))
            .collect()
    }
}
