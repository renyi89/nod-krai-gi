use crate::dynamic_float::any_to_float;
use std::collections::HashMap;
use common::string_util::InternString;

#[derive(Debug, serde::Deserialize)]
pub struct TalentConfig {
    #[serde(flatten)]
    pub talents: HashMap<InternString, Vec<TalentAction>>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum TalentAction {
    #[serde(rename = "AddAbility")]
    AddAbility {
        #[serde(rename = "abilityName")]
        ability_name: InternString,
    },
    #[serde(rename = "UnlockTalentParam")]
    UnlockTalentParam {
        #[serde(rename = "abilityName")]
        ability_name: InternString,
        #[serde(rename = "talentParam")]
        talent_param: InternString,
    },
    #[serde(rename = "ModifyAbility")]
    ModifyAbility {
        #[serde(rename = "abilityName")]
        ability_name: InternString,
        #[serde(deserialize_with = "any_to_float")]
        #[serde(default, rename = "paramDelta")]
        param_delta: f32,
        #[serde(default, rename = "paramSpecial")]
        param_special: InternString,
    },
    #[serde(rename = "AddTalentExtraLevel")]
    AddTalentExtraLevel {
        #[serde(rename = "extraLevel")]
        extra_level: u32,
        #[serde(default, rename = "talentType")]
        talent_type: InternString,
        #[serde(default, rename = "talentIndex")]
        talent_index: u32,
    },
    #[serde(rename = "ModifySkillPoint")]
    ModifySkillPoint {
        #[serde(rename = "pointDelta")]
        point_delta: u32,
        #[serde(rename = "skillID")]
        skill_id: u32,
    },
    #[serde(rename = "ModifySkillCD")]
    ModifySkillCD {
        #[serde(rename = "skillID")]
        skill_id: u32,
        #[serde(default, rename = "cdRatio")]
        cd_ratio: f32,
        #[serde(default, rename = "cdDelta")]
        cd_delta: f32,
    },
    #[serde(other)]
    Unknown,
}
