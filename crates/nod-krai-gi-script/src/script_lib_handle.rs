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
            "ActivateGroupLinkBundleByBundleId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("ActivateGroupLinkBundleByBundleId called");
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
            "AddChallengeDuration",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, bool)| {
                tracing::debug!("AddChallengeDuration called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddChessBuildingPoints",
            |_, _this, (_ctx, _param1, _param2, _param3, _param4): (Table, u32, u32, u32, u32)| {
                tracing::debug!("AddChessBuildingPoints called");
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
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("AddExhibitionAccumulableData called");
                Ok(-1)
            },
        );

        methods.add_method("AddExhibitionAccumulableDataAfterSuccess", |_, _this, (_ctx, _param1, _param2, _param3, _param4): (Table, u32, String, u32, Table)| {
            tracing::debug!("AddExhibitionAccumulableDataAfterSuccess called");
            Ok(-1)
        });

        methods.add_method(
            "AddExhibitionReplaceableData",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("AddExhibitionReplaceableData called");
                Ok(-1)
            },
        );

        methods.add_method("AddExhibitionReplaceableDataAfterSuccess", |_, _this, (_ctx, _param1, _param2, _param3, _param4): (Table, u32, String, u32, Table)| {
            tracing::debug!("AddExhibitionReplaceableDataAfterSuccess called");
            Ok(-1)
        });

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
            "AddFleurFairMultistagePlayBuffEnergy",
            |_, _this, (_ctx, _param1, _param2, _param3, _param4): (Table, u32, u32, u32, u32)| {
                tracing::debug!("AddFleurFairMultistagePlayBuffEnergy called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddGalleryProgressScore",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("AddGalleryProgressScore called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddIrodoriChessBuildingPoints",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("AddIrodoriChessBuildingPoints called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddIrodoriChessTowerServerGlobalValue",
            |_, _this, (_ctx, _param1, _param2, _param3, _param4): (Table, u32, u32, u32, u32)| {
                tracing::debug!("AddIrodoriChessTowerServerGlobalValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddMechanicusBuildingPoints",
            |_, _this, (_ctx, _param1, _param2, _param3, _param4): (Table, u32, u32, u32, u32)| {
                tracing::debug!("AddMechanicusBuildingPoints called");
                Ok(-1)
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
            "AddRegionRecycleProgress",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("AddRegionRecycleProgress called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddRegionSearchProgress",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("AddRegionSearchProgress called");
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
            "AddSceneMultiStagePlayUidValue",
            |_,
             _this,
             (_ctx, _param1, _param2, _param3, _param4, _param5): (
                Table,
                u32,
                u32,
                String,
                u32,
                u32,
            )| {
                tracing::debug!("AddSceneMultiStagePlayUidValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddScenePlayBattleProgress",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("AddScenePlayBattleProgress called");
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
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, Table, u32, u32)| {
                tracing::debug!("AddTeamEntityGlobalFloatValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AddTeamServerGlobalValue",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, String, u32)| {
                tracing::debug!("AddTeamServerGlobalValue called");
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
            "AssignPlayerUidOpNotify",
            |_, _this, (_ctx, _param1): (Table, Table)| {
                tracing::debug!("AssignPlayerUidOpNotify called");
                Ok(-1)
            },
        );

        methods.add_method(
            "AttachChildChallenge",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
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
            "AttachGalleryTeamAbilityGroup",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, Table, u32, u32)| {
                tracing::debug!("AttachGalleryTeamAbilityGroup called");
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
                u32,
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
            |_, _this, (_ctx, _param1): (Table, u32)| {
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
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("CancelGroupTimerEvent called");
                Ok(-1)
            },
        );

        methods.add_method("CauseDungeonFail", |_, _this, _ctx: Table| {
            tracing::debug!("CauseDungeonFail called");
            Ok(-1)
        });

        methods.add_method("CauseDungeonSuccess", |_, _this, _ctx: Table| {
            tracing::debug!("CauseDungeonSuccess called");
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
            "ChangeGroupTempValue",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("ChangeGroupTempValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ChangeGroupVariableValue",
            |_, _this, (_ctx, _param1): (Table, u32)| {
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
            "ChangeToTargetLevelTagWithParamTable",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, Table)| {
                tracing::debug!("ChangeToTargetLevelTagWithParamTable called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CharAmusementMultistagePlaySwitchTeam",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("CharAmusementMultistagePlaySwitchTeam called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CharAmusementUpdateScore",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("CharAmusementUpdateScore called");
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
            "ClearExhibitionReplaceableData",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("ClearExhibitionReplaceableData called");
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
            "ContinueAutoMonster",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("ContinueAutoMonster called");
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

        methods.add_method("CreateFatherChallenge", |_, _this, (_ctx, _param1, _param2, _param3, _param4): (Table, u32, u32, u32, Table)| {
            tracing::debug!("CreateFatherChallenge called");
            Ok(-1)
        });

        methods.add_method(
            "CreateFoundation",
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
                tracing::debug!("CreateFoundation called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateFoundations",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("CreateFoundations called");
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
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("CreateGadgetByConfigIdByPos called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateGadgetByParamTable",
            |_, _this, (_ctx, _param1): (Table, Table)| {
                tracing::debug!("CreateGadgetByParamTable called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateGadgetWave",
            |_,
             _this,
             (_ctx, _param1, _param2, _param3, _param4, _param5): (
                Table,
                u32,
                u32,
                u32,
                Table,
                u32,
            )| {
                tracing::debug!("CreateGadgetWave called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateGadgetWithGlobalValue",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, Table)| {
                tracing::debug!("CreateGadgetWithGlobalValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateGroupTimerEvent",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("CreateGroupTimerEvent called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateGroupTrigger",
            |_, _this, (_ctx, _param1): (Table, String)| {
                tracing::debug!("CreateGroupTrigger called");
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
            |_, _this, (_ctx, _param1, _param2): (Table, u32, Table)| {
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
            "CreateMonsterWithGlobalValue",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, Table)| {
                tracing::debug!("CreateMonsterWithGlobalValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateMonstersFromMonsterPool",
            |_, _this, (_ctx, _param1): (Table, String)| {
                tracing::debug!("CreateMonstersFromMonsterPool called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateScenePlayGeneralRewardGadget",
            |_, _this, (_ctx, _param1): (Table, Table)| {
                tracing::debug!("CreateScenePlayGeneralRewardGadget called");
                Ok(-1)
            },
        );

        methods.add_method(
            "CreateTreasureMapSpotRewardGadget",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("CreateTreasureMapSpotRewardGadget called");
                Ok(-1)
            },
        );

        methods.add_method("CreateVehicle", |_, _this, (_ctx, _param1, _param2, _param3, _param4): (Table, u32, u32, Table, Table)| {
            tracing::debug!("CreateVehicle called");
            Ok(-1)
        });

        methods.add_method(
            "CrystalLinkDungeonTeamSetUp",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, Table)| {
                tracing::debug!("CrystalLinkDungeonTeamSetUp called");
                Ok(-1)
            },
        );

        methods.add_method(
            "DeactivateGroupLinkBundle",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("DeactivateGroupLinkBundle called");
                Ok(-1)
            },
        );

        methods.add_method(
            "DeactivateGroupLinkBundleByBundleId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("DeactivateGroupLinkBundleByBundleId called");
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
            "DelGalleryAbilityGroup",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, Table, u32, u32)| {
                tracing::debug!("DelGalleryAbilityGroup called");
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
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("DelWorktopOptionByGroupId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "DestroyIrodoriChessTower",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
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
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("DigSetSearchingTarget called");
                Ok(-1)
            },
        );

        methods.add_method(
            "DoRoguelikeCardGachaByLua",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("DoRoguelikeCardGachaByLua called");
                Ok(-1)
            },
        );

        methods.add_method("EndAllTimeAxis", |_, _this, _ctx: Table| {
            tracing::debug!("EndAllTimeAxis called");
            Ok(-1)
        });

        methods.add_method(
            "EndFatherChallenge",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("EndFatherChallenge called");
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
            "EndPoolMonsterTide",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("EndPoolMonsterTide called");
                Ok(-1)
            },
        );

        methods.add_method(
            "EndSceneMultiStagePlay",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, bool)| {
                tracing::debug!("EndSceneMultiStagePlay called");
                Ok(-1)
            },
        );

        methods.add_method(
            "EndSceneMultiStagePlayStage",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, String)| {
                tracing::debug!("EndSceneMultiStagePlayStage called");
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
            "EnterCustomDungeonOfficialEdit",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("EnterCustomDungeonOfficialEdit called");
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
            "EnterRogueCell",
            |_, _this, (_ctx, _param1): (Table, u32)| {
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
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("EnterWeatherArea called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ExecuteActiveGroupLua",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, String, Table)| {
                tracing::debug!("ExecuteActiveGroupLua called");
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
            |_, _this, (_ctx, _param1): (Table, u32)| {
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

        methods.add_method(
            "FailScenePlayBattle",
            |_, _this, (_ctx, _param1): (Table, u32)| {
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
            |_, _this, (_ctx, _param1, _param2, _param3, _param4): (Table, u32, u32, u32, bool)| {
                tracing::debug!("FinishFleurFairGalleryStageByUid called");
                Ok(-1)
            },
        );

        methods.add_method(
            "FinishGroupLinkBundle",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("FinishGroupLinkBundle called");
                Ok(-1)
            },
        );

        methods.add_method(
            "FinishRogueDiaryDungeonSingleRoom",
            |_, _this, (_ctx, _param1): (Table, bool)| {
                tracing::debug!("FinishRogueDiaryDungeonSingleRoom called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ForbidPlayerRegionVision",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("ForbidPlayerRegionVision called");
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
            "ForceSetIrodoriFoundationTowers",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("ForceSetIrodoriFoundationTowers called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GadgetPlayUidOp",
            |_,
             _this,
             (_ctx, _param1, _param2, _param3, _param4, _param5, _param6): (
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
            "GetActivityOpenAndCloseTimeByScheduleId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetActivityOpenAndCloseTimeByScheduleId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetAranaraCollectableCountByTypeAndState",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("GetAranaraCollectableCountByTypeAndState called");
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
            "GetBonusTreasureMapSolution",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetBonusTreasureMapSolution called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetChainLevel",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetChainLevel called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetChallengeTransaction",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetChallengeTransaction called");
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
            "GetCharAmusementGalleryTarget",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, bool)| {
                tracing::debug!("GetCharAmusementGalleryTarget called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetCharAmusementMultistagePlayGalleryIdVec",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("GetCharAmusementMultistagePlayGalleryIdVec called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetChessMonsterPoolIdVecByRound",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetChessMonsterPoolIdVecByRound called");
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

        methods.add_method("GetContextGroupId", |_, _this, _ctx: Table| {
            tracing::debug!("GetContextGroupId called");
            Ok(-1)
        });

        methods.add_method(
            "GetCurFungusFighterPlotConfigIdList",
            |_, _this, _ctx: Table| {
                tracing::debug!("GetCurFungusFighterPlotConfigIdList called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetCurFungusFighterTrainingParams",
            |_, _this, _ctx: Table| {
                tracing::debug!("GetCurFungusFighterTrainingParams called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetCurFungusFighterTrainingValidBackupFungusIdList",
            |_, _this, _ctx: Table| {
                tracing::debug!("GetCurFungusFighterTrainingValidBackupFungusIdList called");
                Ok(-1)
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
                Ok(-1)
            },
        );

        methods.add_method(
            "GetCurrentCustomDungeonParamVec",
            |_, _this, _ctx: Table| {
                tracing::debug!("GetCurrentCustomDungeonParamVec called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetCurrentLevelTagVec",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetCurrentLevelTagVec called");
                Ok(-1)
            },
        );

        methods.add_method("GetCustomDungeonCoinNum", |_, _this, _ctx: Table| {
            tracing::debug!("GetCustomDungeonCoinNum called");
            Ok(-1)
        });

        methods.add_method("GetCustomDungeonOpenRoomVec", |_, _this, _ctx: Table| {
            tracing::debug!("GetCustomDungeonOpenRoomVec called");
            Ok(-1)
        });

        methods.add_method(
            "GetDeathZoneStatus",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetDeathZoneStatus called");
                Ok(-1)
            },
        );

        methods.add_method("GetDungeonTeamPlayerNum", |_, _this, _ctx: Table| {
            tracing::debug!("GetDungeonTeamPlayerNum called");
            Ok(-1)
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
            "GetExhibitionReplaceableData",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("GetExhibitionReplaceableData called");
                Ok(-1)
            },
        );

        methods.add_method("GetFleurFairDungeonSectionId", |_, _this, _ctx: Table| {
            tracing::debug!("GetFleurFairDungeonSectionId called");
            Ok(-1)
        });

        methods.add_method(
            "GetFleurFairMultistagePlayBuffEnergy",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("GetFleurFairMultistagePlayBuffEnergy called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetFleurFairMultistagePlayGalleryIdVec",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("GetFleurFairMultistagePlayGalleryIdVec called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetFleurFairMultistagePlayGalleryTempValue",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, String)| {
                tracing::debug!("GetFleurFairMultistagePlayGalleryTempValue called");
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
            "GetGadgetHpPercent",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("GetGadgetHpPercent called");
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
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetGalleryProgressScore called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetGalleryTransaction",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetGalleryTransaction called");
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

        methods.add_method("GetGameTimePassed", |_, _this, _ctx: Table| {
            tracing::debug!("GetGameTimePassed called");
            Ok(-1)
        });

        methods.add_method(
            "GetGivingItemList",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetGivingItemList called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetGroupAliveMonsterList",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetGroupAliveMonsterList called");
                Ok(-1)
            },
        );

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
            |_, _this, (_ctx, _param1): (Table, u32)| {
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
            "GetHideAndSeekHunter",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetHideAndSeekHunter called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetHideAndSeekMap",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetHideAndSeekMap called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetHideAndSeekPlayGalleryId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetHideAndSeekPlayGalleryId called");
                Ok(-1)
            },
        );

        methods.add_method("GetHideAndSeekPlayIndex", |_, _this, _ctx: Table| {
            tracing::debug!("GetHideAndSeekPlayIndex called");
            Ok(-1)
        });

        methods.add_method(
            "GetHideAndSeekPlayerSkillList",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("GetHideAndSeekPlayerSkillList called");
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

        methods.add_method(
            "GetIrodoriChessSelectedCards",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("GetIrodoriChessSelectedCards called");
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
            "GetLunaRiteSacrificeNum",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetLunaRiteSacrificeNum called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetMechanicusBuildingPoints",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("GetMechanicusBuildingPoints called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetMechanicusMonsterPoolVec",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetMechanicusMonsterPoolVec called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetMonsterAbilityFloatValue",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, String)| {
                tracing::debug!("GetMonsterAbilityFloatValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetMonsterAffixListByConfigId",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("GetMonsterAffixListByConfigId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetMonsterConfigId",
            |_, _this, (_ctx, _param1): (Table, Table)| {
                tracing::debug!("GetMonsterConfigId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetMonsterIdByEntityId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetMonsterIdByEntityId called");
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
            "GetPlatformArrayInfoByPointId",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("GetPlatformArrayInfoByPointId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetPlatformPointArray",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetPlatformPointArray called");
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

        methods.add_method("GetPotionDungeonAffixParams", |_, _this, _ctx: Table| {
            tracing::debug!("GetPotionDungeonAffixParams called");
            Ok(-1)
        });

        methods.add_method(
            "GetQuestState",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("GetQuestState called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetQuestStateByUid",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("GetQuestStateByUid called");
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
            "GetRegionalPlayVarValue",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("GetRegionalPlayVarValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetRogueCellState",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetRogueCellState called");
                Ok(-1)
            },
        );

        methods.add_method("GetRogueDiaryDungeonStage", |_, _this, _ctx: Table| {
            tracing::debug!("GetRogueDiaryDungeonStage called");
            Ok(-1)
        });

        methods.add_method("GetRogueDiaryRoundAndRoom", |_, _this, _ctx: Table| {
            tracing::debug!("GetRogueDiaryRoundAndRoom called");
            Ok(-1)
        });

        methods.add_method(
            "GetRotationByEntityId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetRotationByEntityId called");
                Ok(-1)
            },
        );

        methods.add_method("GetSceneMultiStagePlayUidValue", |_, _this, (_ctx, _param1, _param2, _param3, _param4): (Table, u32, u32, String, u32)| {
            tracing::debug!("GetSceneMultiStagePlayUidValue called");
            Ok(-1)
        });

        methods.add_method("GetSceneOwnerUid", |_, _this, _ctx: Table| {
            tracing::debug!("GetSceneOwnerUid called");
            Ok(-1)
        });

        methods.add_method(
            "GetScenePlayBattleHostUid",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetScenePlayBattleHostUid called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetScenePlayBattleType",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetScenePlayBattleType called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetScenePlayBattleUidValue",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("GetScenePlayBattleUidValue called");
                Ok(-1)
            },
        );

        methods.add_method("GetSceneTimeSeconds", |_, _this, _ctx: Table| {
            tracing::debug!("GetSceneTimeSeconds called");
            Ok(-1)
        });

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
            "GetSurroundUidList",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("GetSurroundUidList called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetTeamAbilityFloatValue",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("GetTeamAbilityFloatValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetTeamServerGlobalValue",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetTeamServerGlobalValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "GetTreasureSeelieDayByGroupId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("GetTreasureSeelieDayByGroupId called");
                Ok(-1)
            },
        );

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

        methods.add_method("InitGalleryProgressScore", |_, _this, (_ctx, _param1, _param2, _param3, _param4): (Table, String, u32, u32, u32)| {
            tracing::debug!("InitGalleryProgressScore called");
            Ok(-1)
        });

        methods.add_method(
            "InitGalleryProgressWithScore",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("InitGalleryProgressWithScore called");
                Ok(-1)
            },
        );

        methods.add_method("InitSceneMultistagePlay", |_, _this, (_ctx, _param1, _param2, _param3, _param4): (Table, u32, u32, Table, u32)| {
            tracing::debug!("InitSceneMultistagePlay called");
            Ok(-1)
        });

        methods.add_method("InitTimeAxis", |_, _this, (_ctx, _param1): (Table, u32)| {
            tracing::debug!("InitTimeAxis called");
            Ok(-1)
        });

        methods.add_method(
            "InstableSprayGetSGVByBuffId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("InstableSprayGetSGVByBuffId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "InstableSprayRandomBuffs",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("InstableSprayRandomBuffs called");
                Ok(-1)
            },
        );

        methods.add_method(
            "InvaildGravenPhotoBundleMark",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("InvaildGravenPhotoBundleMark called");
                Ok(-1)
            },
        );

        methods.add_method(
            "IsChallengeStartedByChallengeId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("IsChallengeStartedByChallengeId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "IsChallengeStartedByChallengeIndex",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("IsChallengeStartedByChallengeIndex called");
                Ok(-1)
            },
        );

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
            "IsFungusCaptured",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("IsFungusCaptured called");
                Ok(-1)
            },
        );

        methods.add_method(
            "IsGalleryStart",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("IsGalleryStart called");
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
            "IsLevelTagChangeInCD",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("IsLevelTagChangeInCD called");
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
            "IsPlayerTransmittable",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("IsPlayerTransmittable called");
                Ok(-1)
            },
        );

        methods.add_method("IsRogueBossCellPrevCellFinish", |_, _this, _ctx: Table| {
            tracing::debug!("IsRogueBossCellPrevCellFinish called");
            Ok(-1)
        });

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
            "MarkGroupLuaAction",
            |_, _this, (_ctx, _param1, _param2): (Table, String, u32)| {
                tracing::debug!("MarkGroupLuaAction called");
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
            "ModifyClimatePolygonParamTable",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, Table)| {
                tracing::debug!("ModifyClimatePolygonParamTable called");
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
            "PauseAutoMonsterTide",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("PauseAutoMonsterTide called");
                Ok(-1)
            },
        );

        methods.add_method(
            "PauseAutoPoolMonsterTide",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("PauseAutoPoolMonsterTide called");
                Ok(-1)
            },
        );

        methods.add_method(
            "PauseChallenge",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("PauseChallenge called");
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
            "PlayCutSceneWithParam",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, Table)| {
                tracing::debug!("PlayCutSceneWithParam called");
                Ok(-1)
            },
        );

        methods.add_method(
            "PrestartScenePlayBattle",
            |_, _this, (_ctx, _param1): (Table, Table)| {
                tracing::debug!("PrestartScenePlayBattle called");
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

        methods.add_method("PrintLog", |_, _this, _ctx: String| {
            tracing::debug!("PrintLog called");
            Ok(-1)
        });

        methods.add_method(
            "RecieveAllAranaraCollectionByType",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("RecieveAllAranaraCollectionByType called");
                Ok(-1)
            },
        );

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

        methods.add_method("RefreshGroup", |_, _this, (_ctx, _param1): (Table, u32)| {
            tracing::debug!("RefreshGroup called");
            Ok(-1)
        });

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
            "ResumeAutoPoolMonsterTide",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("ResumeAutoPoolMonsterTide called");
                Ok(-1)
            },
        );

        methods.add_method(
            "RevertPlayerRegionVision",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("RevertPlayerRegionVision called");
                Ok(-1)
            },
        );

        methods.add_method(
            "RevokePlayerShowTemplateReminder",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, Table)| {
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
                _param1,
                _param2,
                _param3,
                _param4,
                _param5,
                _param6,
                _param7,
                _param8,
                _param9,
            ): (Table, u32, u32, Table, u32, String, Table, Table, u32, u32)| {
                tracing::debug!("ScenePlayBattleUidOp called");
                Ok(-1)
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
            "SetChainLevel",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, bool)| {
                tracing::debug!("SetChainLevel called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetChallengeDuration",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("SetChallengeDuration called");
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
            "SetChessMystery",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("SetChessMystery called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetCurFungusFighterTrainingParams",
            |_, _this, (_ctx, _param1): (Table, Table)| {
                tracing::debug!("SetCurFungusFighterTrainingParams called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetDarkPressureLevel",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("SetDarkPressureLevel called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetEntityServerGlobalValueByConfigId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
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

        methods.add_method("SetEnvironmentEffectState", |_, _this, (_ctx, _param1, _param2, _param3, _param4): (Table, u32, String, u32, u32)| {
            tracing::debug!("SetEnvironmentEffectState called");
            Ok(-1)
        });

        methods.add_method(
            "SetFleurFairMultistagePlayBuffEnergy",
            |_, _this, (_ctx, _param1, _param2, _param3, _param4): (Table, u32, u32, u32, u32)| {
                tracing::debug!("SetFleurFairMultistagePlayBuffEnergy called");
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

        methods.add_method(
            "SetGalleryRevivePoint",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("SetGalleryRevivePoint called");
                Ok(-1)
            },
        );

        methods.add_method("SetGroupDead", |_, _this, (_ctx, _param1): (Table, u32)| {
            tracing::debug!("SetGroupDead called");
            Ok(-1)
        });

        methods.add_method(
            "SetGroupGadgetStateByConfigId",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
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
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("SetGroupTempValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetGroupVariableValue",
            |_, _this, (_ctx, _param1): (Table, u32)| {
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
            "SetLanternRiteValue",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("SetLanternRiteValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetLimitOptimization",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, bool)| {
                tracing::debug!("SetLimitOptimization called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetMechanicusChallengeState",
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
                tracing::debug!("SetMechanicusChallengeState called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetMechanicusMonsterPoolVec",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("SetMechanicusMonsterPoolVec called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetMistTrialServerGlobalValue",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("SetMistTrialServerGlobalValue called");
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
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
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
            "SetPlatformRouteIndexToNext",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("SetPlatformRouteIndexToNext called");
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
            |_, _this, (_ctx, _param1, _param2): (Table, Table, Table)| {
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
            "SetPlayerStartGallery",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("SetPlayerStartGallery called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetRogueCellState",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("SetRogueCellState called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetSceneMultiStagePlayUidValue",
            |_,
             _this,
             (_ctx, _param1, _param2, _param3, _param4, _param5): (
                Table,
                u32,
                u32,
                String,
                u32,
                u32,
            )| {
                tracing::debug!("SetSceneMultiStagePlayUidValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetSceneMultiStagePlayValue",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, String, u32)| {
                tracing::debug!("SetSceneMultiStagePlayValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetSceneMultiStagePlayValues",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("SetSceneMultiStagePlayValues called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetScenePlayBattlePlayTeamEntityGadgetId",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("SetScenePlayBattlePlayTeamEntityGadgetId called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetScenePlayBattleUidValue",
            |_, _this, (_ctx, _param1, _param2, _param3, _param4): (Table, u32, u32, u32, u32)| {
                tracing::debug!("SetScenePlayBattleUidValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetTeamEntityGlobalFloatValue",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, Table, String, u32)| {
                tracing::debug!("SetTeamEntityGlobalFloatValue called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetTeamServerGlobalValue",
            |_, _this, (_ctx, _param1): (Table, u32)| {
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
            |_, _this, (_ctx, _param1): (Table, Table)| {
                tracing::debug!("SetWorktopOptions called");
                Ok(-1)
            },
        );

        methods.add_method(
            "SetWorktopOptionsByGroupId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
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

        methods.add_method(
            "ShowCommonPlayerTips",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, Table)| {
                tracing::debug!("ShowCommonPlayerTips called");
                Ok(-1)
            },
        );

        methods.add_method("ShowReminder", |_, _this, (_ctx, _param1): (Table, u32)| {
            tracing::debug!("ShowReminder called");
            Ok(-1)
        });

        methods.add_method(
            "ShowReminderByUid",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("ShowReminderByUid called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ShowReminderRadius",
            |_, _this, (_ctx, _param1, _param2, _param3): (Table, u32, u32, u32)| {
                tracing::debug!("ShowReminderRadius called");
                Ok(-1)
            },
        );

        methods.add_method(
            "ShowTemplateReminder",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, Table)| {
                tracing::debug!("ShowTemplateReminder called");
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

        methods.add_method("StartSceneMultiStagePlayStage", |_, _this, (_ctx, _param1, _param2, _param3, _param4): (Table, u32, u32, u32, String)| {
            tracing::debug!("StartSceneMultiStagePlayStage called");
            Ok(-1)
        });

        methods.add_method(
            "StartSealBattle",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, Table)| {
                tracing::debug!("StartSealBattle called");
                Ok(-1)
            },
        );

        methods.add_method(
            "StopChallenge",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("StopChallenge called");
                Ok(-1)
            },
        );

        methods.add_method("StopFishing", |_, _this, (_ctx, _param1): (Table, u32)| {
            tracing::debug!("StopFishing called");
            Ok(-1)
        });

        methods.add_method("StopGallery", |_, _this, (_ctx, _param1): (Table, u32)| {
            tracing::debug!("StopGallery called");
            Ok(-1)
        });

        methods.add_method(
            "StopGalleryByReason",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("StopGalleryByReason called");
                Ok(-1)
            },
        );

        methods.add_method("StopPlatform", |_, _this, (_ctx, _param1): (Table, u32)| {
            tracing::debug!("StopPlatform called");
            Ok(-1)
        });

        methods.add_method("StopReminder", |_, _this, (_ctx, _param1): (Table, u32)| {
            tracing::debug!("StopReminder called");
            Ok(-1)
        });

        methods.add_method(
            "SwitchSceneEnvAnimal",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("SwitchSceneEnvAnimal called");
                Ok(-1)
            },
        );

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
            "TreasureSeelieCollectOrbsNotify",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, u32)| {
                tracing::debug!("TreasureSeelieCollectOrbsNotify called");
                Ok(-1)
            },
        );

        methods.add_method(
            "TryFinishLuminanceStoneChallengeStage",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("TryFinishLuminanceStoneChallengeStage called");
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
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("UpdatePlayerGalleryScore called");
                Ok(-1)
            },
        );

        methods.add_method(
            "UpdateStakeHomePlayRecord",
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("UpdateStakeHomePlayRecord called");
                Ok(-1)
            },
        );

        methods.add_method(
            "VintageFinishGroupByPresentId",
            |_, _this, (_ctx, _param1): (Table, u32)| {
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
            |_, _this, (_ctx, _param1): (Table, u32)| {
                tracing::debug!("WinterCampSnowDriftInteract called");
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

        methods.add_method(
            "updateBundleMarkShowStateByGroupId",
            |_, _this, (_ctx, _param1, _param2): (Table, u32, bool)| {
                tracing::debug!("updateBundleMarkShowStateByGroupId called");
                Ok(-1)
            },
        );
    }
}
