use crate::script_lib::ScriptLib;
use mlua::{Table, UserData, UserDataMethods};
use nod_krai_gi_proto::dy_parser::get_ty_value_by_version;
use std::sync::Arc;

pub struct LuaScriptLibHandle {
    pub script_lib: Arc<dyn ScriptLib>,
    pub protocol_version: String,
}

impl UserData for LuaScriptLibHandle {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "ActivateDungeonCheckPoint",
            |_, _this, (_ctx, _point_id): (Table, u32)| {
                tracing::debug!("ActivateDungeonCheckPoint called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ActivateGroupLinkBundle",
            |_, _this, (_ctx, _group_id): (Table, u32)| {
                tracing::debug!("ActivateGroupLinkBundle called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ActivateGroupLinkBundleByBundleId",
            |_, _this, (_ctx, _bundle_id): (Table, u32)| {
                tracing::debug!("ActivateGroupLinkBundleByBundleId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ActiveChallenge",
            |_,
             this,
             (ctx, source_name, challenge_id, param1, param2, param3, param4): (
                Table,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            )| {
                let group_id: u32 = ctx.get("group_id").unwrap_or(0);
                tracing::debug!(
                    "[ScriptLib] ActiveChallenge group={} source={} challenge={} params=({},{},{},{})",
                    group_id,
                    source_name,
                    challenge_id,
                    param1,
                    param2,
                    param3,
                    param4
                );
                this.script_lib.active_challenge(
                    group_id,
                    source_name,
                    challenge_id,
                    0,
                    param1,
                    param2,
                    param3,
                    param4,
                );
                Ok(0)
            },
        );

        methods.add_method(
            "ActiveGadgetItemGiving",
            |_, _this, (_ctx, _giving_id, _group_id, _config_id): (Table, u32, u32, u32)| {
                tracing::debug!("ActiveGadgetItemGiving called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddBlossomScheduleProgressByGroupId",
            |_, _this, (_ctx, _group_id): (Table, u32)| {
                tracing::debug!("AddBlossomScheduleProgressByGroupId called");
                Ok(-1)
            },
        );

        methods.add_method("AddChallengeDuration", |_, _this, (_ctx, _challenge_index, _delta, _can_exceed_limit): (Table, u32, i32, bool)| {
            tracing::debug!("AddChallengeDuration called");
            Ok(-1)
        });

        methods.add_method(
            "AddChallengeProgress",
            |_, this, (ctx, challenge_index, count): (Table, u32, u32)| {
                let group_id: u32 = ctx.get("group_id").unwrap_or(0);
                tracing::debug!(
                    "[ScriptLib] AddChallengeProgress group={} index={} count={}",
                    group_id,
                    challenge_index,
                    count
                );
                this.script_lib
                    .add_challenge_progress(group_id, challenge_index, count);
                Ok(0)
            },
        );

        methods.add_method("AddChessBuildingPoints", |_, _this, (_ctx, _group_id, _play_index, _player_uid, _delta): (Table, u32, u32, u32, i32)| {
            tracing::debug!("AddChessBuildingPoints called");
            Ok(-1)
        });

        methods.add_method(
            "AddEntityGlobalFloatValueByConfigId",
            |_, _this, (_ctx, _param_table, _key, _delta): (Table, Table, String, f32)| {
                tracing::debug!("AddEntityGlobalFloatValueByConfigId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddEntityGlobalFloatValueByEntityId",
            |_, _this, (_ctx, _param_table, _key, _delta): (Table, Table, String, f32)| {
                tracing::debug!("AddEntityGlobalFloatValueByEntityId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddExhibitionAccumulableData",
            |_, _this, (_ctx, _uid, _key_name, _value): (Table, u32, String, u32)| {
                tracing::debug!("AddExhibitionAccumulableData called");
                Ok(-1)
            },
        );

        methods.add_method("AddExhibitionAccumulableDataAfterSuccess", |_, _this, (_ctx, _uid, _key_name, _value, _play_param): (Table, u32, String, u32, Table)| {
            tracing::debug!("AddExhibitionAccumulableDataAfterSuccess called");
            Ok(-1)
        });

        methods.add_method(
            "AddExhibitionAccumulableDataById",
            |_, _this, (_ctx, _uid, _exhibition_id, _value): (Table, u32, u32, u32)| {
                tracing::debug!("AddExhibitionAccumulableDataById called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddExhibitionReplaceableData",
            |_, _this, (_ctx, _uid, _key_name, _value): (Table, u32, String, u32)| {
                tracing::debug!("AddExhibitionReplaceableData called");
                Ok(-1)
            },
        );

        methods.add_method("AddExhibitionReplaceableDataAfterSuccess", |_, _this, (_ctx, _uid, _key_name, _value, _play_param): (Table, u32, String, u32, Table)| {
            tracing::debug!("AddExhibitionReplaceableDataAfterSuccess called");
            Ok(-1)
        });

        methods.add_method(
            "AddExhibitionReplaceableDataById",
            |_, _this, (_ctx, _uid, _exhibition_id, _value): (Table, u32, u32, u32)| {
                tracing::debug!("AddExhibitionReplaceableDataById called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddExtraFlowSuite",
            |_, _this, (_ctx, _group_id, _suite_index, _policy): (Table, u32, u32, u32)| {
                tracing::debug!("AddExtraFlowSuite called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddExtraGroupSuite",
            |_, _this, (_ctx, group_id, suite_index): (Table, u32, u32)| {
                tracing::debug!("AddExtraGroupSuite called");
                _this
                    .script_lib
                    .add_extra_group_suite(_ctx, group_id, suite_index);
                Ok(0)
            },
        );

        methods.add_method("AddFleurFairMultistagePlayBuffEnergy", |_, _this, (_ctx, _group_id, _play_index, _uid, _value): (Table, u32, u32, u32, i32)| {
            tracing::debug!("AddFleurFairMultistagePlayBuffEnergy called");
            Ok(-1)
        });

        methods.add_method(
            "AddGadgetPlayProgress",
            |_, _this, (_ctx, _group_id, _config_id, _add_progress): (Table, u32, u32, i32)| {
                tracing::debug!("AddGadgetPlayProgress called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddGalleryProgressScore",
            |_, _this, (_ctx, _key, _gallery_id, _score): (Table, String, u32, i32)| {
                tracing::debug!("AddGalleryProgressScore called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddIrodoriChessBuildingPoints",
            |_, _this, (_ctx, _group_id, _play_index, _delta): (Table, u32, u32, i32)| {
                tracing::debug!("AddIrodoriChessBuildingPoints called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddIrodoriChessTowerServerGlobalValue",
            |_,
             _this,
             (_ctx, _main_group_id, _play_index, _gadget_id, _server_global_value_table): (
                Table,
                u32,
                u32,
                u32,
                Table,
            )| {
                tracing::debug!("AddIrodoriChessTowerServerGlobalValue called");
                Ok(-1)
            },
        );

        methods.add_method("AddMechanicusBuildingPoints", |_, _this, (_ctx, _group_id, _play_index, _player_uid, _delta): (Table, u32, u32, u32, i32)| {
            tracing::debug!("AddMechanicusBuildingPoints called");
            Ok(-1)
        });

        methods.add_method(
            "AddPlayerGroupVisionType",
            |_, _this, (_ctx, _uid_list, _type_list): (Table, Table, Table)| {
                tracing::debug!("AddPlayerGroupVisionType called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddQuestProgress",
            |_, this, (ctx, quest_param): (Table, String)| {
                let group_id: u32 = ctx.get("group_id").unwrap_or(0);
                tracing::debug!(
                    "[ScriptLib] AddQuestProgress group={} param={}",
                    group_id,
                    quest_param
                );
                this.script_lib.add_quest_progress(group_id, &quest_param);
                Ok(0)
            },
        );

        methods.add_method(
            "AddRegionRecycleProgress",
            |_, _this, (_ctx, _region_id, _progress_add): (Table, u32, u32)| {
                tracing::debug!("AddRegionRecycleProgress called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddRegionSearchProgress",
            |_, _this, (_ctx, _region_id, _progress_add): (Table, u32, u32)| {
                tracing::debug!("AddRegionSearchProgress called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddRegionalPlayVarValue",
            |_, _this, (_ctx, _uid, _var_type, _add_value): (Table, u32, u32, f32)| {
                tracing::debug!("AddRegionalPlayVarValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddSceneMultiStagePlayUidValue",
            |_,
             _this,
             (_ctx, _group_id, _play_index, _param_name, _uid, _delta): (
                Table,
                u32,
                u32,
                String,
                u32,
                i32,
            )| {
                tracing::debug!("AddSceneMultiStagePlayUidValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddScenePlayBattleProgress",
            |_, _this, (_ctx, _group_id, _add_progress): (Table, u32, i32)| {
                tracing::debug!("AddScenePlayBattleProgress called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddSceneTag",
            |_, _this, (_ctx, _scene_id, _scene_tag_id): (Table, u32, u32)| {
                tracing::debug!("AddSceneTag called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddTeamEntityGlobalFloatValue",
            |_, _this, (_ctx, _param_table, _key, _delta): (Table, Table, String, f32)| {
                tracing::debug!("AddTeamEntityGlobalFloatValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddTeamServerGlobalValue",
            |_, _this, (_ctx, _uid, _key, _value): (Table, u32, String, f32)| {
                tracing::debug!("AddTeamServerGlobalValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AssignPlayerShowTemplateReminder",
            |_, _this, (_ctx, _reminder_id, _param_table): (Table, u32, Table)| {
                tracing::debug!("AssignPlayerShowTemplateReminder called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AssignPlayerUidOpNotify",
            |_, _this, (_ctx, _param_table): (Table, Table)| {
                tracing::debug!("AssignPlayerUidOpNotify called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AttachChildChallenge",
            |_,
             _this,
             (
                _ctx,
                _father_index,
                _child_index,
                _challenge_id,
                _param_table,
                _uid_list,
                _points_table,
            ): (Table, u32, u32, u32, Table, Table, Table)| {
                tracing::debug!("AttachChildChallenge called");
                Ok(-1)
            },
        );

        methods.add_method("AttachGalleryAbilityGroup", |_, _this, (_ctx, _uid_table, _gallery_id, _ability_group_index): (Table, Table, u32, u32)| {
            tracing::debug!("AttachGalleryAbilityGroup called");
            Ok(-1)
        });

        methods.add_method("AttachGalleryTeamAbilityGroup", |_, _this, (_ctx, _uid_table, _gallery_id, _ability_group_index): (Table, Table, u32, u32)| {
            tracing::debug!("AttachGalleryTeamAbilityGroup called");
            Ok(-1)
        });

        methods.add_method(
            "AutoMonsterTide",
            |_, this, (_ctx, source_id, group_id, orders_table, tide_count, scene_limit, _param6): (Table, u32, u32, Table, u32, u32, u32)| {
                let orders: Vec<u32> = orders_table.sequence_values::<u32>().filter_map(|v| v.ok()).collect();
                this.script_lib.auto_monster_tide(group_id, source_id, orders, tide_count, scene_limit);
                Ok(0)
            },
        );

        methods.add_method(
            "AutoPoolMonsterTide",
            |_,
             _this,
             (_ctx, _param1, _param2, _param3, _param4, _param5, _param6, _param7): (
                Table,
                u32,
                u32,
                Table,
                u32,
                Table,
                Table,
                Table,
            )| {
                tracing::debug!("AutoPoolMonsterTide called");
                Ok(-1)
            },
        );

        methods.add_method(
            "BeginCameraSceneLook",
            |_, _this, (_ctx, _param1): (Table, Table)| {
                tracing::debug!("BeginCameraSceneLook called");
                Ok(-1)
            },
        );

        methods.add_method(
            "BeginCameraSceneLookWithTemplate",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, Table)| {
                tracing::debug!("BeginCameraSceneLookWithTemplate called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CancelGroupTimerEvent",
            |_, this, (_ctx, group_id, source): (Table, u32, String)| {
                this.script_lib.cancel_group_timer_event(group_id, &source);
                Ok(0)
            },
        );

        methods.add_method("CauseDungeonFail", |_, this, _ctx: Table| {
            tracing::debug!("[ScriptLib] CauseDungeonFail");
            this.script_lib.cause_dungeon_result(false);
            Ok(0)
        });

        methods.add_method("CauseDungeonSuccess", |_, this, _ctx: Table| {
            tracing::debug!("[ScriptLib] CauseDungeonSuccess");
            this.script_lib.cause_dungeon_result(true);
            Ok(0)
        });

        methods.add_method(
            "ChangeDeathZone",
            |_, _this, (_ctx, _death_zone_id, _param_table): (Table, u32, Table)| {
                tracing::debug!("ChangeDeathZone called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ChangeGroupGadget",
            |_, this, (_ctx, param_table): (Table, Table)| {
                let config_id: u32 = param_table.get("config_id").unwrap_or(0);
                let state: u32 = param_table.get("state").unwrap_or(0);
                let group_id: u32 = _ctx.get("group_id").unwrap_or(0);
                this.script_lib.set_group_gadget_state_by_config_id(group_id, config_id, state);
                Ok(0)
            },
        );

        methods.add_method(
            "ChangeGroupTempValue",
            |_, _this, (_ctx, _key, _delta, _param_table): (Table, String, i32, Table)| {
                tracing::debug!("ChangeGroupTempValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ChangeGroupVariableValue",
            |_, this, (ctx, name, delta): (Table, String, i32)| {
                let group_id: u32 = ctx.get("group_id").unwrap_or(0);
                let result = this
                    .script_lib
                    .change_group_variable_value(group_id, &name, delta);
                tracing::debug!(
                    "ChangeGroupVariableValue group_id={} name={} delta={} -> {}",
                    group_id,
                    name,
                    delta,
                    result
                );
                Ok(result)
            },
        );

        methods.add_method(
            "ChangeGroupVariableValueByGroup",
            |_, this, (_ctx, name, delta, group_id): (Table, String, i32, u32)| {
                let result = this
                    .script_lib
                    .change_group_variable_value_by_group(group_id, &name, delta);
                tracing::debug!(
                    "ChangeGroupVariableValue group_id={} name={} delta={} -> {}",
                    group_id,
                    name,
                    delta,
                    result
                );
                Ok(result)
            },
        );

        methods.add_method(
            "ChangeToTargetLevelTag",
            |_, _this, (_ctx, _level_tag_id): (Table, u32)| {
                tracing::debug!("ChangeToTargetLevelTag called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ChangeToTargetLevelTagWithParamTable",
            |_, _this, (_ctx, _level_tag_id, _param_table): (Table, u32, Table)| {
                tracing::debug!("ChangeToTargetLevelTagWithParamTable called");
                Ok(-1)
            },
        );

        methods.add_method("CharAmusementMultistagePlaySwitchTeam", |_, _this, (_ctx, _group_id, _play_index, _preview_stage_index): (Table, u32, u32, u32)| {
            tracing::debug!("CharAmusementMultistagePlaySwitchTeam called");
            Ok(-1)
        });

        methods.add_method(
            "CharAmusementUpdateScore",
            |_, _this, (_ctx, _group_id, _play_index, _score): (Table, u32, u32, u32)| {
                tracing::debug!("CharAmusementUpdateScore called");
                Ok(0)
            },
        );

        methods.add_method(
            "CheckIsInGroup",
            |_, _this, (_ctx, _group_id, _config_id): (Table, u32, u32)| {
                tracing::debug!("CheckIsInGroup called");
                Ok(false)
            },
        );

        methods.add_method("CheckIsInMpMode", |_, _this, _ctx: Table| {
            tracing::debug!("CheckIsInMpMode called");
            Ok(false)
        });

        methods.add_method(
            "CheckRemainGadgetCountByGroupId",
            |_, _this, (_ctx, _param_table): (Table, Table)| {
                tracing::debug!("CheckRemainGadgetCountByGroupId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CheckSceneTag",
            |_, _this, (_ctx, _scene_id, _scene_tag_id): (Table, u32, u32)| {
                tracing::debug!("CheckSceneTag called");
                Ok(false)
            },
        );

        methods.add_method(
            "ClearExhibitionReplaceableData",
            |_, _this, (_ctx, _uid, _key_name): (Table, u32, String)| {
                tracing::debug!("ClearExhibitionReplaceableData called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ClearExhibitionReplaceableDataById",
            |_, _this, (_ctx, _uid, _exhibition_id): (Table, u32, u32)| {
                tracing::debug!("ClearExhibitionReplaceableDataById called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ClearPermanentSeriesExhibitionData",
            |_, _this, (_ctx, _uid, _series_id): (Table, u32, u32)| {
                tracing::debug!("ClearPermanentSeriesExhibitionData called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ClearPlayerEyePoint",
            |_, _this, (_ctx, _target_region_config_id): (Table, u32)| {
                tracing::debug!("ClearPlayerEyePoint called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ClearPoolMonsterTide",
            |_, _this, (_ctx, _group_id, _tide_index): (Table, u32, u32)| {
                tracing::debug!("ClearPoolMonsterTide called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CloseClimateArea",
            |_, _this, (_ctx, _climate_area_id): (Table, u32)| {
                tracing::debug!("CloseClimateArea called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ContinueAutoMonster",
            |_, _this, (_ctx, _group_id, _tide_index): (Table, u32, u32)| {
                tracing::debug!("ContinueAutoMonster called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ContinueTimeAxis",
            |_, _this, (_ctx, _key): (Table, String)| {
                tracing::debug!("ContinueTimeAxis called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateAsterMidGeneralRewardGadget",
            |_, _this, (_ctx, _param_table): (Table, Table)| {
                tracing::debug!("CreateAsterMidGeneralRewardGadget called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateBlossomChestByGroupId",
            |_, _this, (_ctx, _group_id, _config_id): (Table, u32, u32)| {
                tracing::debug!("CreateBlossomChestByGroupId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateChannellerSlabCampRewardGadget",
            |_, _this, (_ctx, _config_id): (Table, u32)| {
                tracing::debug!("CreateChannellerSlabCampRewardGadget called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateEffigyChallengeMonster",
            |_, _this, (_ctx, _group_id, _pool_list): (Table, u32, Table)| {
                tracing::debug!("CreateEffigyChallengeMonster called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateFatherChallenge",
            |_,
             _this,
             (_ctx, _father_index, _challenge_id, _duration, _param_table): (
                Table,
                u32,
                u32,
                u32,
                Table,
            )| {
                tracing::debug!("CreateFatherChallenge called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateFoundation",
            |_,
             _this,
             (_ctx, _param_list, _config_id, _point_config_id, _main_group_id, _play_index): (
                Table,
                Table,
                u32,
                u32,
                u32,
                u32,
            )| {
                tracing::debug!("CreateFoundation called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateFoundations",
            |_,
             _this,
             (_ctx, _config_id_2_point_table, _main_group_id, _play_index): (
                Table,
                Table,
                u32,
                u32,
            )| {
                tracing::debug!("CreateFoundations called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateGadget",
            |_, this, (ctx, config_table): (Table, Table)| {
                let group_id: u32 = ctx.get("group_id").unwrap_or(0);
                let config_id: u32 = config_table.get("config_id").unwrap_or(0);
                tracing::debug!(
                    "[ScriptLib] CreateGadget group={} config={}",
                    group_id,
                    config_id
                );
                if config_id != 0 {
                    this.script_lib.create_gadget(group_id, config_id);
                }
                Ok(0)
            },
        );

        methods.add_method(
            "CreateGadgetByConfigIdByPos",
            |_, _this, (_ctx, _config_id, _pos_table, _rot_table): (Table, u32, Table, Table)| {
                tracing::debug!("CreateGadgetByConfigIdByPos called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateGadgetByParamTable",
            |_, _this, (_ctx, _param_table): (Table, Table)| {
                tracing::debug!("CreateGadgetByParamTable called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateGadgetWave",
            |_,
             _this,
             (
                _ctx,
                _config_id,
                _suite_index,
                _offset_y,
                _param_bunding_box_size,
                _param_gadget_size,
            ): (Table, u32, u32, u32, Table, Table)| {
                tracing::debug!("CreateGadgetWave called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateGadgetWithGlobalValue",
            |_, _this, (_ctx, _config_id, _value_table): (Table, u32, Table)| {
                tracing::debug!("CreateGadgetWithGlobalValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateGather",
            |_, _this, (_ctx, _p_config_table): (Table, Table)| {
                tracing::debug!("CreateGather called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateGroupTimerEvent",
            |_, this, (_ctx, group_id, source, time): (Table, u32, String, f64)| {
                this.script_lib.create_group_timer_event(group_id, &source, time);
                Ok(0)
            },
        );

        methods.add_method(
            "CreateGroupTrigger",
            |_, _this, (_ctx, _name): (Table, String)| {
                tracing::debug!("CreateGroupTrigger called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateMonster",
            |_, this, (ctx, config_table): (Table, Table)| {
                let group_id: u32 = ctx.get("group_id").unwrap_or(0);
                let config_id: u32 = config_table.get("config_id").unwrap_or(0);
                let delay_time: u32 = config_table.get("delay_time").unwrap_or(0);
                tracing::debug!(
                    "[ScriptLib] CreateMonster group={} config={} delay={}",
                    group_id,
                    config_id,
                    delay_time
                );
                this.script_lib.create_monster(group_id, config_id);
                Ok(0)
            },
        );

        methods.add_method(
            "CreateMonsterByConfigIdByPos",
            |_, _this, (_ctx, _config_id, _pos_table, _rot_table): (Table, u32, Table, Table)| {
                tracing::debug!("CreateMonsterByConfigIdByPos called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateMonsterByConfigIdByPosByBornType",
            |_,
             _this,
             (_ctx, _config_id, _pos_table, _rot_table, _born_type): (
                Table,
                u32,
                Table,
                Table,
                u32,
            )| {
                tracing::debug!("CreateMonsterByConfigIdByPosByBornType called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateMonsterFaceAvatar",
            |_, _this, (_ctx, _param_table): (Table, Table)| {
                tracing::debug!("CreateMonsterFaceAvatar called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateMonsterWithGlobalValue",
            |_, _this, (_ctx, _config_id, _value_table): (Table, u32, Table)| {
                tracing::debug!("CreateMonsterWithGlobalValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateMonstersFromMonsterPool",
            |_, _this, (_ctx, _drop_tag): (Table, String)| {
                tracing::debug!("CreateMonstersFromMonsterPool called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateScenePlayGeneralRewardGadget",
            |_, _this, (_ctx, _param_table): (Table, Table)| {
                tracing::debug!("CreateScenePlayGeneralRewardGadget called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateTreasureMapSpotRewardGadget",
            |_, _this, (_ctx, _config_id): (Table, u32)| {
                tracing::debug!("CreateTreasureMapSpotRewardGadget called");
                Ok(-1)
            },
        );

        methods.add_method("CreateVehicle", |_, _this, (_ctx, _uid, _vehicle_id, _pos_table, _rot_table): (Table, u32, u32, Table, Table)| {
            tracing::debug!("CreateVehicle called");
            Ok(0)
        });

        methods.add_method(
            "CrystalLinkDungeonTeamSetUp",
            |_, _this, (_ctx, _team_id, _param_table): (Table, u32, Table)| {
                tracing::debug!("CrystalLinkDungeonTeamSetUp called");
                Ok(-1)
            },
        );

        methods.add_method(
            "DeactivateGroupLinkBundle",
            |_, _this, (_ctx, _group_id): (Table, u32)| {
                tracing::debug!("DeactivateGroupLinkBundle called");
                Ok(-1)
            },
        );

        methods.add_method(
            "DeactivateGroupLinkBundleByBundleId",
            |_, _this, (_ctx, _bundle_id): (Table, u32)| {
                tracing::debug!("DeactivateGroupLinkBundleByBundleId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "DelAllSubEntityByOriginOwnerConfigId",
            |_, _this, (_ctx, _origin_owner_config_id): (Table, u32)| {
                tracing::debug!("DelAllSubEntityByOriginOwnerConfigId called");
                Ok(-1)
            },
        );

        methods.add_method("DelGalleryAbilityGroup", |_, _this, (_ctx, _uid_table, _gallery_id, _ability_group_index): (Table, Table, u32, u32)| {
            tracing::debug!("DelGalleryAbilityGroup called");
            Ok(-1)
        });

        methods.add_method("DelGalleryTeamAbilityGroup", |_, _this, (_ctx, _uid_table, _gallery_id, _ability_group_index): (Table, Table, u32, u32)| {
            tracing::debug!("DelGalleryTeamAbilityGroup called");
            Ok(-1)
        });

        methods.add_method(
            "DelPlayerGroupVisionType",
            |_, _this, (_ctx, _uid_list, _type_list): (Table, Table, Table)| {
                tracing::debug!("DelPlayerGroupVisionType called");
                Ok(-1)
            },
        );

        methods.add_method(
            "DelSceneTag",
            |_, _this, (_ctx, _scene_id, _scene_tag_id): (Table, u32, u32)| {
                tracing::debug!("DelSceneTag called");
                Ok(-1)
            },
        );

        methods.add_method(
            "DelWorktopOption",
            |_, _this, (_ctx, _option): (Table, u32)| {
                tracing::debug!("DelWorktopOption called");
                Ok(-1)
            },
        );

        methods.add_method(
            "DelWorktopOptionByGroupId",
            |_, this, (_ctx, group_id, config_id, option): (Table, u32, u32, u32)| {
                tracing::debug!(
                    "[ScriptLib] DelWorktopOptionByGroupId group={} config={} option={}",
                    group_id,
                    config_id,
                    option
                );

                this.script_lib
                    .del_worktop_option_by_group_id(group_id, config_id, option);

                Ok(0)
            },
        );

        methods.add_method(
            "DestroyIrodoriChessTower",
            |_, _this, (_ctx, _entity_id, _main_group_id, _play_index): (Table, u32, u32, u32)| {
                tracing::debug!("DestroyIrodoriChessTower called");
                Ok(-1)
            },
        );

        methods.add_method("DigRetractAllWidget", |_, _this, _ctx: Table| {
            tracing::debug!("DigRetractAllWidget called");
            Ok(-1)
        });

        methods.add_method(
            "DigSetSearchingTarget",
            |_, _this, (_ctx, _config_id): (Table, u32)| {
                tracing::debug!("DigSetSearchingTarget called");
                Ok(-1)
            },
        );

        methods.add_method(
            "DisableRoguelikeTrapBySgv",
            |_, _this, (_ctx, _sgv_key, _uid): (Table, String, u32)| {
                tracing::debug!("DisableRoguelikeTrapBySgv called");
                Ok(-1)
            },
        );

        methods.add_method(
            "DoRoguelikeCardGachaByLua",
            |_, _this, (_ctx, _uid): (Table, u32)| {
                tracing::debug!("DoRoguelikeCardGachaByLua called");
                Ok(-1)
            },
        );

        methods.add_method(
            "DropSubfield",
            |_, _this, (_ctx, _param_table): (Table, Table)| {
                tracing::debug!("DropSubfield called");
                Ok(-1)
            },
        );

        methods.add_method("EndAllTimeAxis", |_, this, _ctx: Table| {
            this.script_lib.end_all_time_axis();
            Ok(0)
        });

        methods.add_method(
            "EndFatherChallenge",
            |_, _this, (_ctx, _father_index): (Table, u32)| {
                tracing::debug!("EndFatherChallenge called");
                Ok(-1)
            },
        );

        methods.add_method(
            "EndMonsterTide",
            |_, _this, (_ctx, _group_id, _tide_index, _end_type): (Table, u32, u32, u32)| {
                tracing::debug!("EndMonsterTide called");
                Ok(-1)
            },
        );

        methods.add_method(
            "EndPoolMonsterTide",
            |_, _this, (_ctx, _group_id, _tide_index): (Table, u32, u32)| {
                tracing::debug!("EndPoolMonsterTide called");
                Ok(-1)
            },
        );

        methods.add_method(
            "EndSceneMultiStagePlay",
            |_, _this, (_ctx, _play_index, _is_succ): (Table, u32, bool)| {
                tracing::debug!("EndSceneMultiStagePlay called");
                Ok(-1)
            },
        );

        methods.add_method(
            "EndSceneMultiStagePlayStage",
            |_, _this, (_ctx, _play_index, _stage_name, _is_succ): (Table, u32, String, bool)| {
                tracing::debug!("EndSceneMultiStagePlayStage called");
                Ok(-1)
            },
        );

        methods.add_method("EndTimeAxis", |_, this, (_ctx, key): (Table, String)| {
            this.script_lib.end_time_axis(&key);
            Ok(0)
        });

        methods.add_method(
            "EnterCustomDungeonOfficialEdit",
            |_, _this, (_ctx, _room_id): (Table, u32)| {
                tracing::debug!("EnterCustomDungeonOfficialEdit called");
                Ok(-1)
            },
        );

        methods.add_method(
            "EnterPersistentDungeon",
            |_, _this, (_ctx, _uid, _dungeon_id, _param_table): (Table, u32, u32, Table)| {
                tracing::debug!("EnterPersistentDungeon called");
                Ok(-1)
            },
        );

        methods.add_method(
            "EnterRogueCell",
            |_, _this, (_ctx, _group_id): (Table, u32)| {
                tracing::debug!("EnterRogueCell called");
                Ok(-1)
            },
        );

        methods.add_method("EnterRogueDungeonNextLevel", |_, _this, _ctx: Table| {
            tracing::debug!("EnterRogueDungeonNextLevel called");
            Ok(-1)
        });

        methods.add_method(
            "EnterWeatherArea",
            |_, _this, (_ctx, _height_area_id): (Table, u32)| {
                tracing::debug!("EnterWeatherArea called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ExecuteActiveGroupLua",
            |_, _this, (_ctx, _group_id, _func_name, _param_list): (Table, u32, String, Table)| {
                tracing::debug!("ExecuteActiveGroupLua called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ExecuteGadgetLua",
            |_,
             _this,
             (_ctx, _group_id, _config_id, _param1, _param2, _param3): (
                Table,
                u32,
                u32,
                i32,
                i32,
                i32,
            )| {
                tracing::debug!("ExecuteGadgetLua called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ExecuteGroupLua",
            |_, _this, (_ctx, _group_id, _func_name, _param_list): (Table, u32, String, Table)| {
                tracing::debug!("ExecuteGroupLua called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ExpeditionChallengeEnterRegion",
            |_, _this, (_ctx, _is_puzzle_finished): (Table, bool)| {
                tracing::debug!("ExpeditionChallengeEnterRegion called");
                Ok(-1)
            },
        );

        methods.add_method(
            "FailMistTrialDungeonChallenge",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("FailMistTrialDungeonChallenge called");
                Ok(-1)
            },
        );

        methods.add_method(
            "FailScenePlayBattle",
            |_, _this, (_ctx, _group_id): (Table, u32)| {
                tracing::debug!("FailScenePlayBattle called");
                Ok(-1)
            },
        );

        methods.add_method("FinishExpeditionChallenge", |_, _this, _ctx: Table| {
            tracing::debug!("FinishExpeditionChallenge called");
            Ok(-1)
        });

        methods.add_method("FinishFindHilichurlLevel", |_, _this, _ctx: Table| {
            tracing::debug!("FinishFindHilichurlLevel called");
            Ok(-1)
        });

        methods.add_method(
            "FinishFleurFairGalleryStageByUid",
            |_,
             _this,
             (_ctx, _group_id, _play_index, _uid, _is_last_player): (
                Table,
                u32,
                u32,
                u32,
                bool,
            )| {
                tracing::debug!("FinishFleurFairGalleryStageByUid called");
                Ok(-1)
            },
        );

        methods.add_method(
            "FinishGroupLinkBundle",
            |_, _this, (_ctx, _group_id): (Table, u32)| {
                tracing::debug!("FinishGroupLinkBundle called");
                Ok(-1)
            },
        );

        methods.add_method(
            "FinishGroupLinkBundleByBundleId",
            |_, _this, (_ctx, _bundle_id): (Table, u32)| {
                tracing::debug!("FinishGroupLinkBundleByBundleId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "FinishRandTask",
            |_, _this, (_ctx, _rand_task_id, _is_succ): (Table, u32, bool)| {
                tracing::debug!("FinishRandTask called");
                Ok(-1)
            },
        );

        methods.add_method(
            "FinishRogueDiaryDungeonSingleRoom",
            |_, _this, (_ctx, _is_fail): (Table, bool)| {
                tracing::debug!("FinishRogueDiaryDungeonSingleRoom called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ForbidPlayerRegionVision",
            |_, _this, (_ctx, _uid): (Table, u32)| {
                tracing::debug!("ForbidPlayerRegionVision called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ForceRefreshAuthorityByConfigId",
            |_, _this, (_ctx, _config_id, _uid): (Table, u32, u32)| {
                tracing::debug!("ForceRefreshAuthorityByConfigId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ForceSetIrodoriFoundationTowers",
            |_,
             _this,
             (_ctx, _config_id_to_gear_table, _main_group_id, _play_index): (
                Table,
                Table,
                u32,
                u32,
            )| {
                tracing::debug!("ForceSetIrodoriFoundationTowers called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GadgetLuaNotifyGroup",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, i32, i32, i32)| {
                tracing::debug!("GadgetLuaNotifyGroup called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GadgetPlayUidOp",
            |_,
             _this,
             (_ctx, _group_id, _config_id, _uid_list, _op, _param_str, _param_list): (
                Table,
                u32,
                u32,
                Table,
                u32,
                String,
                Table,
            )| {
                tracing::debug!("GadgetPlayUidOp called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetActivityDayIndexByScheduleId",
            |_, _this, (_ctx, _activity_schedule_id): (Table, u32)| {
                tracing::debug!("GetActivityDayIndexByScheduleId called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetActivityOpenAndCloseTimeByScheduleId",
            |_, _this, (_ctx, _context): (Table, u32)| {
                tracing::debug!("GetActivityOpenAndCloseTimeByScheduleId called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetAranaraCollectableCountByTypeAndState",
            |_, _this, (_ctx, _collection_type, _collection_state): (Table, u32, u32)| {
                tracing::debug!("GetAranaraCollectableCountByTypeAndState called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetAvatarEntityIdByUid",
            |_, _this, (_ctx, _uid): (Table, u32)| {
                tracing::debug!("GetAvatarEntityIdByUid called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetBlossomRefreshTypeByGroupId",
            |_, _this, (_ctx, _group_id): (Table, u32)| {
                tracing::debug!("GetBlossomRefreshTypeByGroupId called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetBlossomScheduleStateByGroupId",
            |_, _this, (_ctx, _group_id): (Table, u32)| {
                tracing::debug!("GetBlossomScheduleStateByGroupId called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetBonusTreasureMapSolution",
            |_, _this, (_ctx, _context): (Table, u32)| {
                tracing::debug!("GetBonusTreasureMapSolution called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetChainLevel",
            |_, _this, (_ctx, _uid, _chain_id): (Table, u32, u32)| {
                tracing::debug!("GetChainLevel called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetChallengeTransaction",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetChallengeTransaction called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetChannellerSlabLoopDungeonLimitTime",
            |_, _this, _ctx: Table| {
                tracing::debug!("GetChannellerSlabLoopDungeonLimitTime called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetCharAmusementGalleryTarget",
            |_, _this, (_ctx, _gallery_id, _is_mp): (Table, u32, bool)| {
                tracing::debug!("GetCharAmusementGalleryTarget called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetCharAmusementMultistagePlayGalleryIdVec",
            |_, _this, (_ctx, _context, _group_id): (Table, u32, u32)| {
                tracing::debug!("GetCharAmusementMultistagePlayGalleryIdVec called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetChessBuildingPoints",
            |_, _this, (_ctx, _group_id, _play_index, _player_uid): (Table, u32, u32, u32)| {
                tracing::debug!("GetChessBuildingPoints called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetChessMonsterPoolIdVecByRound",
            |_, _this, (_ctx, _context, _group_id, _play_index): (Table, u32, u32, u32)| {
                tracing::debug!("GetChessMonsterPoolIdVecByRound called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetCoinCollectGalleryPlayerSkillInfo",
            |_, _this, (_ctx, _context, _uid): (Table, u32, u32)| {
                tracing::debug!("GetCoinCollectGalleryPlayerSkillInfo called");
                Ok(0)
            },
        );

        methods.add_method("GetContextGadgetConfigId", |_, _this, _ctx: Table| {
            tracing::debug!("GetContextGadgetConfigId called");
            Ok(0)
        });

        methods.add_method("GetContextGadgetEntityId", |_, _this, _ctx: Table| {
            tracing::debug!("GetContextGadgetEntityId called");
            Ok(0)
        });

        methods.add_method("GetContextGroupId", |_, _this, _ctx: Table| {
            tracing::debug!("GetContextGroupId called");
            Ok(0)
        });

        methods.add_method(
            "GetCurFungusFighterPlotConfigIdList",
            |_, _this, _ctx: Table| {
                tracing::debug!("GetCurFungusFighterPlotConfigIdList called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetCurFungusFighterTrainingParams",
            |_, _this, _ctx: Table| {
                tracing::debug!("GetCurFungusFighterTrainingParams called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetCurFungusFighterTrainingValidBackupFungusIdList",
            |_, _this, _ctx: Table| {
                tracing::debug!("GetCurFungusFighterTrainingValidBackupFungusIdList called");
                Ok(0)
            },
        );

        methods.add_method("GetCurTriggerCount", |_, _this, _ctx: Table| {
            tracing::debug!("GetCurTriggerCount called");
            Ok(-1)
        });

        methods.add_method(
            "GetCurrentCustomDungeonForbidSkill",
            |_, _this, _ctx: Table| {
                tracing::debug!("GetCurrentCustomDungeonForbidSkill called");
                Ok(false)
            },
        );

        methods.add_method(
            "GetCurrentCustomDungeonParamVec",
            |_, _this, _ctx: Table| {
                tracing::debug!("GetCurrentCustomDungeonParamVec called");
                Ok(0)
            },
        );

        methods.add_method("GetCurrentGroupWeather", |_, _this, _ctx: Table| {
            tracing::debug!("GetCurrentGroupWeather called");
            Ok(-1)
        });

        methods.add_method(
            "GetCurrentLevelTagVec",
            |_, _this, (_ctx, _context): (Table, u32)| {
                tracing::debug!("GetCurrentLevelTagVec called");
                Ok(0)
            },
        );

        methods.add_method("GetCustomDungeonCoinNum", |_, _this, _ctx: Table| {
            tracing::debug!("GetCustomDungeonCoinNum called");
            Ok(0)
        });

        methods.add_method("GetCustomDungeonOpenRoomVec", |_, _this, _ctx: Table| {
            tracing::debug!("GetCustomDungeonOpenRoomVec called");
            Ok(0)
        });

        methods.add_method(
            "GetDeathZoneStatus",
            |_, _this, (_ctx, _death_zone_id): (Table, u32)| {
                tracing::debug!("GetDeathZoneStatus called");
                Ok(-1)
            },
        );

        methods.add_method("GetDungeonTeamPlayerNum", |_, _this, _ctx: Table| {
            tracing::debug!("GetDungeonTeamPlayerNum called");
            Ok(0)
        });

        methods.add_method("GetDungeonTransaction", |_, _this, _ctx: Table| {
            tracing::debug!("GetDungeonTransaction called");
            Ok(0)
        });

        methods.add_method("GetEffigyChallengeLimitTime", |_, _this, _ctx: Table| {
            tracing::debug!("GetEffigyChallengeLimitTime called");
            Ok(-1)
        });

        methods.add_method("GetEffigyChallengeMonsterLevel", |_, _this, _ctx: Table| {
            tracing::debug!("GetEffigyChallengeMonsterLevel called");
            Ok(-1)
        });

        methods.add_method(
            "GetEffigyChallengeV2DungeonDifficulty",
            |_, _this, _ctx: Table| {
                tracing::debug!("GetEffigyChallengeV2DungeonDifficulty called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetEntityIdByConfigId",
            |_, _this, (_ctx, _config_id): (Table, u32)| {
                tracing::debug!("GetEntityIdByConfigId called");
                Ok(0)
            },
        );

        methods.add_method("GetEntityType", |_, this, entity_id: u32| {
            tracing::debug!("GetEntityType called");
            Ok(entity_id >> get_ty_value_by_version(this.protocol_version.as_str()))
        });

        methods.add_method(
            "GetExhibitionAccumulableData",
            |_, _this, (_ctx, _uid, _id): (Table, u32, u32)| {
                tracing::debug!("GetExhibitionAccumulableData called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetExhibitionReplaceableData",
            |_, _this, (_ctx, _uid, _id): (Table, u32, u32)| {
                tracing::debug!("GetExhibitionReplaceableData called");
                Ok(0)
            },
        );

        methods.add_method("GetFleurFairDungeonSectionId", |_, _this, _ctx: Table| {
            tracing::debug!("GetFleurFairDungeonSectionId called");
            Ok(0)
        });

        methods.add_method(
            "GetFleurFairMultistagePlayBuffEnergy",
            |_, _this, (_ctx, _group_id, _play_index, _uid): (Table, u32, u32, u32)| {
                tracing::debug!("GetFleurFairMultistagePlayBuffEnergy called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetFleurFairMultistagePlayGalleryIdVec",
            |_, _this, (_ctx, _context, _group_id): (Table, u32, u32)| {
                tracing::debug!("GetFleurFairMultistagePlayGalleryIdVec called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetFleurFairMultistagePlayGalleryTempValue",
            |_, _this, (_ctx, _group_id, _play_index, _key): (Table, u32, u32, String)| {
                tracing::debug!("GetFleurFairMultistagePlayGalleryTempValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetGadgetAbilityFloatValue",
            |_, _this, (_ctx, _group_id, _config_id, _key): (Table, u32, u32, String)| {
                tracing::debug!("GetGadgetAbilityFloatValue called");
                Ok(0.0f32)
            },
        );

        methods.add_method(
            "GetGadgetAbilityUIntValue",
            |_, _this, (_ctx, _group_id, _config_id, _key): (Table, u32, u32, String)| {
                tracing::debug!("GetGadgetAbilityUIntValue called");
                Ok(0)
            },
        );

        methods.add_method("GetGadgetArguments", |_, _this, _ctx: Table| {
            tracing::debug!("GetGadgetArguments called");
            Ok(0)
        });

        methods.add_method(
            "GetGadgetConfigId",
            |_, _this, (_ctx, _param_table): (Table, Table)| {
                tracing::debug!("GetGadgetConfigId called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetGadgetHpPercent",
            |_, _this, (_ctx, _group_id, _config_id): (Table, u32, u32)| {
                tracing::debug!("GetGadgetHpPercent called");
                Ok(0.0f32)
            },
        );

        methods.add_method(
            "GetGadgetIdByEntityId",
            |_, _this, (_ctx, _entity_id): (Table, u32)| {
                tracing::debug!("GetGadgetIdByEntityId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetGadgetPlayProgress",
            |_, _this, (_ctx, _group_id, _config_id): (Table, u32, u32)| {
                tracing::debug!("GetGadgetPlayProgress called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetGadgetPlayStage",
            |_, _this, (_ctx, _group_id, _config_id): (Table, u32, u32)| {
                tracing::debug!("GetGadgetPlayStage called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetGadgetPlayStageBeginProgress",
            |_, _this, (_ctx, _group_id, _config_id): (Table, u32, u32)| {
                tracing::debug!("GetGadgetPlayStageBeginProgress called");
                Ok(-1)
            },
        );

        methods.add_method("GetGadgetPlayUidValue", |_, _this, (_ctx, _group_id, _config_id, _uid, _key): (Table, u32, u32, u32, String)| {
            tracing::debug!("GetGadgetPlayUidValue called");
            Ok(-1)
        });

        methods.add_method("GetGadgetState", |_, this, ctx: Table| {
            let uid: u32 = ctx.get("uid").unwrap_or(0);
            let group_id: u32 = ctx.get("group_id").unwrap_or(0);
            let config_id: u32 = ctx.get("config_id").unwrap_or(0);
            let state = this
                .script_lib
                .get_gadget_state_by_config_id(uid, group_id, config_id);
            tracing::debug!(
                "GetGadgetState: group_id={}, config_id={}, state={}",
                group_id,
                config_id,
                state
            );
            Ok(state)
        });

        methods.add_method("GetGadgetStateBeginTime", |_, _this, _ctx: Table| {
            tracing::debug!("GetGadgetStateBeginTime called");
            Ok(0)
        });

        methods.add_method(
            "GetGadgetStateByConfigId",
            |_, this, (ctx, group_id, config_id): (Table, u32, u32)| {
                let uid: u32 = ctx.get("uid").unwrap_or(0);
                let state = this
                    .script_lib
                    .get_gadget_state_by_config_id(uid, group_id, config_id);
                tracing::debug!(
                    "GetGadgetStateByConfigId: group_id={}, config_id={}, state={}",
                    group_id,
                    config_id,
                    state
                );
                Ok(state)
            },
        );

        methods.add_method(
            "GetGalleryProgressScore",
            |_, _this, (_ctx, _key, _gallery_id): (Table, String, u32)| {
                tracing::debug!("GetGalleryProgressScore called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetGalleryTransaction",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetGalleryTransaction called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetGalleryUidList",
            |_, _this, (_ctx, _context): (Table, u32)| {
                tracing::debug!("GetGalleryUidList called");
                Ok(0)
            },
        );

        methods.add_method("GetGameHour", |_, _this, _ctx: Table| {
            // Return game time hour (0-23)
            let hours = (common::time_util::unix_timestamp() / 1000 % (24 * 60 * 60) / (60 * 60)) as i32;
            tracing::debug!("[ScriptLib] GetGameHour = {}", hours);
            Ok(hours)
        });

        methods.add_method("GetGameTimePassed", |_, _this, _ctx: Table| {
            tracing::debug!("GetGameTimePassed called");
            Ok(0)
        });

        methods.add_method("GetGatherConfigIdList", |_, _this, _ctx: Table| {
            tracing::debug!("GetGatherConfigIdList called");
            Ok(0)
        });

        methods.add_method("GetGearStartValue", |_, _this, _ctx: Table| {
            tracing::debug!("GetGearStartValue called");
            Ok(-1)
        });

        methods.add_method("GetGearStopValue", |_, _this, _ctx: Table| {
            tracing::debug!("GetGearStopValue called");
            Ok(-1)
        });

        methods.add_method(
            "GetGivingItemList",
            |_, _this, (_ctx, _context): (Table, u32)| {
                tracing::debug!("GetGivingItemList called");
                Ok(0)
            },
        );

        methods.add_method("GetGmtOffset", |_, _this, _ctx: Table| {
            tracing::debug!("GetGmtOffset called");
            Ok(0)
        });

        methods.add_method(
            "GetGroupAliveMonsterList",
            |_, _this, (_ctx, _context): (Table, u32)| {
                tracing::debug!("GetGroupAliveMonsterList called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetGroupIdByEntityId",
            |_, _this, (_ctx, _entity_id): (Table, u32)| {
                tracing::debug!("GetGroupIdByEntityId called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetGroupLogicStateValue",
            |_, _this, (_ctx, _name): (Table, String)| {
                tracing::debug!("GetGroupLogicStateValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetGroupLogicStateValueByGroup",
            |_, _this, (_ctx, _name, _group_id): (Table, String, u32)| {
                tracing::debug!("GetGroupLogicStateValueByGroup called");
                Ok(-1)
            },
        );

        methods.add_method("GetGroupMonsterCount", |_, this, ctx: Table| {
            tracing::debug!("GetGroupMonsterCount called");
            let uid: u32 = ctx.get("uid").unwrap_or(0);
            let group_id: u32 = ctx.get("group_id").unwrap_or(0);
            let count = this
                .script_lib
                .get_group_monster_count_by_config_id(uid, group_id);
            Ok(count)
        });

        methods.add_method(
            "GetGroupMonsterCountByGroupId",
            |_, this, (ctx, group_id): (Table, u32)| {
                tracing::debug!("GetGroupMonsterCountByGroupId called");
                let uid: u32 = ctx.get("uid").unwrap_or(0);
                let count = this
                    .script_lib
                    .get_group_monster_count_by_config_id(uid, group_id);
                Ok(count)
            },
        );

        methods.add_method(
            "GetGroupSuite",
            |_, this, (_ctx, group_id): (Table, u32)| {
                Ok(this.script_lib.get_group_suite(group_id))
            },
        );

        methods.add_method(
            "GetGroupTempValue",
            |_, _this, (_ctx, _key, _param_table): (Table, String, Table)| {
                tracing::debug!("GetGroupTempValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetGroupVariableValue",
            |_, _this, (ctx, name): (Table, String)| {
                let group_id: u32 = ctx.get("group_id").unwrap_or(0);
                let result = _this.script_lib.get_group_variable_value(group_id, &name);
                tracing::debug!(
                    "GetGroupVariableValue group={} name={} -> {}",
                    group_id,
                    name,
                    result
                );
                Ok(result)
            },
        );

        methods.add_method(
            "GetGroupVariableValueByGroup",
            |_, _this, (_ctx, name, group_id): (Table, String, u32)| {
                let result = _this.script_lib.get_group_variable_value(group_id, &name);
                tracing::debug!(
                    "GetGroupVariableValueByGroup group_id={} name={} -> {}",
                    group_id,
                    name,
                    result
                );
                Ok(result)
            },
        );

        methods.add_method(
            "GetHideAndSeekHunter",
            |_, _this, (_ctx, _play_index): (Table, u32)| {
                tracing::debug!("GetHideAndSeekHunter called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetHideAndSeekMap",
            |_, _this, (_ctx, _play_index): (Table, u32)| {
                tracing::debug!("GetHideAndSeekMap called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetHideAndSeekPlayGalleryId",
            |_, _this, (_ctx, _play_index): (Table, u32)| {
                tracing::debug!("GetHideAndSeekPlayGalleryId called");
                Ok(0)
            },
        );

        methods.add_method("GetHideAndSeekPlayIndex", |_, _this, _ctx: Table| {
            tracing::debug!("GetHideAndSeekPlayIndex called");
            Ok(0)
        });

        methods.add_method(
            "GetHideAndSeekPlayerSkillList",
            |_, _this, (_ctx, _context, _play_index): (Table, u32, u32)| {
                tracing::debug!("GetHideAndSeekPlayerSkillList called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetHideAndSeekPreyUidList",
            |_, _this, (_ctx, _context): (Table, u32)| {
                tracing::debug!("GetHideAndSeekPreyUidList called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetHostCityLevel",
            |_, _this, (_ctx, _scene_id, _city_id): (Table, u32, u32)| {
                tracing::debug!("GetHostCityLevel called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetHostQuestState",
            |_, _this, (_ctx, _quest_id): (Table, u32)| {
                tracing::debug!("GetHostQuestState called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetHuntingMonsterExtraSuiteIndexVec",
            |_, _this, _ctx: Table| {
                tracing::debug!("GetHuntingMonsterExtraSuiteIndexVec called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetIrodoriChessBuildingPoints",
            |_, _this, (_ctx, _group_id, _play_index): (Table, u32, u32)| {
                tracing::debug!("GetIrodoriChessBuildingPoints called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetIrodoriChessSelectedCards",
            |_, _this, (_ctx, _context, _main_group_id): (Table, u32, u32)| {
                tracing::debug!("GetIrodoriChessSelectedCards called");
                Ok(0)
            },
        );

        methods.add_method("GetLanternRiteValue", |_, _this, _ctx: Table| {
            tracing::debug!("GetLanternRiteValue called");
            Ok(0)
        });

        methods.add_method(
            "GetLevelTagNameById",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetLevelTagNameById called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetLunaRiteSacrificeNum",
            |_, _this, (_ctx, _area_id): (Table, u32)| {
                tracing::debug!("GetLunaRiteSacrificeNum called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetMechanicusBuildingPoints",
            |_, _this, (_ctx, _group_id, _play_index, _player_uid): (Table, u32, u32, u32)| {
                tracing::debug!("GetMechanicusBuildingPoints called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetMechanicusMonsterPoolVec",
            |_, _this, (_ctx, _context, _group_id): (Table, u32, u32)| {
                tracing::debug!("GetMechanicusMonsterPoolVec called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetMonsterAbilityFloatValue",
            |_, _this, (_ctx, _group_id, _config_id, _key): (Table, u32, u32, String)| {
                tracing::debug!("GetMonsterAbilityFloatValue called");
                Ok(0.0f32)
            },
        );

        methods.add_method(
            "GetMonsterAffixListByConfigId",
            |_, _this, (_ctx, _context, _group_id): (Table, u32, u32)| {
                tracing::debug!("GetMonsterAffixListByConfigId called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetMonsterConfigId",
            |_, _this, (_ctx, _param_table): (Table, Table)| {
                tracing::debug!("GetMonsterConfigId called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetMonsterHp",
            |_, _this, (_ctx, _group_id, _config_id): (Table, u32, u32)| {
                tracing::debug!("GetMonsterHp called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetMonsterHpPercent",
            |_, _this, (_ctx, _group_id, _config_id): (Table, u32, u32)| {
                tracing::debug!("GetMonsterHpPercent called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetMonsterIdByEntityId",
            |_, _this, (_ctx, _entity_id): (Table, u32)| {
                tracing::debug!("GetMonsterIdByEntityId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetNpcConfigId",
            |_, _this, (_ctx, _param_table): (Table, Table)| {
                tracing::debug!("GetNpcConfigId called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetOfferingLevel",
            |_, _this, (_ctx, _offering_id): (Table, u32)| {
                tracing::debug!("GetOfferingLevel called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetOpeningDungeonListByRosterId",
            |_, _this, (_ctx, _context): (Table, u32)| {
                tracing::debug!("GetOpeningDungeonListByRosterId called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetPlatformArrayInfoByPointId",
            |_,
             _this,
             (_ctx, _point_array_id, _point_id, _retcode, _pos, _rot): (
                Table,
                u32,
                u32,
                i32,
                u32,
                u32,
            )| {
                tracing::debug!("GetPlatformArrayInfoByPointId called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetPlatformPointArray",
            |_, _this, (_ctx, _context): (Table, u32)| {
                tracing::debug!("GetPlatformPointArray called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetPlayerGroupVisionType",
            |_, _this, (_ctx, _context): (Table, u32)| {
                tracing::debug!("GetPlayerGroupVisionType called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetPlayerItemNum",
            |_, _this, (_ctx, _uid, _item_id): (Table, u32, u32)| {
                tracing::debug!("GetPlayerItemNum called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetPlayerVehicleType",
            |_, _this, (_ctx, _uid): (Table, u32)| {
                tracing::debug!("GetPlayerVehicleType called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetPosByEntityId",
            |_, _this, (_ctx, _entity_id): (Table, u32)| {
                tracing::debug!("GetPosByEntityId called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetPotionDungeonAffixParams",
            |_, _this, (_ctx, _context): (Table, Table)| {
                tracing::debug!("GetPotionDungeonAffixParams called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetQuestState",
            |_, _this, (_ctx, _entity_id, quest_id): (Table, u32, u32)| {
                tracing::debug!("[ScriptLib] GetQuestState quest={}", quest_id);
                // 0=None, 1=Unstarted, 2=Unfinished, 3=Finished, 4=Failed
                // Without player access, return default
                Ok(0)
            },
        );

        methods.add_method(
            "GetQuestStateByUid",
            |_, _this, (_ctx, _uid, _quest_id): (Table, u32, u32)| {
                tracing::debug!("GetQuestStateByUid called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetRegionConfigId",
            |_, _this, (_ctx, _param_table): (Table, Table)| {
                tracing::debug!("GetRegionConfigId called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetRegionEntityCount",
            |_, _this, (_ctx, _param_table): (Table, Table)| {
                tracing::debug!("GetRegionEntityCount called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetRegionalPlayVarValue",
            |_, _this, (_ctx, _uid, _var_type): (Table, u32, u32)| {
                tracing::debug!("GetRegionalPlayVarValue called");
                Ok(0.0f32)
            },
        );

        methods.add_method(
            "GetRogueCellState",
            |_, _this, (_ctx, _group_id): (Table, u32)| {
                tracing::debug!("GetRogueCellState called");
                Ok(0)
            },
        );

        methods.add_method("GetRogueDiaryDungeonStage", |_, _this, _ctx: Table| {
            tracing::debug!("GetRogueDiaryDungeonStage called");
            Ok(0)
        });

        methods.add_method("GetRogueDiaryRoundAndRoom", |_, _this, _ctx: Table| {
            tracing::debug!("GetRogueDiaryRoundAndRoom called");
            Ok(0)
        });

        methods.add_method(
            "GetRotationByEntityId",
            |_, _this, (_ctx, _entity_id): (Table, u32)| {
                tracing::debug!("GetRotationByEntityId called");
                Ok(0)
            },
        );

        methods.add_method("GetSceneMultiStagePlayUidValue", |_, _this, (_ctx, _group_id, _play_index, _param_name, _uid): (Table, u32, u32, String, u32)| {
            tracing::debug!("GetSceneMultiStagePlayUidValue called");
            Ok(-1)
        });

        methods.add_method("GetSceneOwnerUid", |_, _this, _ctx: Table| {
            tracing::debug!("GetSceneOwnerUid called");
            Ok(0)
        });

        methods.add_method(
            "GetScenePlayBattleHostUid",
            |_, _this, (_ctx, _group_id): (Table, u32)| {
                tracing::debug!("GetScenePlayBattleHostUid called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetScenePlayBattleProgress",
            |_, _this, (_ctx, _group_id): (Table, u32)| {
                tracing::debug!("GetScenePlayBattleProgress called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetScenePlayBattleStage",
            |_, _this, (_ctx, _group_id): (Table, u32)| {
                tracing::debug!("GetScenePlayBattleStage called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetScenePlayBattleStageBeginProgress",
            |_, _this, (_ctx, _group_id): (Table, u32)| {
                tracing::debug!("GetScenePlayBattleStageBeginProgress called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetScenePlayBattleType",
            |_, _this, (_ctx, _group_id): (Table, u32)| {
                tracing::debug!("GetScenePlayBattleType called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetScenePlayBattleUidValue",
            |_, _this, (_ctx, _group_id, _uid, _key): (Table, u32, u32, String)| {
                tracing::debug!("GetScenePlayBattleUidValue called");
                Ok(0)
            },
        );

        methods.add_method("GetSceneTimeSeconds", |_, _this, _ctx: Table| {
            tracing::debug!("GetSceneTimeSeconds called");
            Ok(0)
        });

        methods.add_method("GetSceneUidList", |_, _this, _ctx: Table| {
            tracing::debug!("GetSceneUidList called");
            Ok(0)
        });

        methods.add_method("GetServerTime", |_, _this, _ctx: Table| {
            let time = common::time_util::unix_timestamp() as i64;
            tracing::debug!("[ScriptLib] GetServerTime = {}", time);
            Ok(time)
        });

        methods.add_method(
            "GetServerTimeByDate",
            |_, _this, (_ctx, _year, _month, _day): (Table, i32, i32, i32)| {
                tracing::debug!("GetServerTimeByDate called");
                Ok(0)
            },
        );

        methods.add_method("GetServerTimeByWeek", |_, _this, _ctx: Table| {
            use std::time::{SystemTime, UNIX_EPOCH};
            let secs = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            // Days since Thursday 1970-01-01 (weekday 4), mod 7 to get 1=Mon..7=Sun
            let day_of_week = ((secs / 86400 + 3) % 7 + 1) as i64;
            tracing::debug!("[ScriptLib] GetServerTimeByWeek = {}", day_of_week);
            Ok(day_of_week)
        });

        methods.add_method(
            "GetSurroundUidList",
            |_, _this, (_ctx, _context, _config_id): (Table, u32, u32)| {
                tracing::debug!("GetSurroundUidList called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetTeamAbilityFloatValue",
            |_, _this, (_ctx, _uid, _key): (Table, u32, String)| {
                tracing::debug!("GetTeamAbilityFloatValue called");
                Ok(0.0f32)
            },
        );

        methods.add_method(
            "GetTeamAbilityUIntValue",
            |_, _this, (_ctx, _uid, _key): (Table, u32, String)| {
                tracing::debug!("GetTeamAbilityUIntValue called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetTeamEntityIdByUid",
            |_, _this, (_ctx, _uid): (Table, u32)| {
                tracing::debug!("GetTeamEntityIdByUid called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetTeamServerGlobalValue",
            |_, _this, (_ctx, _uid, _key): (Table, u32, String)| {
                tracing::debug!("GetTeamServerGlobalValue called");
                Ok(0.0f32)
            },
        );

        methods.add_method("GetTimeOffsetSec", |_, _this, _ctx: Table| {
            tracing::debug!("GetTimeOffsetSec called");
            Ok(0)
        });

        methods.add_method(
            "GetTreasureSeelieDayByGroupId",
            |_, _this, (_ctx, _group_id): (Table, u32)| {
                tracing::debug!("GetTreasureSeelieDayByGroupId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetUidByAvatarEntityId",
            |_, _this, (_ctx, _entity_id): (Table, u32)| {
                tracing::debug!("GetUidByAvatarEntityId called");
                Ok(0)
            },
        );

        methods.add_method(
            "GetUidByTeamEntityId",
            |_, _this, (_ctx, _entity_id): (Table, u32)| {
                tracing::debug!("GetUidByTeamEntityId called");
                Ok(0)
            },
        );

        methods.add_method(
            "GoBackGroupSuite",
            |_, _this, (_ctx, _group_id): (Table, u32)| {
                tracing::debug!("GoBackGroupSuite called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GoToFlowSuite",
            |_, _this, (_ctx, _group_id, _suite_index): (Table, u32, u32)| {
                tracing::debug!("GoToFlowSuite called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GoToGroupSuite",
            |_, this, (_ctx, group_id, suite_index): (Table, u32, u32)| {
                this.script_lib.go_to_group_suite(group_id, suite_index);
                Ok(0)
            },
        );

        methods.add_method(
            "HideScenePoint",
            |_, _this, (_ctx, _point_id): (Table, u32)| {
                tracing::debug!("HideScenePoint called");
                Ok(-1)
            },
        );

        methods.add_method(
            "InitGalleryProgressScore",
            |_,
             _this,
             (_ctx, _key, _gallery_id, _progress_table, _ui_form, _progress_type): (
                Table,
                String,
                u32,
                Table,
                u32,
                u32,
            )| {
                tracing::debug!("InitGalleryProgressScore called");
                Ok(-1)
            },
        );

        methods.add_method("InitGalleryProgressWithScore", |_, _this, (_ctx, _key, _gallery_id, _progress_table, _init_score, _ui_form, _progress_type): (Table, String, u32, Table, u32, u32, u32)| {
            tracing::debug!("InitGalleryProgressWithScore called");
            Ok(-1)
        });

        methods.add_method(
            "InitSceneMultistagePlay",
            |_,
             _this,
             (_ctx, _play_index, _play_type, _param_table, _uid_list): (
                Table,
                u32,
                u32,
                Table,
                Table,
            )| {
                tracing::debug!("InitSceneMultistagePlay called");
                Ok(-1)
            },
        );

        methods.add_method(
            "InitTimeAxis",
            |_, this, (_ctx, key, delays_table, should_loop): (Table, String, Table, bool)| {
                let delays: Vec<f64> = delays_table.sequence_values::<f64>().filter_map(|v| v.ok()).collect();
                this.script_lib.init_time_axis(&key, delays, should_loop);
                Ok(0)
            },
        );

        methods.add_method(
            "InstableSprayGetSGVByBuffId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("InstableSprayGetSGVByBuffId called");
                Ok(0)
            },
        );

        methods.add_method(
            "InstableSprayRandomBuffs",
            |_, _this, (_ctx, _context, _gallery_id): (Table, u32, u32)| {
                tracing::debug!("InstableSprayRandomBuffs called");
                Ok(0)
            },
        );

        methods.add_method(
            "InvaildGravenPhotoBundleMark",
            |_, _this, (_ctx, _group_bundle_id): (Table, u32)| {
                tracing::debug!("InvaildGravenPhotoBundleMark called");
                Ok(0)
            },
        );

        methods.add_method(
            "IsChallengeStartedByChallengeId",
            |_, _this, (_ctx, _challenge_id): (Table, u32)| {
                tracing::debug!("IsChallengeStartedByChallengeId called");
                Ok(false)
            },
        );

        methods.add_method(
            "IsChallengeStartedByChallengeIndex",
            |_, _this, (_ctx, _group_id, _challenge_index): (Table, u32, u32)| {
                tracing::debug!("IsChallengeStartedByChallengeIndex called");
                Ok(false)
            },
        );

        methods.add_method(
            "IsChannellerSlabLoopDungeonConditionSelected",
            |_, _this, (_ctx, _condition_id): (Table, u32)| {
                tracing::debug!("IsChannellerSlabLoopDungeonConditionSelected called");
                Ok(false)
            },
        );

        methods.add_method(
            "IsEffigyChallengeConditionSelected",
            |_, _this, (_ctx, _condition_id): (Table, u32)| {
                tracing::debug!("IsEffigyChallengeConditionSelected called");
                Ok(false)
            },
        );

        methods.add_method(
            "IsFungusCaptured",
            |_, _this, (_ctx, _uid, _monster_id): (Table, u32, u32)| {
                tracing::debug!("IsFungusCaptured called");
                Ok(-1)
            },
        );

        methods.add_method(
            "IsGalleryStart",
            |_, _this, (_ctx, _gallery_id): (Table, u32)| {
                tracing::debug!("IsGalleryStart called");
                Ok(false)
            },
        );

        methods.add_method(
            "IsGroupDead",
            |_, _this, (_ctx, _group_id): (Table, u32)| {
                tracing::debug!("IsGroupDead called");
                Ok(false)
            },
        );

        methods.add_method(
            "IsGroupRegisteredInCurScene",
            |_, _this, (_ctx, _group_id): (Table, u32)| {
                tracing::debug!("IsGroupRegisteredInCurScene called");
                Ok(false)
            },
        );

        methods.add_method(
            "IsInRegion",
            |_, _this, (_ctx, _uid, _region_config_id): (Table, u32, u32)| {
                tracing::debug!("IsInRegion called");
                Ok(false)
            },
        );

        methods.add_method(
            "IsInWidgetCaptureMonsterMatchTags",
            |_, _this, (_ctx, _material_id, _monster_id, _capture_tag): (Table, u32, u32, u32)| {
                tracing::debug!("IsInWidgetCaptureMonsterMatchTags called");
                Ok(false)
            },
        );

        methods.add_method(
            "IsLevelTagChangeInCD",
            |_, _this, (_ctx, _level_tag_id): (Table, u32)| {
                tracing::debug!("IsLevelTagChangeInCD called");
                Ok(false)
            },
        );

        methods.add_method(
            "IsPlayerAllAvatarDie",
            |_, _this, (_ctx, _uid): (Table, u32)| {
                tracing::debug!("IsPlayerAllAvatarDie called");
                Ok(false)
            },
        );

        methods.add_method(
            "IsPlayerTransmittable",
            |_, _this, (_ctx, _uid): (Table, u32)| {
                tracing::debug!("IsPlayerTransmittable called");
                Ok(false)
            },
        );

        methods.add_method("IsRogueBossCellPrevCellFinish", |_, _this, _ctx: Table| {
            tracing::debug!("IsRogueBossCellPrevCellFinish called");
            Ok(false)
        });

        methods.add_method(
            "IsWidgetEquipped",
            |_, _this, (_ctx, _uid, _material_id): (Table, u32, u32)| {
                tracing::debug!("IsWidgetEquipped called");
                Ok(false)
            },
        );

        methods.add_method(
            "KillEntityByConfigId",
            |_, this, (ctx, param_table): (Table, Table)| {
                let group_id: u32 = ctx.get("group_id").unwrap_or(0);
                let config_id: u32 = param_table.get("config_id").unwrap_or(0);
                tracing::debug!(
                    "[ScriptLib] KillEntityByConfigId group={} config={}",
                    group_id,
                    config_id
                );
                this.script_lib.kill_entity_by_config_id(group_id, config_id);
                Ok(0)
            },
        );

        methods.add_method(
            "KillExtraFlowSuite",
            |_, _this, (_ctx, _group_id, _suite_index, _policy): (Table, u32, u32, u32)| {
                tracing::debug!("KillExtraFlowSuite called");
                Ok(-1)
            },
        );

        methods.add_method(
            "KillExtraGroupSuite",
            |_, _this, (_ctx, _group_id, _suite_index): (Table, u32, u32)| {
                tracing::debug!("KillExtraGroupSuite called");
                Ok(-1)
            },
        );

        methods.add_method(
            "KillGroupEntity",
            |_, this, (_ctx, param_table): (Table, Table)| {
                let group_id: u32 = param_table.get("group_id").unwrap_or(0);
                let kill_policy: u32 = param_table.get("kill_policy").unwrap_or(0);
                this.script_lib.kill_group_entity(group_id, kill_policy);
                Ok(0)
            },
        );

        methods.add_method(
            "KillMonsterTide",
            |_, _this, (_ctx, _group_id, _tide_index): (Table, u32, u32)| {
                tracing::debug!("KillMonsterTide called");
                Ok(-1)
            },
        );

        methods.add_method(
            "LockCurScenePoint",
            |_, _this, (_ctx, _point_id): (Table, u32)| {
                tracing::debug!("LockCurScenePoint called");
                Ok(-1)
            },
        );

        methods.add_method("LockForce", |_, this, (_ctx, force_id): (Table, u32)| {
            tracing::debug!("[ScriptLib] LockForce {}", force_id);
            this.script_lib.lock_force(force_id);
            Ok(0)
        });

        methods.add_method(
            "LockMonsterHp",
            |_, _this, (_ctx, _config_id): (Table, u32)| {
                tracing::debug!("LockMonsterHp called");
                Ok(-1)
            },
        );

        methods.add_method("MarkGroupLuaAction", |_, _this, (_ctx, _action_str, _transaction, _param_table): (Table, String, String, Table)| {
            tracing::debug!("MarkGroupLuaAction called");
            Ok(-1)
        });

        methods.add_method(
            "MarkPlayerAction",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("MarkPlayerAction called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ModifyClimatePolygonParamTable",
            |_, _this, (_ctx, _climate_area_id, _param_table): (Table, u32, Table)| {
                tracing::debug!("ModifyClimatePolygonParamTable called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ModifyFatherChallengeProperty",
            |_, _this, (_ctx, _father_index, _father_property, _value): (Table, u32, u32, i32)| {
                tracing::debug!("ModifyFatherChallengeProperty called");
                Ok(-1)
            },
        );

        methods.add_method(
            "MoveAvatarByPointArray",
            |_,
             _this,
             (_ctx, _uid, _point_array_id, _point_id_list, _param_table, _client_params): (
                Table,
                u32,
                u32,
                Table,
                Table,
                String,
            )| {
                tracing::debug!("MoveAvatarByPointArray called");
                Ok(-1)
            },
        );

        methods.add_method(
            "MoveAvatarByPointArrayWithTemplate",
            |_,
             _this,
             (_ctx, _uid, _point_array_id, _point_id_list, _template_id, _param_table): (
                Table,
                u32,
                u32,
                Table,
                u32,
                Table,
            )| {
                tracing::debug!("MoveAvatarByPointArrayWithTemplate called");
                Ok(-1)
            },
        );

        methods.add_method(
            "MovePlayerToPos",
            |_, _this, (_ctx, _param_table): (Table, Table)| {
                tracing::debug!("MovePlayerToPos called");
                Ok(-1)
            },
        );

        methods.add_method(
            "NotifyAllPlayerPerformOperation",
            |_,
             _this,
             (_ctx, _entity_id, _operate_type, _operate_index, _pos_table, _rot_table): (
                Table,
                u32,
                u32,
                u32,
                Table,
                Table,
            )| {
                tracing::debug!("NotifyAllPlayerPerformOperation called");
                Ok(-1)
            },
        );

        methods.add_method(
            "NotifyQuestWorktopSelection",
            |_, _this, (_ctx, _group_id, _config_id): (Table, u32, u32)| {
                tracing::debug!("NotifyQuestWorktopSelection called");
                Ok(-1)
            },
        );

        methods.add_method(
            "PauseAutoMonsterTide",
            |_, _this, (_ctx, _group_id, _tide_index): (Table, u32, u32)| {
                tracing::debug!("PauseAutoMonsterTide called");
                Ok(-1)
            },
        );

        methods.add_method(
            "PauseAutoPoolMonsterTide",
            |_, _this, (_ctx, _group_id, _tide_index): (Table, u32, u32)| {
                tracing::debug!("PauseAutoPoolMonsterTide called");
                Ok(-1)
            },
        );

        methods.add_method(
            "PauseChallenge",
            |_, _this, (_ctx, _source_name): (Table, u32)| {
                tracing::debug!("PauseChallenge called");
                Ok(-1)
            },
        );

        methods.add_method(
            "PauseTimeAxis",
            |_, _this, (_ctx, _key): (Table, String)| {
                tracing::debug!("PauseTimeAxis called");
                Ok(-1)
            },
        );

        methods.add_method(
            "PlayCutScene",
            |_, this, (_ctx, cutscene_id, _wait_time): (Table, u32, u32)| {
                tracing::debug!("[ScriptLib] PlayCutScene id={}", cutscene_id);
                this.script_lib.play_cut_scene(cutscene_id);
                Ok(0)
            },
        );

        methods.add_method(
            "PlayCutSceneWithParam",
            |_, _this, (_ctx, _cutscene_id, _wait_time, _param_list): (Table, u32, u32, Table)| {
                tracing::debug!("PlayCutSceneWithParam called");
                Ok(-1)
            },
        );

        methods.add_method(
            "PrestartScenePlayBattle",
            |_, _this, (_ctx, _param_table): (Table, Table)| {
                tracing::debug!("PrestartScenePlayBattle called");
                Ok(-1)
            },
        );

        methods.add_method(
            "PrintContextLog",
            |_, _this, (_ctx, log_str): (Table, String)| {
                tracing::info!("[ScriptLib] [PrintContextLog] {}", log_str);
                Ok(0)
            },
        );

        methods.add_method(
            "PrintGroupWarning",
            |_, _this, (_ctx, _log_str): (Table, String)| {
                tracing::debug!("PrintGroupWarning called");
                Ok(-1)
            },
        );

        methods.add_method("PrintLog", |_, _this, log_str: String| {
            tracing::debug!("PrintLog called {}", log_str);
            Ok(-1)
        });

        methods.add_method(
            "RecieveAllAranaraCollectionByType",
            |_, _this, (_ctx, _group_id, _collection_type): (Table, u32, u32)| {
                tracing::debug!("RecieveAllAranaraCollectionByType called");
                Ok(-1)
            },
        );

        methods.add_method(
            "RecoverDeadGroup",
            |_, _this, (_ctx, _group_id): (Table, u32)| {
                tracing::debug!("RecoverDeadGroup called");
                Ok(-1)
            },
        );

        methods.add_method(
            "RefreshBlossomDropRewardByGroupId",
            |_, _this, (_ctx, _group_id): (Table, u32)| {
                tracing::debug!("RefreshBlossomDropRewardByGroupId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "RefreshBlossomGroup",
            |_, _this, (_ctx, _param_table): (Table, Table)| {
                tracing::debug!("RefreshBlossomGroup called");
                Ok(-1)
            },
        );

        methods.add_method(
            "RefreshGroup",
            |_, this, (_ctx, param_table): (Table, Table)| {
                let group_id: u32 = param_table.get("group_id").unwrap_or(0);
                let suite_id: u32 = param_table.get("suite").unwrap_or(1);
                tracing::debug!(
                    "[ScriptLib] RefreshGroup group={} suite={}",
                    group_id,
                    suite_id
                );
                this.script_lib.refresh_group(group_id, suite_id);
                Ok(0)
            },
        );

        methods.add_method("RefreshHuntingClueGroup", |_, _this, _ctx: Table| {
            tracing::debug!("RefreshHuntingClueGroup called");
            Ok(-1)
        });

        methods.add_method(
            "RemoveEntityByConfigId",
            |_, _this, (_ctx, _group_id, _entity_type, _config_id): (Table, u32, u32, u32)| {
                tracing::debug!("RemoveEntityByConfigId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "RemoveExtraFlowSuite",
            |_, _this, (_ctx, _group_id, _suite_index, _policy): (Table, u32, u32, u32)| {
                tracing::debug!("RemoveExtraFlowSuite called");
                Ok(-1)
            },
        );

        methods.add_method(
            "RemoveExtraGroupSuite",
            |_, _this, (_ctx, group_id, suite_index): (Table, u32, u32)| {
                tracing::debug!("RemoveExtraGroupSuite called");
                _this
                    .script_lib
                    .remove_extra_group_suite(_ctx, group_id, suite_index);
                Ok(0)
            },
        );

        methods.add_method("ResetGadgetState", |_, this, (ctx, state): (Table, u32)| {
            let uid: u32 = ctx.get("uid").unwrap_or(0);
            let group_id: u32 = ctx.get("group_id").unwrap_or(0);
            let config_id: u32 = ctx.get("config_id").unwrap_or(0);
            this.script_lib
                .set_gadget_state_by_config_id(uid, group_id, config_id, state);
            tracing::debug!(
                "ResetGadgetState: group_id={}, config_id={}, state={}",
                group_id,
                config_id,
                state
            );
            Ok(0)
        });

        methods.add_method(
            "ResetGadgetStateByConfigId",
            |_, this, (ctx, config_id, state): (Table, u32, u32)| {
                let uid: u32 = ctx.get("uid").unwrap_or(0);
                let group_id: u32 = ctx.get("group_id").unwrap_or(0);
                this.script_lib
                    .set_gadget_state_by_config_id(uid, group_id, config_id, state);
                tracing::debug!(
                    "ResetGadgetStateByConfigId: group_id={}, config_id={}, state={}",
                    group_id,
                    config_id,
                    state
                );
                Ok(0)
            },
        );

        methods.add_method(
            "ResumeAutoPoolMonsterTide",
            |_, _this, (_ctx, _group_id, _tide_index): (Table, u32, u32)| {
                tracing::debug!("ResumeAutoPoolMonsterTide called");
                Ok(-1)
            },
        );

        methods.add_method(
            "RevertPlayerRegionVision",
            |_, _this, (_ctx, _uid): (Table, u32)| {
                tracing::debug!("RevertPlayerRegionVision called");
                Ok(-1)
            },
        );

        methods.add_method(
            "RevokePlayerShowTemplateReminder",
            |_, _this, (_ctx, _reminder_id, _param_table): (Table, u32, Table)| {
                tracing::debug!("RevokePlayerShowTemplateReminder called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ScenePlayBattleUidOp",
            |_,
             _this,
             (
                _ctx,
                _group_id,
                _config_id,
                _uid_list,
                _op,
                _param_str,
                _param_list,
                _param_target_list,
                _param_index,
                _param_duration,
            ): (Table, u32, u32, Table, u32, String, Table, Table, u32, u32)| {
                tracing::debug!("ScenePlayBattleUidOp called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ScenePlaySound",
            |_, this, (ctx, sound_info): (Table, Table)| {
                let group_id: u32 = ctx.get("group_id").unwrap_or(0);
                let sound_name: String = sound_info.get("sound_name").unwrap_or_default();
                let play_pos: Option<Table> = sound_info.get("play_pos").ok();
                let play_type: u32 = sound_info.get("play_type").unwrap_or(0);
                let pos = if let Some(t) = play_pos {
                    (t.get("x").unwrap_or(0.0f32), t.get("y").unwrap_or(0.0), t.get("z").unwrap_or(0.0))
                } else {
                    (0.0, 0.0, 0.0)
                };
                tracing::debug!(
                    "[ScriptLib] ScenePlaySound group={} name={} type={}",
                    group_id, sound_name, play_type
                );
                this.script_lib.scene_play_sound(&sound_name, pos, play_type);
                Ok(0)
            },
        );

        methods.add_method(
            "SendServerMessageByLuaKey",
            |_, _this, (_ctx, _key_name, _uid_list): (Table, String, Table)| {
                tracing::debug!("SendServerMessageByLuaKey called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetBlossomScheduleStateByGroupId",
            |_, _this, (_ctx, _group_id, _state): (Table, u32, u32)| {
                tracing::debug!("SetBlossomScheduleStateByGroupId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetChainLevel",
            |_, _this, (_ctx, _chain_id, _target_level, _is_notify): (Table, u32, u32, bool)| {
                tracing::debug!("SetChainLevel called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetChallengeDuration",
            |_, _this, (_ctx, _challenge_index, _value): (Table, u32, u32)| {
                tracing::debug!("SetChallengeDuration called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetChallengeEventMark",
            |_, _this, (_ctx, _challenge_index, _mark_type): (Table, u32, u32)| {
                tracing::debug!("SetChallengeEventMark called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetChessMystery",
            |_, _this, (_ctx, _group_id, _play_index, _param_table): (Table, u32, u32, Table)| {
                tracing::debug!("SetChessMystery called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetCurFungusFighterTrainingParams",
            |_, _this, (_ctx, _param_table): (Table, Table)| {
                tracing::debug!("SetCurFungusFighterTrainingParams called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetDarkPressureLevel",
            |_, _this, (_ctx, _level): (Table, u32)| {
                tracing::debug!("SetDarkPressureLevel called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetEntityServerGlobalValueByConfigId",
            |_, this, (ctx, config_id, key, value): (Table, u32, String, f32)| {
                let group_id: u32 = ctx.get("group_id").unwrap_or(0);
                tracing::debug!(
                    "[ScriptLib] SetEntityServerGlobalValueByConfigId group={} config={} key={} val={}",
                    group_id, config_id, key, value
                );
                this.script_lib.set_entity_server_global_value(group_id, config_id, &key, value as u32);
                Ok(0)
            },
        );

        methods.add_method(
            "SetEntityServerGlobalValueByEntityId",
            |_, _this, (_ctx, _entity_id, _key, _value): (Table, u32, String, f32)| {
                tracing::debug!("SetEntityServerGlobalValueByEntityId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetEnvironmentEffectState",
            |_,
             _this,
             (_ctx, _type, _effect_alias, _float_param, _int_param): (
                Table,
                u32,
                String,
                Table,
                Table,
            )| {
                tracing::debug!("SetEnvironmentEffectState called");
                Ok(-1)
            },
        );

        methods.add_method("SetFleurFairMultistagePlayBuffEnergy", |_, _this, (_ctx, _group_id, _play_index, _uid, _value): (Table, u32, u32, u32, u32)| {
            tracing::debug!("SetFleurFairMultistagePlayBuffEnergy called");
            Ok(-1)
        });

        methods.add_method(
            "SetFlowSuite",
            |_, _this, (_ctx, _group_id, _suite_index): (Table, u32, u32)| {
                tracing::debug!("SetFlowSuite called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetGadgetEnableInteract",
            |_, this, (_ctx, group_id, config_id, enable): (Table, u32, u32, bool)| {
                this.script_lib.set_gadget_enable_interact(group_id, config_id, enable);
                Ok(0)
            },
        );

        methods.add_method(
            "SetGadgetHp",
            |_, _this, (_ctx, _group_id, _config_id, _hp_percent): (Table, u32, u32, f32)| {
                tracing::debug!("SetGadgetHp called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetGadgetPlayUidValue",
            |_,
             _this,
             (_ctx, _group_id, _config_id, _uid, _key, _value): (
                Table,
                u32,
                u32,
                u32,
                String,
                i32,
            )| {
                tracing::debug!("SetGadgetPlayUidValue called");
                Ok(-1)
            },
        );

        methods.add_method("SetGadgetState", |_, this, (ctx, state): (Table, u32)| {
            let uid: u32 = ctx.get("uid").unwrap_or(0);
            let group_id: u32 = ctx.get("group_id").unwrap_or(0);
            let config_id: u32 = ctx.get("config_id").unwrap_or(0);
            this.script_lib
                .set_gadget_state_by_config_id(uid, group_id, config_id, state);
            tracing::debug!(
                "SetGadgetState: group_id={}, config_id={}, state={}",
                group_id,
                config_id,
                state
            );
            Ok(0)
        });

        methods.add_method(
            "SetGadgetStateByConfigId",
            |_, this, (ctx, config_id, state): (Table, u32, u32)| {
                let uid: u32 = ctx.get("uid").unwrap_or(0);
                let group_id: u32 = ctx.get("group_id").unwrap_or(0);
                this.script_lib
                    .set_gadget_state_by_config_id(uid, group_id, config_id, state);
                tracing::debug!(
                    "[ScriptLib] SetGadgetStateByConfigId group_id={} config={} state={}",
                    group_id,
                    config_id,
                    state
                );
                Ok(0)
            },
        );

        methods.add_method(
            "SetGadgetTalkByConfigId",
            |_, _this, (_ctx, _group_id, _config_id, _talk_state): (Table, u32, u32, u32)| {
                tracing::debug!("SetGadgetTalkByConfigId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetGalleryProgressScore",
            |_, _this, (_ctx, _key, _gallery_id, _score): (Table, String, u32, u32)| {
                tracing::debug!("SetGalleryProgressScore called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetGalleryRevivePoint",
            |_, _this, (_ctx, _gallery_id, _group_id, _config_id): (Table, u32, u32, u32)| {
                tracing::debug!("SetGalleryRevivePoint called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetGearStartValue",
            |_, _this, (_ctx, _start_value): (Table, i32)| {
                tracing::debug!("SetGearStartValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetGearStopValue",
            |_, _this, (_ctx, _stop_value): (Table, i32)| {
                tracing::debug!("SetGearStopValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetGroupDead",
            |_, _this, (_ctx, _group_id): (Table, u32)| {
                tracing::debug!("SetGroupDead called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetGroupGadgetStateByConfigId",
            |_, this, (_ctx, group_id, config_id, gadget_state): (Table, u32, u32, u32)| {
                this.script_lib.set_group_gadget_state_by_config_id(group_id, config_id, gadget_state);
                Ok(0)
            },
        );

        methods.add_method(
            "SetGroupLogicStateValue",
            |_, _this, (_ctx, _name, _value): (Table, String, i32)| {
                tracing::debug!("SetGroupLogicStateValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetGroupLogicStateValueByGroup",
            |_, _this, (_ctx, _name, _value, _group_id): (Table, String, i32, u32)| {
                tracing::debug!("SetGroupLogicStateValueByGroup called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetGroupReplaceable",
            |_, _this, (_ctx, _group_id, _is_replaceable): (Table, u32, bool)| {
                tracing::debug!("SetGroupReplaceable called");
                Ok(false)
            },
        );

        methods.add_method(
            "SetGroupTempValue",
            |_, _this, (_ctx, _key, _value, _param_table): (Table, String, i32, Table)| {
                tracing::debug!("SetGroupTempValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetGroupVariableValue",
            |_, _this, (ctx, name, value): (Table, String, i32)| {
                let group_id: u32 = ctx.get("group_id").unwrap_or(0);
                let result = _this
                    .script_lib
                    .set_group_variable_value(group_id, &name, value);
                tracing::debug!(
                    "SetGroupVariableValue group_id={} name={} value={} -> {}",
                    group_id,
                    name,
                    value,
                    result
                );
                Ok(result)
            },
        );

        methods.add_method(
            "SetGroupVariableValueByGroup",
            |_, this, (_ctx, name, value, group_id): (Table, String, i32, u32)| {
                let result = this
                    .script_lib
                    .set_group_variable_value_by_group(group_id, &name, value);
                tracing::debug!(
                    "SetGroupVariableValueByGroup group_id={} name={} value={} -> {}",
                    group_id,
                    name,
                    value,
                    result
                );
                Ok(result)
            },
        );

        methods.add_method(
            "SetHandballGalleryBallPosAndRot",
            |_, _this, (_ctx, _gallery_id, _pos_table, _rot_table): (Table, u32, Table, Table)| {
                tracing::debug!("SetHandballGalleryBallPosAndRot called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetIsAllowUseSkill",
            |_, this, (_ctx, can_use): (Table, u32)| {
                tracing::debug!("[ScriptLib] SetIsAllowUseSkill canUse={}", can_use);
                this.script_lib.set_is_allow_use_skill(can_use == 1);
                Ok(0)
            },
        );

        methods.add_method(
            "SetLanternRiteValue",
            |_, _this, (_ctx, _value): (Table, u32)| {
                tracing::debug!("SetLanternRiteValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetLimitOptimization",
            |_, _this, (_ctx, _uid, _is_active): (Table, u32, bool)| {
                tracing::debug!("SetLimitOptimization called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetMechanicusChallengeState",
            |_,
             _this,
             (_ctx, _group_id, _play_index, _card_id, _effect_id, _state): (
                Table,
                u32,
                u32,
                i32,
                i32,
                u32,
            )| {
                tracing::debug!("SetMechanicusChallengeState called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetMechanicusMonsterPoolVec",
            |_, _this, (_ctx, _group_id, _play_index, _param_table): (Table, u32, u32, Table)| {
                tracing::debug!("SetMechanicusMonsterPoolVec called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetMistTrialServerGlobalValue",
            |_, _this, (_ctx, _floor_level): (Table, u32)| {
                tracing::debug!("SetMistTrialServerGlobalValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetMonsterBattleByGroup",
            |_, _this, (_ctx, _config_id, _group_id): (Table, u32, u32)| {
                tracing::debug!("SetMonsterBattleByGroup called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetMonsterHp",
            |_, _this, (_ctx, _group_id, _config_id, _hp_percent): (Table, u32, u32, f32)| {
                tracing::debug!("SetMonsterHp called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetPlatformPointArray",
            |_,
             _this,
             (_ctx, _config_id, _point_array_id, _point_id_list, _param_table): (
                Table,
                u32,
                u32,
                Table,
                Table,
            )| {
                tracing::debug!("SetPlatformPointArray called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetPlatformRouteId",
            |_, this, (ctx, config_id, route_id): (Table, u32, u32)| {
                let group_id: u32 = ctx.get("group_id").unwrap_or(0);
                tracing::debug!(
                    "[ScriptLib] SetPlatformRouteId group={} config={} route={}",
                    group_id, config_id, route_id
                );
                this.script_lib.set_platform_route_id(group_id, config_id, route_id);
                Ok(0)
            },
        );

        methods.add_method(
            "SetPlatformRouteIndexToNext",
            |_, _this, (_ctx, _config_id): (Table, u32)| {
                tracing::debug!("SetPlatformRouteIndexToNext called");
                Ok(-1)
            },
        );

        methods.add_method("SetPlayerEyePoint", |_, _this, (_ctx, _target_region_config_id, _related_big_region_config_id): (Table, u32, u32)| {
            tracing::debug!("SetPlayerEyePoint called");
            Ok(-1)
        });

        methods.add_method(
            "SetPlayerEyePointImpl",
            |_,
             _this,
             (
                _ctx,
                _target_region_config_id,
                _related_big_region_config_id,
                _is_stream,
                _fix_lod_level,
            ): (Table, i32, i32, bool, i32)| {
                tracing::debug!("SetPlayerEyePointImpl called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetPlayerEyePointLOD",
            |_,
             _this,
             (_ctx, _target_region_config_id, _related_big_region_config_id, _fix_lod_level): (
                Table,
                u32,
                u32,
                i32,
            )| {
                tracing::debug!("SetPlayerEyePointLOD called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetPlayerEyePointStream",
            |_,
             _this,
             (_ctx, _target_region_config_id, _related_big_region_config_id, _is_stream): (
                Table,
                u32,
                u32,
                bool,
            )| {
                tracing::debug!("SetPlayerEyePointStream called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetPlayerGroupVisionType",
            |_, _this, (_ctx, _uid_list, _type_list): (Table, Table, Table)| {
                tracing::debug!("SetPlayerGroupVisionType called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetPlayerInteractOption",
            |_, _this, (_ctx, _option): (Table, String)| {
                tracing::debug!("SetPlayerInteractOption called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetPlayerStartGallery",
            |_, _this, (_ctx, _gallery_id, _uid_list): (Table, u32, Table)| {
                tracing::debug!("SetPlayerStartGallery called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetRegionalPlayVarValue",
            |_, _this, (_ctx, _uid, _var_type, _var_value): (Table, u32, u32, f32)| {
                tracing::debug!("SetRegionalPlayVarValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetRogueCellState",
            |_, _this, (_ctx, _group_id, _target_state): (Table, u32, u32)| {
                tracing::debug!("SetRogueCellState called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetSceneMultiStagePlayUidValue",
            |_,
             _this,
             (_ctx, _group_id, _play_index, _param_name, _uid, _value): (
                Table,
                u32,
                u32,
                String,
                u32,
                i32,
            )| {
                tracing::debug!("SetSceneMultiStagePlayUidValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetSceneMultiStagePlayValue",
            |_,
             _this,
             (_ctx, _play_index, _param_name, _value, _is_notify): (
                Table,
                u32,
                String,
                u32,
                bool,
            )| {
                tracing::debug!("SetSceneMultiStagePlayValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetSceneMultiStagePlayValues",
            |_, _this, (_ctx, _play_index, _param_table, _is_notify): (Table, u32, Table, bool)| {
                tracing::debug!("SetSceneMultiStagePlayValues called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetScenePlayBattlePlayTeamEntityGadgetId",
            |_, _this, (_ctx, _group_id, _gadget_id): (Table, u32, u32)| {
                tracing::debug!("SetScenePlayBattlePlayTeamEntityGadgetId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetScenePlayBattleUidValue",
            |_, _this, (_ctx, _group_id, _uid, _key, _value): (Table, u32, u32, String, u32)| {
                tracing::debug!("SetScenePlayBattleUidValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetTeamEntityGlobalFloatValue",
            |_, _this, (_ctx, _param_table, _key, _value): (Table, Table, String, f32)| {
                tracing::debug!("SetTeamEntityGlobalFloatValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetTeamServerGlobalValue",
            |_, _this, (_ctx, _uid, _key, _value): (Table, u32, String, f32)| {
                tracing::debug!("SetTeamServerGlobalValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetWeatherAreaState",
            |_, this, (_ctx, height_area_id, open_weather): (Table, u32, u32)| {
                tracing::debug!(
                    "[ScriptLib] SetWeatherAreaState area={} climate={}",
                    height_area_id, open_weather
                );
                this.script_lib.set_weather_area_state(height_area_id, open_weather);
                Ok(0)
            },
        );

        methods.add_method(
            "SetWidgetClientDetectorCoolDown",
            |_, _this, (_ctx, _material_id, _is_success): (Table, u32, bool)| {
                tracing::debug!("SetWidgetClientDetectorCoolDown called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetWorktopOptions",
            |_, _this, (_ctx, _option_table): (Table, Table)| {
                tracing::debug!("SetWorktopOptions called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetWorktopOptionsByGroupId",
            |_, this, (_ctx, group_id, config_id, option_table): (Table, u32, u32, Table)| {
                tracing::debug!(
                    "[ScriptLib] SetWorktopOptionsByGroupId group={} config={}",
                    group_id,
                    config_id
                );
                let mut option_list: Vec<u32> = Vec::new();

                for pair in option_table.sequence_values::<u32>() {
                    let value = pair?;
                    option_list.push(value);
                }

                this.script_lib
                    .set_worktop_options_by_group_id(group_id, config_id, option_list);

                Ok(0)
            },
        );

        methods.add_method(
            "ShowClientGuide",
            |_, this, (_ctx, guide_name): (Table, String)| {
                tracing::debug!("[ScriptLib] ShowClientGuide name={}", guide_name);
                this.script_lib.show_client_guide(&guide_name);
                Ok(0)
            },
        );

        methods.add_method(
            "ShowClientTutorial",
            |_, _this, (_ctx, _tutorial_id, _uid_list): (Table, u32, Table)| {
                tracing::debug!("ShowClientTutorial called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ShowCommonPlayerTips",
            |_, _this, (_ctx, _notify_type, _text_map_id_list): (Table, u32, Table)| {
                tracing::debug!("ShowCommonPlayerTips called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ShowReminder",
            |_, this, (_ctx, reminder_id): (Table, u32)| {
                tracing::debug!("[ScriptLib] ShowReminder id={}", reminder_id);
                this.script_lib.show_reminder(reminder_id);
                Ok(0)
            },
        );

        methods.add_method(
            "ShowReminderByUid",
            |_, _this, (_ctx, _uid_list, _reminder_id): (Table, Table, u32)| {
                tracing::debug!("ShowReminderByUid called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ShowReminderRadius",
            |_, _this, (_ctx, _reminder_id, _center, _radius): (Table, u32, Table, u32)| {
                tracing::debug!("ShowReminderRadius called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ShowTemplateReminder",
            |_, _this, (_ctx, _reminder_id, _param_list): (Table, u32, Table)| {
                tracing::debug!("ShowTemplateReminder called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SkipTeyvatTime",
            |_, _this, (_ctx, _skip_teyvat_minute, _sim_speed): (Table, u32, u32)| {
                tracing::debug!("SkipTeyvatTime called");
                Ok(-1)
            },
        );

        methods.add_method("StartChallenge", |_, _this, (_ctx, _challenge_index, _challenge_id, _param_table): (Table, u32, u32, Table)| {
            tracing::debug!("StartChallenge called");
            Ok(-1)
        });

        methods.add_method(
            "StartFatherChallenge",
            |_, _this, (_ctx, _father_index): (Table, u32)| {
                tracing::debug!("StartFatherChallenge called");
                Ok(-1)
            },
        );

        methods.add_method(
            "StartGadgetPlay",
            |_, _this, (_ctx, _group_id, _config_id): (Table, u32, u32)| {
                tracing::debug!("StartGadgetPlay called");
                Ok(-1)
            },
        );

        methods.add_method(
            "StartGallery",
            |_, _this, (_ctx, _gallery_id): (Table, u32)| {
                tracing::debug!("StartGallery called");
                Ok(-1)
            },
        );

        methods.add_method(
            "StartHomeGallery",
            |_, _this, (_ctx, _gallery_id, _owner_uid): (Table, u32, u32)| {
                tracing::debug!("StartHomeGallery called");
                Ok(-1)
            },
        );

        methods.add_method(
            "StartPlatform",
            |_, this, (ctx, config_id): (Table, u32)| {
                let group_id: u32 = ctx.get("group_id").unwrap_or(0);
                tracing::debug!("[ScriptLib] StartPlatform group={} config={}", group_id, config_id);
                this.script_lib.start_platform(group_id, config_id);
                Ok(0)
            },
        );

        methods.add_method(
            "StartSceneMultiStagePlayStage",
            |_,
             _this,
             (_ctx, _play_index, _duration, _stage_type, _stage_name, _param_table): (
                Table,
                u32,
                u32,
                u32,
                String,
                Table,
            )| {
                tracing::debug!("StartSceneMultiStagePlayStage called");
                Ok(-1)
            },
        );

        methods.add_method(
            "StartSealBattle",
            |_, _this, (_ctx, _config_id, _seal_table): (Table, u32, Table)| {
                tracing::debug!("StartSealBattle called");
                Ok(-1)
            },
        );

        methods.add_method(
            "StopChallenge",
            |_, this, (ctx, challenge_index, is_success): (Table, u32, bool)| {
                let group_id: u32 = ctx.get("group_id").unwrap_or(0);
                tracing::debug!(
                    "[ScriptLib] StopChallenge group={} index={} is_success={}",
                    group_id,
                    challenge_index,
                    is_success
                );
                this.script_lib
                    .stop_challenge(group_id, challenge_index, is_success);
                Ok(0)
            },
        );

        methods.add_method("StopFishing", |_, _this, (_ctx, _uid): (Table, u32)| {
            tracing::debug!("StopFishing called");
            Ok(-1)
        });

        methods.add_method(
            "StopGadgetPlay",
            |_, _this, (_ctx, _group_id, _config_id): (Table, u32, u32)| {
                tracing::debug!("StopGadgetPlay called");
                Ok(-1)
            },
        );

        methods.add_method(
            "StopGallery",
            |_, _this, (_ctx, _gallery_id, _is_fail): (Table, u32, bool)| {
                tracing::debug!("StopGallery called");
                Ok(-1)
            },
        );

        methods.add_method(
            "StopGalleryByReason",
            |_, _this, (_ctx, _gallery_id, _reason): (Table, u32, u32)| {
                tracing::debug!("StopGalleryByReason called");
                Ok(-1)
            },
        );

        methods.add_method(
            "StopPlatform",
            |_, this, (ctx, config_id): (Table, u32)| {
                let group_id: u32 = ctx.get("group_id").unwrap_or(0);
                tracing::debug!("[ScriptLib] StopPlatform group={} config={}", group_id, config_id);
                this.script_lib.stop_platform(group_id, config_id);
                Ok(0)
            },
        );

        methods.add_method(
            "StopReminder",
            |_, _this, (_ctx, _reminder_id): (Table, u32)| {
                tracing::debug!("StopReminder called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SwitchSceneEnvAnimal",
            |_, _this, (_ctx, _type): (Table, u32)| {
                tracing::debug!("SwitchSceneEnvAnimal called");
                Ok(-1)
            },
        );

        methods.add_method(
            "TowerCountTimeStatus",
            |_, _this, (_ctx, _is_stop): (Table, u32)| {
                tracing::debug!("TowerCountTimeStatus called");
                Ok(-1)
            },
        );

        methods.add_method(
            "TowerMirrorTeamSetUp",
            |_, _this, (_ctx, _tower_team_id): (Table, u32)| {
                tracing::debug!("TowerMirrorTeamSetUp called");
                Ok(-1)
            },
        );

        methods.add_method(
            "TransPlayerToPos",
            |_, this, (_ctx, param_table): (Table, Table)| {
                let uid_list: Vec<u32> = param_table.get("uid_list").unwrap_or_default();
                let pos_table: Option<Table> = param_table.get("pos").ok();
                let pos = match pos_table {
                    Some(t) => (t.get("x").unwrap_or(0.0), t.get("y").unwrap_or(0.0), t.get("z").unwrap_or(0.0)),
                    None => (0.0, 0.0, 0.0),
                };
                let rot = (0.0f32, 0.0, 0.0);
                let scene_id: u32 = param_table.get("scene_id").unwrap_or(0);
                let radius: f32 = param_table.get("radius").unwrap_or(0.0);
                this.script_lib.trans_player_to_pos(uid_list, pos, rot, scene_id, radius);
                Ok(0)
            },
        );

        methods.add_method(
            "TreasureSeelieCollectOrbsNotify",
            |_, _this, (_ctx, _current_num, _total_num): (Table, u32, u32)| {
                tracing::debug!("TreasureSeelieCollectOrbsNotify called");
                Ok(0)
            },
        );

        methods.add_method(
            "TriggerRoguelikeCurseByLua",
            |_, _this, (_ctx, _uid): (Table, u32)| {
                tracing::debug!("TriggerRoguelikeCurseByLua called");
                Ok(-1)
            },
        );

        methods.add_method(
            "TryFinishLuminanceStoneChallengeStage",
            |_, _this, (_ctx, _group_id): (Table, u32)| {
                tracing::debug!("TryFinishLuminanceStoneChallengeStage called");
                Ok(-1)
            },
        );

        methods.add_method(
            "TryReallocateEntityAuthority",
            |_, _this, (_ctx, _uid, _config_id, _region_config_id): (Table, u32, u32, u32)| {
                tracing::debug!("TryReallocateEntityAuthority called");
                Ok(-1)
            },
        );

        methods.add_method(
            "TryRecordActivityPushTips",
            |_, _this, (_ctx, _activity_push_tips_id): (Table, u32)| {
                tracing::debug!("TryRecordActivityPushTips called");
                Ok(-1)
            },
        );

        methods.add_method(
            "UnfreezeGroupLimit",
            |_, _this, (_ctx, _point_id): (Table, u32)| {
                tracing::debug!("UnfreezeGroupLimit called");
                Ok(-1)
            },
        );

        methods.add_method(
            "UnhideScenePoint",
            |_, _this, (_ctx, _point_id): (Table, u32)| {
                tracing::debug!("UnhideScenePoint called");
                Ok(-1)
            },
        );

        methods.add_method(
            "UnlockFloatSignal",
            |_, _this, (_ctx, _group_id, _config_id): (Table, u32, u32)| {
                tracing::debug!("UnlockFloatSignal called");
                Ok(-1)
            },
        );

        methods.add_method(
            "UnlockForce",
            |_, this, (_ctx, force_id): (Table, u32)| {
                tracing::debug!("[ScriptLib] UnlockForce {}", force_id);
                this.script_lib.unlock_force(force_id);
                Ok(0)
            },
        );

        methods.add_method(
            "UnlockMonsterHp",
            |_, _this, (_ctx, _config_id): (Table, u32)| {
                tracing::debug!("UnlockMonsterHp called");
                Ok(-1)
            },
        );

        methods.add_method(
            "UnlockScenePoint",
            |_, _this, (_ctx, _point_id): (Table, u32)| {
                tracing::debug!("UnlockScenePoint called");
                Ok(-1)
            },
        );

        methods.add_method(
            "UpdatePlayerGalleryScore",
            |_, _this, (_ctx, _gallery_id, _param_table): (Table, u32, Table)| {
                tracing::debug!("UpdatePlayerGalleryScore called");
                Ok(-1)
            },
        );

        methods.add_method(
            "UpdatePlayerValidPos",
            |_, _this, (_ctx, _param_table): (Table, Table)| {
                tracing::debug!("UpdatePlayerValidPos called");
                Ok(-1)
            },
        );

        methods.add_method(
            "UpdateStakeHomePlayRecord",
            |_, _this, (_ctx, _uid_list): (Table, Table)| {
                tracing::debug!("UpdateStakeHomePlayRecord called");
                Ok(-1)
            },
        );

        methods.add_method(
            "VintageFinishGroupByPresentId",
            |_, _this, (_ctx, _present_id): (Table, u32)| {
                tracing::debug!("VintageFinishGroupByPresentId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "WinterCampGetBattleGroupBundleId",
            |_, _this, _ctx: Table| {
                tracing::debug!("WinterCampGetBattleGroupBundleId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "WinterCampGetExploreGroupBundleId",
            |_, _this, _ctx: Table| {
                tracing::debug!("WinterCampGetExploreGroupBundleId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "WinterCampSnowDriftInteract",
            |_, _this, (_ctx, _config_id): (Table, u32)| {
                tracing::debug!("WinterCampSnowDriftInteract called");
                Ok(-1)
            },
        );

        methods.add_method(
            "findGadgetByConfigId",
            |_, _this, (_ctx, _group_id, _config_id): (Table, i32, i32)| {
                tracing::debug!("findGadgetByConfigId called");
                Ok(0)
            },
        );

        methods.add_method(
            "findScenePlayBattlePtr",
            |_, _this, (_ctx, _group_id): (Table, i32)| {
                tracing::debug!("findScenePlayBattlePtr called");
                Ok(0)
            },
        );

        methods.add_method(
            "getConfigIdByEntityId",
            |_, _this, (_ctx, _key_name, _param_table): (Table, String, Table)| {
                tracing::debug!("getConfigIdByEntityId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "getOrCreateExhibitionTemporaryData",
            |_, _this, (_ctx, _play_param): (Table, Table)| {
                tracing::debug!("getOrCreateExhibitionTemporaryData called");
                Ok(0)
            },
        );

        methods.add_method(
            "getPlayerFromSceneOwnerOrThread",
            |_, _this, (_ctx, _uid): (Table, i32)| {
                tracing::debug!("getPlayerFromSceneOwnerOrThread called");
                Ok(0)
            },
        );

        methods.add_method(
            "getPlayerPtrByExhibitionId",
            |_, _this, (_ctx, _uid, _exhibition_id): (Table, i32, i32)| {
                tracing::debug!("getPlayerPtrByExhibitionId called");
                Ok(0)
            },
        );

        methods.add_method(
            "internalExecuteGroupLua",
            |_,
             _this,
             (_ctx, _group_id, _func_name, _param_list, _is_force_load_group): (
                Table,
                i32,
                String,
                Table,
                bool,
            )| {
                tracing::debug!("internalExecuteGroupLua called");
                Ok(-1)
            },
        );

        methods.add_method(
            "internalMoveAvatarByPointArray",
            |_,
             _this,
             (
                _ctx,
                _uid,
                _point_array_id,
                _point_id_list,
                _param_table,
                _client_params,
                _template_id,
            ): (Table, i32, i32, Table, Table, String, i32)| {
                tracing::debug!("internalMoveAvatarByPointArray called");
                Ok(-1)
            },
        );

        methods.add_method("killEntity", |_, _this, _entity_id: u32| {
            tracing::debug!("killEntity called");
            Ok(-1)
        });

        methods.add_method(
            "notifyServerErrorLog",
            |_, _this, (_ctx, _file_path, _func_name, _e): (Table, String, String, u32)| {
                tracing::debug!("notifyServerErrorLog called");
                Ok(0)
            },
        );

        methods.add_method(
            "registerLib",
            |_, _this, (_ctx, _lua_state_ptr): (u32, u32)| {
                tracing::debug!("registerLib called");
                Ok(-1)
            },
        );

        methods.add_method("sendCloseCommonTipsToClient", |_, this, _ctx: Table| {
            tracing::debug!("[ScriptLib] sendCloseCommonTipsToClient");
            this.script_lib.close_common_tips();
            Ok(0)
        });

        methods.add_method(
            "sendShowCommonTipsToClient",
            |_, this, (_ctx, title, content, close_time): (Table, String, String, u32)| {
                tracing::debug!("[ScriptLib] sendShowCommonTipsToClient title={}", title);
                this.script_lib.show_common_tips(&title, &content, close_time);
                Ok(0)
            },
        );

        methods.add_method(
            "updateBundleMarkShowStateByGroupId",
            |_, _this, (_ctx, _group_id, _is_show): (Table, u32, bool)| {
                tracing::debug!("updateBundleMarkShowStateByGroupId called");
                Ok(-1)
            },
        );
    }
}
