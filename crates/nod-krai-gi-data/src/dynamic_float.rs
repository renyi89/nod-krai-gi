use serde::{Deserialize, Deserializer};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum DynamicFloat {
    Number(f64),
    String(String),
    Array(Vec<NumberOrString>),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum NumberOrString {
    Number(f64),
    String(String),
}

fn parse_any_to_float(value: &Value) -> f32 {
    match value {
        Value::Number(n) => n.as_f64().map(|v| v as f32).unwrap_or(0.0),
        Value::String(s) => {
            let (value, negative) = if s.starts_with("%") {
                (&s[1..], false)
            } else if s.starts_with("-%") {
                (&s[2..], true)
            } else {
                (s.as_str(), false)
            };
            let value = value.parse::<f32>().ok().unwrap();
            if negative {
                -value
            } else {
                value
            }
        }
        Value::Object(obj) => {
            if let Some(v) = obj.get("value") {
                parse_any_to_float(v)
            } else {
                0.0
            }
        }
        _ => 0.0,
    }
}

pub fn any_to_float<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    Ok(parse_any_to_float(&value))
}

pub fn any_to_float_hashmap<'de, D>(
    deserializer: D,
) -> Result<std::collections::HashMap<String, f32>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Object(map) => {
            let mut result = std::collections::HashMap::new();
            for (key, val) in map {
                result.insert(key.clone(), parse_any_to_float(&val));
            }
            Ok(result)
        }
        Value::Null => Ok(std::collections::HashMap::new()),
        _ => Ok(std::collections::HashMap::new()),
    }
}
