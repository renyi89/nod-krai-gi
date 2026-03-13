use bevy_ecs::entity::Entity;
use bevy_ecs::message::Message;
use nod_krai_gi_data::ability::{AbilityMixinData, AbilityModifierAction};
use nod_krai_gi_proto::normal::AbilityInvokeEntry;

//ability
#[derive(Message)]
pub struct AddNewAbilityEvent(pub AbilityInvokeEntry, pub String);

#[derive(Message)]
pub struct ModifierChangeEvent(pub AbilityInvokeEntry, pub String);

#[derive(Message)]
pub struct OverrideParamEvent(pub AbilityInvokeEntry, pub String);

#[derive(Message)]
pub struct ReinitOverrideMapEvent(pub AbilityInvokeEntry, pub String);

#[derive(Message)]
pub struct GlobalFloatValueEvent(pub AbilityInvokeEntry, pub String);

#[derive(Message)]
pub struct ClearGlobalFloatValueEvent(pub AbilityInvokeEntry, pub String);

#[derive(Message)]
pub struct ServerInvokeEvent(pub AbilityInvokeEntry);

#[derive(Message)]
pub struct ExecuteActionEvent(
    pub u32,
    pub Entity,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Option<Entity>,
);

#[derive(Message)]
pub struct ExecuteMixinEvent(
    pub u32,
    pub Entity,
    pub AbilityMixinData,
    pub Vec<u8>,
    pub Option<Entity>,
);

#[derive(Message)]
pub struct AbilityActionHealHPEvent(
    pub u32,
    pub Entity,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Vec<Entity>,
);

#[derive(Message)]
pub struct AbilityActionLoseHPEvent(
    pub u32,
    pub Entity,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Vec<Entity>,
);

#[derive(Message)]
pub struct AbilityActionSetGlobalValueToOverrideMapEvent(
    pub u32,
    pub Entity,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Entity,
);

#[derive(Message)]
pub struct AbilityActionGetHPPaidDebtsEvent(
    pub u32,
    pub Entity,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Vec<Entity>,
);

#[derive(Message)]
pub struct AbilityActionSetOverrideMapValueEvent(
    pub u32,
    pub Entity,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Entity,
);

#[derive(Message)]
pub struct AbilityActionSetRandomOverrideMapValueEvent(
    pub u32,
    pub Entity,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Entity,
);

#[derive(Message)]
pub struct AbilityActionAddHPDebtsEvent(
    pub u32,
    pub Entity,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Vec<Entity>,
);

#[derive(Message)]
pub struct AbilityActionReduceHPDebtsEvent(
    pub u32,
    pub Entity,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Vec<Entity>,
);

#[derive(Message)]
pub struct AbilityActionModifyAvatarSkillCDEvent(
    pub u32,
    pub Entity,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Vec<Entity>,
);

#[derive(Message)]
pub struct AbilityActionSetGlobalValueEvent(
    pub u32,
    pub Entity,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Entity,
);

#[derive(Message)]
pub struct AbilityActionAddGlobalValueEvent(
    pub u32,
    pub Entity,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Entity,
);

#[derive(Message)]
pub struct AbilityActionApplyModifierEvent(
    pub u32,
    pub Entity,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Entity,
);

#[derive(Message)]
pub struct AbilityActionRemoveModifierEvent(
    pub u32,
    pub Entity,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Entity,
);

#[derive(Message)]
pub struct AbilityActionCopyGlobalValueEvent(
    pub u32,
    pub Entity,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Entity,
);

#[derive(Message)]
pub struct AbilityActionClearGlobalValueEvent(
    pub u32,
    pub Entity,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Entity,
);

#[derive(Message)]
pub struct AbilityActionAttachModifierEvent(
    pub u32,
    pub Entity,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Entity,
);

#[derive(Message)]
pub struct AbilityActionRemoveUniqueModifierEvent(
    pub u32,
    pub Entity,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Entity,
);

#[derive(Message)]
pub struct AbilityActionTriggerAbilityEvent(
    pub u32,
    pub Entity,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Entity,
);

#[derive(Message)]
pub struct AbilityActionKillSelfEvent(
    pub u32,
    pub Entity,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Vec<Entity>,
);

#[derive(Message)]
pub struct AbilityActionAvatarSkillStartEvent(
    pub u32,
    pub Entity,
    pub AbilityModifierAction,
    pub Vec<u8>,
    pub Vec<Entity>,
);

#[derive(Message)]
pub struct AttackLandedEvent(pub Entity, pub Entity); // attacker, target