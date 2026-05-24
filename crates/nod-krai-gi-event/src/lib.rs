pub mod ability;
pub mod avatar;
pub mod combat;
pub mod command;
pub mod entity;
pub mod inventory;
pub mod lua;
pub mod luashell;
pub mod quest;
pub mod scene;
pub mod social;
pub mod time;

use crate::ability::*;
use crate::avatar::*;
use crate::combat::*;
use crate::command::*;
use crate::entity::*;
use crate::inventory::*;
use crate::lua::*;
use crate::luashell::*;
use crate::quest::*;
use crate::scene::*;
use bevy_app::{App, Plugin};

pub struct EventRegistryPlugin;

impl Plugin for EventRegistryPlugin {
    fn build(&self, app: &mut App) {
        app
            //avatar
            .add_message::<AvatarEquipChangeEvent>()
            .add_message::<AvatarAppearanceChangeEvent>()
            //command
            .add_message::<DebugCommandEvent>()
            .add_message::<ConsoleChatReqEvent>()
            .add_message::<ConsoleChatNotifyEvent>()
            .add_message::<GmCommandEvent>()
            //inventory
            .add_message::<StoreItemChangeEvent>()
            .add_message::<ItemAddEvent>()
            .add_message::<ItemDropEvent>()
            //entity
            .add_message::<EntityPropertyUpdateEvent>()
            .add_message::<EntityPropertySeparateUpdateEvent>()
            .add_message::<EntityDisappearEvent>()
            .add_message::<EntityFightPropChangeReasonNotifyEvent>()
            .add_message::<GadgetInteractEvent>()
            .add_message::<GadgetStateChangeEvent>()
            //lua
            .add_message::<LuaTriggerEvent>()
            .add_message::<ScriptCommandEvent>()
            .add_message::<SpawnGroupEntityEvent>()
            .add_message::<DespawnGroupEntityEvent>()
            .add_message::<SpawnSuiteEntitiesEvent>()
            .add_message::<DespawnSuiteEntitiesEvent>()
            .add_message::<RefreshGroupEntityEvent>()
            .add_message::<OnClientExecuteReqEvent>()
            .add_message::<OnBeHurtEvent>()
            .add_message::<ChallengeStartEvent>()
            .add_message::<ChallengeFinishEvent>()
            .add_message::<ChallengeProgressEvent>()
            .add_message::<MonsterKillEvent>()
            //quest
            .add_message::<QuestAcceptCondEvent>()
            .add_message::<QuestAcceptEvent>()
            .add_message::<QuestFinishEvent>()
            .add_message::<QuestFailEvent>()
            .add_message::<QuestContentProgressEvent>()
            .add_message::<QuestExecEvent>()
            //scene
            .insert_resource(WorldOwnerUID(0))
            .insert_resource(WorldVersionConfig {
                protocol_version: "unknown version".to_string(),
                ty_value: 24,
            })
            .add_message::<BeginEnterSceneEvent>()
            .add_message::<EnterSceneReadyEvent>()
            .add_message::<SceneInitFinishEvent>()
            .add_message::<EnterSceneDoneEvent>()
            .add_message::<PostEnterSceneEvent>()
            .add_message::<PlayerJoinTeamEvent>()
            .add_message::<SceneTeamUpdateEvent>()
            .add_message::<PlayerAvatarTeamChanged>()
            .add_message::<ScenePlayerJumpEvent>()
            .add_message::<ScenePlayerJumpByPointEvent>()
            .add_message::<ScenePlayerEnterDungeonEvent>()
            //luashell
            .add_message::<LuaShellEvent>()
            //combat
            .add_message::<EntityMoveEvent>()
            .add_message::<EntityBeingHitEvent>()
            .add_message::<PlayerMoveEvent>()
            //ability
            .add_message::<AddNewAbilityEvent>()
            .add_message::<ModifierChangeEvent>()
            .add_message::<OverrideParamEvent>()
            .add_message::<ReinitOverrideMapEvent>()
            .add_message::<GlobalFloatValueEvent>()
            .add_message::<ClearGlobalFloatValueEvent>()
            .add_message::<ServerInvokeEvent>()
            .add_message::<ExecuteActionEvent>()
            .add_message::<ExecuteMixinEvent>()
            .add_message::<AbilityActionHealHPEvent>()
            .add_message::<AbilityActionLoseHPEvent>()
            .add_message::<AbilityActionSetGlobalValueToOverrideMapEvent>()
            .add_message::<AbilityActionGetHPPaidDebtsEvent>()
            .add_message::<AbilityActionSetOverrideMapValueEvent>()
            .add_message::<AbilityActionSetRandomOverrideMapValueEvent>()
            .add_message::<AbilityActionAddHPDebtsEvent>()
            .add_message::<AbilityActionReduceHPDebtsEvent>()
            .add_message::<AbilityActionModifyAvatarSkillCDEvent>()
            .add_message::<AbilityActionAvatarSkillStartEvent>()
            .add_message::<AbilityActionSetGlobalValueEvent>()
            .add_message::<AbilityActionAddGlobalValueEvent>()
            .add_message::<AbilityActionApplyModifierEvent>()
            .add_message::<AbilityActionRemoveModifierEvent>()
            .add_message::<AbilityActionCopyGlobalValueEvent>()
            .add_message::<AbilityActionClearGlobalValueEvent>()
            .add_message::<AbilityActionAttachModifierEvent>()
            .add_message::<AbilityActionRemoveUniqueModifierEvent>()
            .add_message::<AbilityActionTriggerAbilityEvent>()
            .add_message::<AbilityActionKillSelfEvent>();
    }
}
