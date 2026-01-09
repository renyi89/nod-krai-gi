use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
pub struct TalentConfig {
    #[serde(flatten)]
    pub talents: HashMap<String, Vec<TalentAction>>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum TalentAction {
    #[serde(rename = "AddAbility")]
    AddAbility {
        #[serde(rename = "abilityName")]
        ability_name: String,
    },
    #[serde(rename = "UnlockTalentParam")]
    UnlockTalentParam {
        #[serde(rename = "abilityName")]
        ability_name: String,
        #[serde(rename = "talentParam")]
        talent_param: String,
    },
    #[serde(rename = "ModifyAbility")]
    ModifyAbility {
        #[serde(rename = "abilityName")]
        ability_name: String,
        #[serde(deserialize_with = "de_float_any")]
        #[serde(default, rename = "paramDelta")]
        param_delta: f64,
        #[serde(default, rename = "paramSpecial")]
        param_special: String,
    },
    #[serde(rename = "AddTalentExtraLevel")]
    AddTalentExtraLevel {
        #[serde(rename = "extraLevel")]
        extra_level: u32,
        #[serde(default, rename = "talentType")]
        talent_type: String,
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

fn de_float_any<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{Error, Unexpected};

    struct FloatVisitor;

    impl<'de> serde::de::Visitor<'de> for FloatVisitor {
        type Value = f64;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("a float, int, or percent string")
        }

        fn visit_f64<E>(self, v: f64) -> Result<f64, E> {
            Ok(v)
        }

        fn visit_i64<E>(self, v: i64) -> Result<f64, E> {
            Ok(v as f64)
        }

        fn visit_u64<E>(self, v: u64) -> Result<f64, E> {
            Ok(v as f64)
        }

        fn visit_str<E>(self, v: &str) -> Result<f64, E>
        where
            E: Error,
        {
            let s = v.trim();

            // 处理正负号
            let (sign, body) = if let Some(rest) = s.strip_prefix('-') {
                (-1.0, rest.trim())
            } else if let Some(rest) = s.strip_prefix('+') {
                (1.0, rest.trim())
            } else {
                (1.0, s)
            };

            // 处理百分比
            if let Some(stripped) = body.strip_prefix('%') {
                let num: f64 = stripped.trim().parse().map_err(|_| {
                    Error::invalid_value(Unexpected::Str(v), &"a percent string like %1 or -%1")
                })?;
                return Ok(sign * (num / 100.0));
            }

            // 普通数字
            let num: f64 = body.parse().map_err(|_| {
                Error::invalid_value(Unexpected::Str(v), &"a string containing a float")
            })?;

            Ok(sign * num)
        }
    }

    deserializer.deserialize_any(FloatVisitor)
}

