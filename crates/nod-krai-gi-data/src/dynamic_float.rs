use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StackOp {
    Constant(f32),
    Key { value: String, negative: bool },
    Add,
    Sub,
    Mul,
    Div,
    NextBoolean(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicFloat {
    ops: Vec<StackOp>,
    dynamic: bool,
    constant: f32,
}

impl DynamicFloat {
    pub fn new(constant: f32) -> Self {
        Self {
            ops: vec![],
            dynamic: false,
            constant,
        }
    }

    pub fn from_ops(ops: Vec<StackOp>) -> Self {
        Self {
            ops,
            dynamic: true,
            constant: 0.0,
        }
    }

    pub fn get(&self) -> f32 {
        if !self.dynamic {
            return self.constant;
        }
        self.evaluate(&std::collections::HashMap::new(), 0.0)
    }

    pub fn get_with_default(&self, default_value: f32) -> f32 {
        if !self.dynamic {
            return self.constant;
        }
        self.evaluate(&std::collections::HashMap::new(), default_value)
    }

    pub fn evaluate(
        &self,
        props: &std::collections::HashMap<String, f32>,
        default_value: f32,
    ) -> f32 {
        if !self.dynamic {
            return self.constant;
        }

        let mut stack: Vec<f32> = vec![];
        for op in &self.ops {
            match op {
                StackOp::Constant(f) => stack.push(*f),
                StackOp::Key { value, negative } => {
                    let val = props.get(value).copied().unwrap_or(0.0);
                    stack.push(if *negative { -val } else { val });
                }
                StackOp::Add => {
                    let b = stack.pop().unwrap_or(0.0);
                    let a = stack.pop().unwrap_or(0.0);
                    stack.push(a + b);
                }
                StackOp::Sub => {
                    let b = stack.pop().unwrap_or(0.0);
                    let a = stack.pop().unwrap_or(0.0);
                    stack.push(a - b);
                }
                StackOp::Mul => {
                    let b = stack.pop().unwrap_or(0.0);
                    let a = stack.pop().unwrap_or(0.0);
                    stack.push(a * b);
                }
                StackOp::Div => {
                    let b = stack.pop().unwrap_or(1.0);
                    let a = stack.pop().unwrap_or(0.0);
                    stack.push(a / b);
                }
                StackOp::NextBoolean(b) => {
                    let val = if *b { 1.0 } else { 0.0 };
                    stack.push(val);
                }
            }
        }

        stack.pop().unwrap_or(default_value)
    }

    pub fn is_dynamic(&self) -> bool {
        self.dynamic
    }

    pub fn get_constant(&self) -> f32 {
        self.constant
    }
}

fn parse_stack_op(value: &Value) -> Option<StackOp> {
    match value {
        Value::Number(n) => Some(StackOp::Constant(n.as_f64().map(|v| v as f32)?)),
        Value::String(s) => {
            let s_upper = s.to_uppercase();
            match s_upper.as_str() {
                "ADD" => Some(StackOp::Add),
                "SUB" => Some(StackOp::Sub),
                "MUL" => Some(StackOp::Mul),
                "DIV" => Some(StackOp::Div),
                _ => {
                    let (value, negative) = if s.starts_with("%") {
                        (&s[1..], false)
                    } else if s.starts_with("-%") {
                        (&s[2..], true)
                    } else {
                        (s.as_str(), false)
                    };
                    Some(StackOp::Key {
                        value: value.to_string(),
                        negative,
                    })
                }
            }
        }
        Value::Bool(b) => Some(StackOp::NextBoolean(*b)),
        Value::Object(obj) => {
            if let Some(v) = obj.get("value") {
                parse_stack_op(v)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn parse_dynamic_float(value: &Value) -> DynamicFloat {
    match value {
        Value::Number(n) => DynamicFloat::new(n.as_f64().map(|v| v as f32).unwrap_or(0.0)),
        Value::String(s) => {
            if let Ok(b) = s.parse::<bool>() {
                DynamicFloat::from_ops(vec![StackOp::NextBoolean(b)])
            } else {
                let op = parse_stack_op(value).unwrap();
                DynamicFloat::from_ops(vec![op])
            }
        }
        Value::Bool(b) => DynamicFloat::from_ops(vec![StackOp::NextBoolean(*b)]),
        Value::Array(arr) => {
            let ops: Vec<StackOp> = arr.iter().filter_map(parse_stack_op).collect();
            if ops.is_empty() {
                DynamicFloat::new(0.0)
            } else {
                DynamicFloat::from_ops(ops)
            }
        }
        Value::Object(obj) => {
            if let Some(v) = obj.get("value") {
                parse_dynamic_float(v)
            } else {
                DynamicFloat::new(0.0)
            }
        }
        Value::Null => DynamicFloat::new(0.0),
    }
}

pub fn dynamic_float<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    let df = parse_dynamic_float(&value);
    if df.is_dynamic() {
        Ok(Some(df.get_constant()))
    } else {
        Ok(Some(df.get_constant()))
    }
}

pub fn dynamic_float_vec<'de, D>(deserializer: D) -> Result<Vec<Option<f32>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Array(arr) => {
            let mut result = vec![];
            for item in arr {
                let df = parse_dynamic_float(&item);
                if df.is_dynamic() {
                    result.push(Some(df.get_constant()));
                } else {
                    result.push(Some(df.get_constant()));
                }
            }
            Ok(result)
        }
        Value::Null => Ok(vec![]),
        _ => Ok(vec![]),
    }
}

pub fn dynamic_float_hashmap<'de, D>(
    deserializer: D,
) -> Result<std::collections::HashMap<String, Option<f32>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Object(map) => {
            let mut result = std::collections::HashMap::new();
            for (key, val) in map {
                let df = parse_dynamic_float(&val);
                if df.is_dynamic() {
                    result.insert(key, Some(df.get_constant()));
                } else {
                    result.insert(key, Some(df.get_constant()));
                }
            }
            Ok(result)
        }
        Value::Null => Ok(std::collections::HashMap::new()),
        _ => Ok(std::collections::HashMap::new()),
    }
}

pub fn skip_strings_in_vec<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Array(arr) => {
            let mut result = vec![];
            for item in arr {
                if !item.is_string() {
                    if let Ok(t) = T::deserialize(item) {
                        result.push(t);
                    }
                }
            }
            Ok(result)
        }
        Value::Null => Ok(vec![]),
        _ => Ok(vec![]),
    }
}
