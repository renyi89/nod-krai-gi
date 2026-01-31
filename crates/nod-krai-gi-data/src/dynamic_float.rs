use serde::{Deserialize, Deserializer};
use serde_json::Value;

#[derive(Debug, Clone)]
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

impl<'de> Deserialize<'de> for DynamicFloat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // 只消费一次
        let v = Value::deserialize(deserializer)?;

        // number
        if let Some(n) = v.as_f64() {
            return Ok(DynamicFloat::Number(n));
        }

        // string
        if let Some(s) = v.as_str() {
            return Ok(DynamicFloat::String(s.to_string()));
        }

        // array
        if let Some(arr) = v.as_array() {
            let parsed: Result<Vec<NumberOrString>, _> = arr
                .iter()
                .map(|x| serde_json::from_value(x.clone()))
                .collect();

            if let Ok(a) = parsed {
                return Ok(DynamicFloat::Array(a));
            }
        }

        // 特殊结构：__exp_DynamicFloat
        if let Some(obj) = v.as_object() {
            if let Some(inner) = obj.get("__exp_DynamicFloat") {
                if let Some(fixed) = inner.get("__exp_DynamicKey").and_then(|x| x.as_str()) {
                    return Ok(DynamicFloat::String(fixed.to_string()));
                } else if let Some(fixed) = inner.get("__exp_FixedValue").and_then(|x| x.as_f64()) {
                    return Ok(DynamicFloat::Number(fixed));
                }
                if let Some(formula) = inner.get("__exp_Formula").and_then(|x| x.as_array()) {
                    let mut items = Vec::new();
                    for elem in formula {
                        if let Some(fixed) = elem.get("__exp_DynamicKey").and_then(|x| x.as_str()) {
                            items.push(NumberOrString::String(fixed.to_string()));
                        } else if let Some(fixed) =
                            elem.get("__exp_FixedValue").and_then(|x| x.as_f64())
                        {
                            items.push(NumberOrString::Number(fixed));
                        } else if let Some(op) =
                            elem.get("__exp_Operation").and_then(|x| x.as_str())
                        {
                            items.push(NumberOrString::String(op.to_string()));
                        }
                    }
                    return Ok(DynamicFloat::Array(items));
                }
            } else {
                return Ok(DynamicFloat::Number(0.0));
            }
        }

        Err(serde::de::Error::custom("invalid DynamicFloat"))
    }
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
