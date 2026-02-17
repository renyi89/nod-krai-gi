pub mod ability;
pub mod combat;
pub mod command;
pub mod inventory;
pub mod lua;
pub mod luashell;
pub mod quest;
pub mod scene;
pub mod social;
pub mod time;

use crate::ability::*;
use crate::combat::*;
use crate::command::*;
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
            //command
            .add_message::<DebugCommandEvent>()
            .add_message::<ConsoleChatReqEvent>()
            .add_message::<ConsoleChatNotifyEvent>()
            .add_message::<CommandQuestEvent>()
            .add_message::<CommandItemEvent>()
            //
            .add_message::<StoreItemChangeEvent>()
            //lua
            .add_message::<LuaTriggerEvent>()
            .add_message::<SpawnGroupEntityEvent>()
            .add_message::<DespawnGroupEntityEvent>()
            //quest
            .add_message::<QuestBeginEvent>()
            .add_message::<QuestFinishEvent>()
            .add_message::<QuestListUpdateEvent>()
            //scene
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
            .add_message::<AbilityActionReduceHPDebtsEvent>();
    }
}
