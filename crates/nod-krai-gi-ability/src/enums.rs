use std::convert::TryFrom;

#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigAbilitySubContainerType {
    Action = 1,
    Mixin = 2,
    ModifierAction = 3,
    ModifierMixin = 4,
}

#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityConfigIdxEnum {
    OnAdded = 0,
    OnRemoved = 1,
    OnAbilityStart = 2,
    OnKill = 3,
    OnFieldEnter = 4,
    OnFieldExit = 5,
    OnAttach = 6,
    OnDetach = 7,
    OnAvatarIn = 8,
    OnAvatarOut = 9,
    OnTriggerAvatarRay = 10,
    OnVehicleIn = 11,
    OnVehicleOut = 12,
    Unknown(i32),
}

impl TryFrom<i32> for AbilityConfigIdxEnum {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AbilityConfigIdxEnum::OnAdded),
            1 => Ok(AbilityConfigIdxEnum::OnRemoved),
            2 => Ok(AbilityConfigIdxEnum::OnAbilityStart),
            3 => Ok(AbilityConfigIdxEnum::OnKill),
            4 => Ok(AbilityConfigIdxEnum::OnFieldEnter),
            5 => Ok(AbilityConfigIdxEnum::OnFieldExit),
            6 => Ok(AbilityConfigIdxEnum::OnAttach),
            7 => Ok(AbilityConfigIdxEnum::OnDetach),
            8 => Ok(AbilityConfigIdxEnum::OnAvatarIn),
            9 => Ok(AbilityConfigIdxEnum::OnAvatarOut),
            10 => Ok(AbilityConfigIdxEnum::OnTriggerAvatarRay),
            11 => Ok(AbilityConfigIdxEnum::OnVehicleIn),
            12 => Ok(AbilityConfigIdxEnum::OnVehicleOut),
            _ => Ok(AbilityConfigIdxEnum::Unknown(value)),
        }
    }
}

#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityModifierConfigIdxEnum {
    OnAdded = 0,
    OnRemoved = 1,
    OnBeingHit = 2,
    OnAttackLanded = 3,
    OnHittingOther = 4,
    OnThinkInterval = 5,
    OnKill = 6,
    OnCrash = 7,
    OnAvatarIn = 8,
    OnAvatarOut = 9,
    OnReconnect = 10,
    OnChangeAuthority = 11,
    OnVehicleIn = 12,
    OnVehicleOut = 13,
    OnZoneEnter = 14,
    OnZoneExit = 15,
    OnHeal = 16,
    OnBeingHealed = 17,
    Unknown(i32),
}

impl TryFrom<i32> for AbilityModifierConfigIdxEnum {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AbilityModifierConfigIdxEnum::OnAdded),
            1 => Ok(AbilityModifierConfigIdxEnum::OnRemoved),
            2 => Ok(AbilityModifierConfigIdxEnum::OnBeingHit),
            3 => Ok(AbilityModifierConfigIdxEnum::OnAttackLanded),
            4 => Ok(AbilityModifierConfigIdxEnum::OnHittingOther),
            5 => Ok(AbilityModifierConfigIdxEnum::OnThinkInterval),
            6 => Ok(AbilityModifierConfigIdxEnum::OnKill),
            7 => Ok(AbilityModifierConfigIdxEnum::OnCrash),
            8 => Ok(AbilityModifierConfigIdxEnum::OnAvatarIn),
            9 => Ok(AbilityModifierConfigIdxEnum::OnAvatarOut),
            10 => Ok(AbilityModifierConfigIdxEnum::OnReconnect),
            11 => Ok(AbilityModifierConfigIdxEnum::OnChangeAuthority),
            12 => Ok(AbilityModifierConfigIdxEnum::OnVehicleIn),
            13 => Ok(AbilityModifierConfigIdxEnum::OnVehicleOut),
            14 => Ok(AbilityModifierConfigIdxEnum::OnZoneEnter),
            15 => Ok(AbilityModifierConfigIdxEnum::OnZoneExit),
            16 => Ok(AbilityModifierConfigIdxEnum::OnHeal),
            17 => Ok(AbilityModifierConfigIdxEnum::OnBeingHealed),
            _ => Ok(AbilityModifierConfigIdxEnum::Unknown(value)),
        }
    }
}
