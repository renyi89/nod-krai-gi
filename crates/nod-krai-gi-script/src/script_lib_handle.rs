use crate::script_lib::ScriptLib;
use mlua::{Table, UserData, UserDataMethods};
use std::sync::Arc;

pub struct LuaScriptLibHandle {
    pub script_lib: Arc<dyn ScriptLib>,
}

impl UserData for LuaScriptLibHandle {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "ActivateDungeonCheckPoint",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("ActivateDungeonCheckPoint called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ActivateGroupLinkBundle",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("ActivateGroupLinkBundle called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ActiveChallenge",
            |_,
             _this,
             (_ctx, _param1, _param2, _param3, _param4, _param5, _param6): (
                Table,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            )| {
                tracing::debug!("ActiveChallenge called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ActiveGadgetItemGiving",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("ActiveGadgetItemGiving called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddBlossomScheduleProgressByGroupId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("AddBlossomScheduleProgressByGroupId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddEntityGlobalFloatValueByConfigId",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, Table, String, u32)| {
                tracing::debug!("AddEntityGlobalFloatValueByConfigId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddExhibitionAccumulableData",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, String, u32)| {
                tracing::debug!("AddExhibitionAccumulableData called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddExtraFlowSuite",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("AddExtraFlowSuite called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddExtraGroupSuite",
            |_, _this, (_ctx, group_id, suite_id): (Table, u32, u32)| {
                tracing::debug!("AddExtraGroupSuite called");
                _this
                    .script_lib
                    .add_extra_group_suite(_ctx, group_id, suite_id);
                Ok(0)
            },
        );

        methods.add_method(
            "AddPlayerGroupVisionType",
            |_, _this, (_ctx, _param1, _param2): (Table, Table, Table)| {
                tracing::debug!("AddPlayerGroupVisionType called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddQuestProgress",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("AddQuestProgress called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddRegionalPlayVarValue",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("AddRegionalPlayVarValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddSceneTag",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("AddSceneTag called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddTeamEntityGlobalFloatValue",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, String, u32)| {
                tracing::debug!("AddTeamEntityGlobalFloatValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AssignPlayerShowTemplateReminder",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, Table)| {
                tracing::debug!("AssignPlayerShowTemplateReminder called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AttachChildChallenge",
            |_,
             _this,
             (_ctx, _param1, _param2, _param3, _param4, _param5, _param6): (
                Table,
                u32,
                u32,
                u32,
                Table,
                Table,
                Table,
            )| {
                tracing::debug!("AttachChildChallenge called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AttachGalleryAbilityGroup",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, Table, u32, u32)| {
                tracing::debug!("AttachGalleryAbilityGroup called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AutoMonsterTide",
            |_,
             _this,
             (_ctx, _param1, _param2, _param3, _param4, _param5, _param6): (
                Table,
                u32,
                u32,
                Vec<u32>,
                u32,
                u32,
                u32,
            )| {
                tracing::debug!("AutoMonsterTide called");
                Ok(-1)
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
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("CancelGroupTimerEvent called");
                Ok(-1)
            },
        );

        methods.add_method("CauseDungeonFail", |_, _this, _ctx: Table| {
            tracing::debug!("CauseDungeonFail called");
            Ok(-1)
        });

        methods.add_method(
            "ChangeDeathZone",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, Table)| {
                tracing::debug!("ChangeDeathZone called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ChangeGroupGadget",
            |_, _this, (_ctx, _param1): (Table, Table)| {
                tracing::debug!("ChangeGroupGadget called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ChangeGroupVariableValue",
            |_, _this, (_ctx, _param1, _param2): (Table, String, u32)| {
                tracing::debug!("ChangeGroupVariableValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ChangeGroupVariableValueByGroup",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, String, u32, u32)| {
                tracing::debug!("ChangeGroupVariableValueByGroup called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ChangeToTargetLevelTag",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("ChangeToTargetLevelTag called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ChangeToTargetLevelTagWith_paramTable",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, Table)| {
                tracing::debug!("ChangeToTargetLevelTagWith_paramTable called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CheckIsInGroup",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("CheckIsInGroup called");
                Ok(-1)
            },
        );

        methods.add_method("CheckIsInMpMode", |_, _this, _ctx: Table| {
            tracing::debug!("CheckIsInMpMode called");
            Ok(-1)
        });

        methods.add_method(
            "CheckRemainGadgetCountByGroupId",
            |_, _this, (_ctx, _param1): (Table, Table)| {
                tracing::debug!("CheckRemainGadgetCountByGroupId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CheckSceneTag",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("CheckSceneTag called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ClearPlayerEyePoint",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("ClearPlayerEyePoint called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ClearPoolMonsterTide",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("ClearPoolMonsterTide called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ContinueTimeAxis",
            |_, _this, (_ctx, _param1): (Table, String)| {
                tracing::debug!("ContinueTimeAxis called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateBlossomChestByGroupId",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("CreateBlossomChestByGroupId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateChannellerSlabCampRewardGadget",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("CreateChannellerSlabCampRewardGadget called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateEffigyChallengeMonster",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, Table)| {
                tracing::debug!("CreateEffigyChallengeMonster called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateFatherChallenge",
            |_,_this, (_ctx, _param1, _param2, _param3, _param4): (Table, u32, u32, u32, Table)| {
                tracing::debug!("CreateFatherChallenge called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateGadget",
            |_, _this, (_ctx, _param1): (Table, Table)| {
                tracing::debug!("CreateGadget called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateGadgetByConfigIdByPos",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, Table, Table)| {
                tracing::debug!("CreateGadgetByConfigIdByPos called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateGroupTimerEvent",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("CreateGroupTimerEvent called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateGroupVariable",
            |_, _this, (_ctx, _param1, _param2): (Table, String, u32)| {
                tracing::debug!("CreateGroupVariable called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateMonster",
            |_, _this, (_ctx, _param1): (Table, Table)| {
                tracing::debug!("CreateMonster called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateMonsterByConfigIdByPos",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("CreateMonsterByConfigIdByPos called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateMonsterFaceAvatar",
            |_, _this, (_ctx, _param1): (Table, Table)| {
                tracing::debug!("CreateMonsterFaceAvatar called");
                Ok(-1)
            },
        );

        methods.add_method(
            "DelAllSubEntityByOriginOwnerConfigId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("DelAllSubEntityByOriginOwnerConfigId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "DelPlayerGroupVisionType",
            |_, _this, (_ctx, _param1, _param2): (Table, Table, Table)| {
                tracing::debug!("DelPlayerGroupVisionType called");
                Ok(-1)
            },
        );

        methods.add_method(
            "DelSceneTag",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("DelSceneTag called");
                Ok(-1)
            },
        );

        methods.add_method(
            "DelWorktopOption",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("DelWorktopOption called");
                Ok(-1)
            },
        );

        methods.add_method(
            "DelWorktopOptionByGroupId",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("DelWorktopOptionByGroupId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "EndMonsterTide",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("EndMonsterTide called");
                Ok(-1)
            },
        );

        methods.add_method(
            "EndTimeAxis",
            |_, _this, (_ctx, _param1): (Table, String)| {
                tracing::debug!("EndTimeAxis called");
                Ok(-1)
            },
        );

        methods.add_method(
            "EnterPersistentDungeon",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, Table)| {
                tracing::debug!("EnterPersistentDungeon called");
                Ok(-1)
            },
        );

        methods.add_method(
            "EnterWeatherArea",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("EnterWeatherArea called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ExecuteGadgetLua",
            |_,
             _this,
             (_ctx, _param1, _param2, _param3, _param4, _param5): (
                Table,
                u32,
                u32,
                u32,
                u32,
                u32,
            )| {
                tracing::debug!("ExecuteGadgetLua called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ExecuteGroupLua",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, String, Table)| {
                tracing::debug!("ExecuteGroupLua called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ExpeditionChallengeEnterRegion",
            |_, _this, (_ctx, _param1): (Table, bool)| {
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

        methods.add_method("FinishExpeditionChallenge", |_, _this, _ctx: Table| {
            tracing::debug!("FinishExpeditionChallenge called");
            Ok(-1)
        });

        methods.add_method(
            "FinishGroupLinkBundle",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("FinishGroupLinkBundle called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ForceRefreshAuthorityByConfigId",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("ForceRefreshAuthorityByConfigId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetActivityOpenAndCloseTimeByScheduleId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetActivityOpenAndCloseTimeByScheduleId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetAvatarEntityIdByUid",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetAvatarEntityIdByUid called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetBlossomRefreshTypeByGroupId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetBlossomRefreshTypeByGroupId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetBlossomScheduleStateByGroupId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetBlossomScheduleStateByGroupId called");
                Ok(-1)
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
            "GetConfigIdByEntityId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetConfigIdByEntityId called");
                Ok(-1)
            },
        );

        methods.add_method("GetCurTriggerCount", |_, _this, _ctx: Table| {
            tracing::debug!("GetCurTriggerCount called");
            Ok(-1)
        });

        methods.add_method(
            "GetCurrentLevelTagVec",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetCurrentLevelTagVec called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetDeathZoneStatus",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetDeathZoneStatus called");
                Ok(-1)
            },
        );

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
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetEntityIdByConfigId called");
                Ok(-1)
            },
        );

        methods.add_method("GetEntityType", |_, _this, entity_id: u32| {
            tracing::debug!("GetEntityType called");
            Ok(entity_id >> 22)
        });

        methods.add_method(
            "GetExhibitionAccumulableData",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("GetExhibitionAccumulableData called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetGadgetAbilityFloatValue",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, String)| {
                tracing::debug!("GetGadgetAbilityFloatValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetGadgetConfigId",
            |_, _this, (_ctx, _param1): (Table, Table)| {
                tracing::debug!("GetGadgetConfigId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetGadgetIdByEntityId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetGadgetIdByEntityId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetGadgetStateByConfigId",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("GetGadgetStateByConfigId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetGalleryProgressScore",
            |_, _this, (_ctx, _param1, _param2): (Table, String, u32)| {
                tracing::debug!("GetGalleryProgressScore called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetGalleryUidList",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetGalleryUidList called");
                Ok(-1)
            },
        );

        methods.add_method("GetGameHour", |_, _this, _ctx: Table| {
            tracing::debug!("GetGameHour called");
            Ok(-1)
        });

        methods.add_method(
            "GetGroupLogicStateValue",
            |_, _this, (_ctx, _param1): (Table, String)| {
                tracing::debug!("GetGroupLogicStateValue called");
                Ok(-1)
            },
        );

        methods.add_method("GetGroupMonsterCount", |_, _this, _ctx: Table| {
            tracing::debug!("GetGroupMonsterCount called");
            _this.script_lib.get_group_monster_count(_ctx);
            Ok(-1)
        });

        methods.add_method(
            "GetGroupMonsterCountByGroupId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetGroupMonsterCountByGroupId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetGroupSuite",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetGroupSuite called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetGroupTempValue",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, Table)| {
                tracing::debug!("GetGroupTempValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetGroupVariableValue",
            |_, _this, (_ctx, _param1): (Table, String)| {
                tracing::debug!("GetGroupVariableValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetGroupVariableValueByGroup",
            |_, _this, (_ctx, _param1, _param2): (Table, String, u32)| {
                tracing::debug!("GetGroupVariableValueByGroup called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetHostQuestState",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetHostQuestState called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetHuntingMonsterExtraSuiteIndexVec",
            |_, _this, _ctx: Table| {
                tracing::debug!("GetHuntingMonsterExtraSuiteIndexVec called");
                Ok(-1)
            },
        );

        methods.add_method("GetLanternRiteValue", |_, _this, _ctx: Table| {
            tracing::debug!("GetLanternRiteValue called");
            Ok(-1)
        });

        methods.add_method(
            "GetLevelTagNameById",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetLevelTagNameById called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetOfferingLevel",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetOfferingLevel called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetOpeningDungeonListByRosterId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetOpeningDungeonListByRosterId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetPlayerVehicleType",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetPlayerVehicleType called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetPosByEntityId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetPosByEntityId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetQuestState",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("GetQuestState called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetRegionConfigId",
            |_, _this, (_ctx, _param1): (Table, Table)| {
                tracing::debug!("GetRegionConfigId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetRegionEntityCount",
            |_, _this, (_ctx, _param1): (Table, Table)| {
                tracing::debug!("GetRegionEntityCount called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetRotationByEntityId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetRotationByEntityId called");
                Ok(-1)
            },
        );

        methods.add_method("GetSceneUidList", |_, _this, _ctx: Table| {
            tracing::debug!("GetSceneUidList called");
            Ok(-1)
        });

        methods.add_method("GetServerTime", |_, _this, _ctx: Table| {
            tracing::debug!("GetServerTime called");
            Ok(-1)
        });

        methods.add_method("GetServerTimeByWeek", |_, _this, _ctx: Table| {
            tracing::debug!("GetServerTimeByWeek called");
            Ok(-1)
        });

        methods.add_method(
            "GoToFlowSuite",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("GoToFlowSuite called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GoToGroupSuite",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("GoToGroupSuite called");
                Ok(-1)
            },
        );

        methods.add_method("InitTimeAxis", |_, _this, (_ctx, _param1): (Table, u32)| {
            tracing::debug!("InitTimeAxis called");
            Ok(-1)
        });

        methods.add_method(
            "IsChannellerSlabLoopDungeonConditionSelected",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("IsChannellerSlabLoopDungeonConditionSelected called");
                Ok(-1)
            },
        );

        methods.add_method(
            "IsEffigyChallengeConditionSelected",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("IsEffigyChallengeConditionSelected called");
                Ok(-1)
            },
        );

        methods.add_method(
            "IsInRegion",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("IsInRegion called");
                Ok(-1)
            },
        );

        methods.add_method(
            "IsPlayerAllAvatarDie",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("IsPlayerAllAvatarDie called");
                Ok(-1)
            },
        );

        methods.add_method(
            "IsWidgetEquipped",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("IsWidgetEquipped called");
                Ok(-1)
            },
        );

        methods.add_method(
            "KillEntityByConfigId",
            |_, _this, (_ctx, _param1): (Table, Table)| {
                tracing::debug!("KillEntityByConfigId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "KillExtraFlowSuite",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("KillExtraFlowSuite called");
                Ok(-1)
            },
        );

        methods.add_method(
            "KillExtraGroupSuite",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("KillExtraGroupSuite called");
                Ok(-1)
            },
        );

        methods.add_method(
            "KillGroupEntity",
            |_, _this, (_ctx, _param1): (Table, Table)| {
                tracing::debug!("KillGroupEntity called");
                Ok(-1)
            },
        );

        methods.add_method(
            "KillMonsterTide",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("KillMonsterTide called");
                Ok(-1)
            },
        );

        methods.add_method(
            "LockMonsterHp",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("LockMonsterHp called");
                Ok(-1)
            },
        );

        methods.add_method(
            "MarkPlayerAction",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("MarkPlayerAction called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ModifyClimatePolygon_paramTable",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, Table)| {
                tracing::debug!("ModifyClimatePolygon_paramTable called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ModifyFatherChallengeProperty",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("ModifyFatherChallengeProperty called");
                Ok(-1)
            },
        );

        methods.add_method(
            "MoveAvatarByPointArray",
            |_,
             _this,
             (_ctx, _param1, _param2, _param3, _param4, _param5): (
                Table,
                u32,
                u32,
                u32,
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
             (_ctx, _param1, _param2, _param3, _param4, _param5): (
                Table,
                u32,
                u32,
                u32,
                u32,
                Table,
            )| {
                tracing::debug!("MoveAvatarByPointArrayWithTemplate called");
                Ok(-1)
            },
        );

        methods.add_method(
            "MovePlayerToPos",
            |_, _this, (_ctx, _param1): (Table, Table)| {
                tracing::debug!("MovePlayerToPos called");
                Ok(-1)
            },
        );

        methods.add_method(
            "PauseTimeAxis",
            |_, _this, (_ctx, _param1): (Table, String)| {
                tracing::debug!("PauseTimeAxis called");
                Ok(-1)
            },
        );

        methods.add_method(
            "PlayCutScene",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("PlayCutScene called");
                Ok(-1)
            },
        );

        methods.add_method(
            "PrintContextLog",
            |_, _this, (_ctx, log): (Table, String)| {
                tracing::debug!("PrintContextLog called {}", log);
                Ok(0)
            },
        );

        methods.add_method(
            "PrintGroupWarning",
            |_, _this, (_ctx, _param1): (Table, String)| {
                tracing::debug!("PrintGroupWarning called");
                Ok(-1)
            },
        );

        methods.add_method("PrintLog", |_, _this, log: String| {
            tracing::debug!("PrintLog called {}", log);
            Ok(-1)
        });

        methods.add_method(
            "RefreshBlossomDropRewardByGroupId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("RefreshBlossomDropRewardByGroupId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "RefreshBlossomGroup",
            |_, _this, (_ctx, _param1): (Table, Table)| {
                tracing::debug!("RefreshBlossomGroup called");
                Ok(-1)
            },
        );

        methods.add_method(
            "RefreshGroup",
            |_, _this, (_ctx, _param1): (Table, Table)| {
                tracing::debug!("RefreshGroup called");
                Ok(-1)
            },
        );

        methods.add_method("RefreshHuntingClueGroup", |_, _this, _ctx: Table| {
            tracing::debug!("RefreshHuntingClueGroup called");
            Ok(-1)
        });

        methods.add_method(
            "RemoveEntityByConfigId",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("RemoveEntityByConfigId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "RemoveExtraFlowSuite",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("RemoveExtraFlowSuite called");
                Ok(-1)
            },
        );

        methods.add_method(
            "RemoveExtraGroupSuite",
            |_, _this, (_ctx, group_id, suite_id): (Table, u32, u32)| {
                tracing::debug!("RemoveExtraGroupSuite called");
                _this
                    .script_lib
                    .remove_extra_group_suite(_ctx, group_id, suite_id);
                Ok(0)
            },
        );

        methods.add_method(
            "ScenePlaySound",
            |_, _this, (_ctx, _param1): (Table, Table)| {
                tracing::debug!("ScenePlaySound called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SendServerMessageByLuaKey",
            |_, _this, (_ctx, _param1, _param2): (Table, String, Table)| {
                tracing::debug!("SendServerMessageByLuaKey called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetBlossomScheduleStateByGroupId",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("SetBlossomScheduleStateByGroupId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetChallengeEventMark",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("SetChallengeEventMark called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetEntityServerGlobalValueByConfigId",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, String, u32)| {
                tracing::debug!("SetEntityServerGlobalValueByConfigId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetEntityServerGlobalValueByEntityId",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, String, u32)| {
                tracing::debug!("SetEntityServerGlobalValueByEntityId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetEnvironmentEffectState",
            |_,_this, (_ctx, _param1, _param2, _param3, _param4): (Table, u32, String, u32, u32)| {
                tracing::debug!("SetEnvironmentEffectState called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetFlowSuite",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("SetFlowSuite called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetGadgetEnableInteract",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, bool)| {
                tracing::debug!("SetGadgetEnableInteract called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetGadgetHp",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("SetGadgetHp called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetGadgetStateByConfigId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("SetGadgetStateByConfigId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetGadgetTalkByConfigId",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("SetGadgetTalkByConfigId called");
                Ok(-1)
            },
        );

        methods.add_method("SetGroupDead", |_, _this, (_ctx, _param1): (Table, u32)| {
            tracing::debug!("SetGroupDead called");
            Ok(-1)
        });

        methods.add_method(
            "SetGroupGadgetStateByConfigId",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("SetGroupGadgetStateByConfigId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetGroupLogicStateValue",
            |_, _this, (_ctx, _param1, _param2): (Table, String, u32)| {
                tracing::debug!("SetGroupLogicStateValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetGroupReplaceable",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, bool)| {
                tracing::debug!("SetGroupReplaceable called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetGroupTempValue",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, Table)| {
                tracing::debug!("SetGroupTempValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetGroupVariableValue",
            |_, _this, (_ctx, _param1, _param2): (Table, String, u32)| {
                tracing::debug!("SetGroupVariableValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetGroupVariableValueByGroup",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, String, u32, u32)| {
                tracing::debug!("SetGroupVariableValueByGroup called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetHandballGalleryBallPosAndRot",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, Table, Table)| {
                tracing::debug!("SetHandballGalleryBallPosAndRot called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetIsAllowUseSkill",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("SetIsAllowUseSkill called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetMonsterAIByGroup",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("SetMonsterAIByGroup called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetMonsterBattleByGroup",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("SetMonsterBattleByGroup called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetMonsterHp",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("SetMonsterHp called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetPlatformPointArray",
            |_,_this, (_ctx, _param1, _param2, _param3, _param4): (Table, u32, u32, Vec<u32>, Table)| {
                tracing::debug!("SetPlatformPointArray called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetPlatformRouteId",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("SetPlatformRouteId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetPlayerEyePoint",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("SetPlayerEyePoint called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetPlayerEyePointStream",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, bool)| {
                tracing::debug!("SetPlayerEyePointStream called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetPlayerGroupVisionType",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, Table)| {
                tracing::debug!("SetPlayerGroupVisionType called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetPlayerInteractOption",
            |_, _this, (_ctx, _param1): (Table, String)| {
                tracing::debug!("SetPlayerInteractOption called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetTeamEntityGlobalFloatValue",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, String, u32)| {
                tracing::debug!("SetTeamEntityGlobalFloatValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetTeamServerGlobalValue",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, String, u32)| {
                tracing::debug!("SetTeamServerGlobalValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetWeatherAreaState",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("SetWeatherAreaState called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetWorktopOptions",
            |_, _this, (_ctx, _param1): (Table, Vec<u32>)| {
                tracing::debug!("SetWorktopOptions called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetWorktopOptionsByGroupId",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, Vec<u32>)| {
                tracing::debug!("SetWorktopOptionsByGroupId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ShowClientGuide",
            |_, _this, (_ctx, _param1): (Table, String)| {
                tracing::debug!("ShowClientGuide called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ShowClientTutorial",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, Table)| {
                tracing::debug!("ShowClientTutorial called");
                Ok(-1)
            },
        );

        methods.add_method("ShowReminder", |_, _this, (_ctx, _param1): (Table, u32)| {
            tracing::debug!("ShowReminder called");
            Ok(-1)
        });

        methods.add_method(
            "ShowReminderRadius",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("ShowReminderRadius called");
                Ok(-1)
            },
        );

        methods.add_method(
            "StartChallenge",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, Table)| {
                tracing::debug!("StartChallenge called");
                Ok(-1)
            },
        );

        methods.add_method(
            "StartFatherChallenge",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("StartFatherChallenge called");
                Ok(-1)
            },
        );

        methods.add_method("StartGallery", |_, _this, (_ctx, _param1): (Table, u32)| {
            tracing::debug!("StartGallery called");
            Ok(-1)
        });

        methods.add_method(
            "StartHomeGallery",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("StartHomeGallery called");
                Ok(-1)
            },
        );

        methods.add_method(
            "StartPlatform",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("StartPlatform called");
                Ok(-1)
            },
        );

        methods.add_method(
            "StartSealBattle",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, Table)| {
                tracing::debug!("StartSealBattle called");
                Ok(-1)
            },
        );

        methods.add_method(
            "StopChallenge",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("StopChallenge called");
                Ok(-1)
            },
        );

        methods.add_method(
            "StopGallery",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, bool)| {
                tracing::debug!("StopGallery called");
                Ok(-1)
            },
        );

        methods.add_method("StopPlatform", |_, _this, (_ctx, _param1): (Table, u32)| {
            tracing::debug!("StopPlatform called");
            Ok(-1)
        });

        methods.add_method(
            "TowerCountTimeStatus",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("TowerCountTimeStatus called");
                Ok(-1)
            },
        );

        methods.add_method(
            "TowerMirrorTeamSetUp",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("TowerMirrorTeamSetUp called");
                Ok(-1)
            },
        );

        methods.add_method(
            "TransPlayerToPos",
            |_, _this, (_ctx, _param1): (Table, Table)| {
                tracing::debug!("TransPlayerToPos called");
                Ok(-1)
            },
        );

        methods.add_method(
            "TryReallocateEntityAuthority",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("TryReallocateEntityAuthority called");
                Ok(-1)
            },
        );

        methods.add_method(
            "TryRecordActivityPushTips",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("TryRecordActivityPushTips called");
                Ok(-1)
            },
        );

        methods.add_method(
            "UnfreezeGroupLimit",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("UnfreezeGroupLimit called");
                Ok(-1)
            },
        );

        methods.add_method(
            "UnhideScenePoint",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("UnhideScenePoint called");
                Ok(-1)
            },
        );

        methods.add_method(
            "UnlockFloatSignal",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("UnlockFloatSignal called");
                Ok(-1)
            },
        );

        methods.add_method("UnlockForce", |_, _this, (_ctx, _param1): (Table, u32)| {
            tracing::debug!("UnlockForce called");
            Ok(-1)
        });

        methods.add_method(
            "UnlockMonsterHp",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("UnlockMonsterHp called");
                Ok(-1)
            },
        );

        methods.add_method(
            "UnlockScenePoint",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("UnlockScenePoint called");
                Ok(-1)
            },
        );

        methods.add_method(
            "UpdatePlayerGalleryScore",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, Table)| {
                tracing::debug!("UpdatePlayerGalleryScore called");
                Ok(-1)
            },
        );

        methods.add_method("sendCloseCommonTipsToClient", |_, _this, _ctx: Table| {
            tracing::debug!("sendCloseCommonTipsToClient called");
            Ok(-1)
        });

        methods.add_method(
            "sendShowCommonTipsToClient",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, String, String, u32)| {
                tracing::debug!("sendShowCommonTipsToClient called");
                Ok(-1)
            },
        );
    }
}
