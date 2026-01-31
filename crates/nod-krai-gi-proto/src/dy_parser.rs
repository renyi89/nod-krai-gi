use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use serde::{de, Serialize};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ReplaceValueConfig {
    pub n1: u64,
    pub n2: u64,
    pub n3: u64,
    pub n4: u64,
    pub n5: u64,
    pub const_in: u64,
    pub const_out: u64,
}

#[derive(Debug)]
pub struct VersionProtocol {
    pub replace_value_map: std::collections::HashMap<String, ReplaceValueConfig>,
    pub cmd_id_map: std::collections::HashMap<String, u16>,
    pub cmd_id_map_re: std::collections::HashMap<u16, String>,
    pub file_descriptor: protobuf::reflect::FileDescriptor,
}

pub static MULTI_VERSION_PROTOCOL: std::sync::OnceLock<
    Arc<dashmap::DashMap<String, VersionProtocol>>,
> = std::sync::OnceLock::new();

pub fn init() {
    MULTI_VERSION_PROTOCOL
        .set(Arc::new(dashmap::DashMap::new()))
        .expect("TODO: panic message");
    let (tx, rx) = std::sync::mpsc::channel();
    let mut debouncer = new_debouncer(Duration::from_millis(500), tx).unwrap();

    let path = std::path::Path::new("./assets/proto/");
    let mut sub_paths: Vec<String> = Vec::new();
    for dir in path.read_dir().unwrap() {
        if let Ok(dir) = dir {
            if dir.file_type().unwrap().is_dir() {
                tracing::info!("find dir {}", dir.file_name().into_string().unwrap());
                sub_paths.push(dir.file_name().into_string().unwrap());
                debouncer
                    .watcher()
                    .watch(&*dir.path(), RecursiveMode::Recursive)
                    .unwrap();
                update_proto(dir.path().file_name().unwrap().to_str().unwrap());
            }
        }
    }

    for res in rx {
        match res {
            Ok(events) => {
                let mut path = std::path::PathBuf::new();
                if !events.is_empty() {
                    events.iter().for_each(|event| {
                        if path.file_name().is_none()
                            || path.file_name().unwrap() != "all.proto"
                            || event.path.file_name().unwrap() == "all.proto"
                        {
                            path = event.path.clone();
                        }
                    })
                }
                if path.is_file() && path.file_name().unwrap() == "all.proto" {
                    update_proto(
                        path.parent()
                            .unwrap()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap(),
                    );
                }
                if path.is_file() && path.file_name().unwrap() == "protocol.json" {
                    update_cmd(
                        path.parent()
                            .unwrap()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap(),
                    );
                }
            }
            Err(e) => {
                tracing::error!(error = ?e, "error while watching file");
            }
        }
    }
}
fn update_proto(name: &str) {
    tracing::info!("begin update proto {}", name);
    let proto_file_path = std::path::Path::new("./assets/proto/")
        .join(name)
        .join("all.proto");

    let protocol_file_path = std::path::Path::new("./assets/proto/")
        .join(name)
        .join("protocol.json");

    let replace_file_path = std::path::Path::new("./assets/proto/")
        .join(name)
        .join("replace_value.json");

    if !proto_file_path.exists() || !protocol_file_path.exists() || !replace_file_path.exists() {
        tracing::info!("file not exists {}", name);
        return;
    }

    match protobuf_parse::Parser::new()
        .pure()
        .includes(&[std::path::Path::new(&proto_file_path).parent().unwrap()])
        .input(&std::path::Path::new(&proto_file_path))
        .parse_and_typecheck()
    {
        Ok(parsed) => {
            // Parse text `.proto` file to `FileDescriptorProto` message.
            // Note this API is not stable and subject to change.
            // But binary protos can always be generated manually with `protoc` command.
            let mut file_descriptor_protos = parsed.file_descriptors;

            // This is our .proto file converted to `FileDescriptorProto` from `descriptor.proto`.
            let file_descriptor_proto: protobuf::descriptor::FileDescriptorProto =
                file_descriptor_protos.pop().unwrap();
            // Now this `FileDescriptorProto` initialized for reflective access.
            let file_descriptor: protobuf::reflect::FileDescriptor =
                protobuf::reflect::FileDescriptor::new_dynamic(file_descriptor_proto, &[]).unwrap();

            let json = serde_json::from_str::<serde_json::Value>(
                std::fs::read_to_string(protocol_file_path)
                    .unwrap()
                    .as_str(),
            )
            .unwrap()
            .as_object()
            .unwrap()
            .clone();

            let mut cmd_id_map: std::collections::HashMap<String, u16> =
                std::collections::HashMap::new();
            let mut cmd_id_map_re: std::collections::HashMap<u16, String> =
                std::collections::HashMap::new();

            for (k, v) in json {
                cmd_id_map.insert(k.clone(), v.as_u64().unwrap() as u16);
                cmd_id_map_re.insert(v.as_u64().unwrap() as u16, k.clone());
            }

            let mut replace_value_map: std::collections::HashMap<String, ReplaceValueConfig> =
                std::collections::HashMap::new();

            if replace_file_path.exists() {
                replace_value_map =
                    serde_json::from_reader(std::fs::File::open(replace_file_path).unwrap())
                        .unwrap();
            };

            MULTI_VERSION_PROTOCOL.get().unwrap().insert(
                name.to_string(),
                VersionProtocol {
                    cmd_id_map,
                    cmd_id_map_re,
                    file_descriptor,
                    replace_value_map,
                },
            );

            tracing::info!("end update proto {}", name);
        }
        Err(error) => {
            tracing::info!("update proto error:{}", error);
        }
    }
}

fn update_cmd(name: &str) {
    tracing::info!("begin update cmd {}", name);
    let proto_file_path = std::path::Path::new("./assets/proto/")
        .join(name)
        .join("all.proto");

    let protocol_file_path = std::path::Path::new("./assets/proto/")
        .join(name)
        .join("protocol.json");

    let replace_file_path = std::path::Path::new("./assets/proto/")
        .join(name)
        .join("replace_value.json");

    if !proto_file_path.exists() || !protocol_file_path.exists() || !replace_file_path.exists() {
        tracing::info!("file not exists {}", name);
        return;
    }

    let json = serde_json::from_str::<serde_json::Value>(
        std::fs::read_to_string(protocol_file_path)
            .unwrap()
            .as_str(),
    )
    .unwrap()
    .as_object()
    .unwrap()
    .clone();

    let mut cmd_id_map: std::collections::HashMap<String, u16> = std::collections::HashMap::new();
    let mut cmd_id_map_re: std::collections::HashMap<u16, String> =
        std::collections::HashMap::new();

    for (k, v) in json {
        cmd_id_map.insert(k.clone(), v.as_u64().unwrap() as u16);
        cmd_id_map_re.insert(v.as_u64().unwrap() as u16, k.clone());
    }

    if MULTI_VERSION_PROTOCOL
        .get()
        .unwrap()
        .contains_key(&name.to_string())
    {
        let file_descriptor = MULTI_VERSION_PROTOCOL
            .get()
            .unwrap()
            .get(&name.to_string())
            .unwrap()
            .file_descriptor
            .clone();

        let replace_value_map = MULTI_VERSION_PROTOCOL
            .get()
            .unwrap()
            .get(&name.to_string())
            .unwrap()
            .replace_value_map
            .clone();

        MULTI_VERSION_PROTOCOL.get().unwrap().insert(
            name.to_string(),
            VersionProtocol {
                replace_value_map,
                cmd_id_map,
                cmd_id_map_re,
                file_descriptor,
            },
        );

        tracing::info!("end update cmd {}", name);
    }
}

pub fn get_version(first_cmd_id: u16) -> String {
    let mut version = "".to_string();
    MULTI_VERSION_PROTOCOL.get().unwrap().iter().for_each(|r| {
        let Some(message_name) = r.cmd_id_map_re.get(&first_cmd_id) else {
            return;
        };
        if message_name == "GetPlayerTokenReq" {
            version = r.key().clone();
        }
    });

    version
}

pub fn get_name_by_cmd_id_version(version: &str, cmd_id: u16) -> Option<String> {
    match MULTI_VERSION_PROTOCOL
        .get()
        .unwrap()
        .get(version)
        .unwrap()
        .cmd_id_map_re
        .get(&cmd_id)
    {
        None => {
            tracing::error!("failed to get name version:{} cmd_id:{}", version, cmd_id,);
            None
        }
        Some(message_name) => Some(message_name.clone()),
    }
}

pub fn get_cmd_id_by_name_version(version: &str, message_name: &str) -> Option<u16> {
    match MULTI_VERSION_PROTOCOL
        .get()
        .unwrap()
        .get(version)
        .unwrap()
        .cmd_id_map
        .get(message_name)
    {
        None => {
            tracing::error!(
                "failed to get cmd_id version:{} message_name:{}",
                version,
                message_name,
            );
            None
        }
        Some(cmd_id) => Some(*cmd_id),
    }
}

pub fn encode_to_vec_by_name_version<T>(
    version: &str,
    message_name: &str,
    value: &T,
) -> Option<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    match MULTI_VERSION_PROTOCOL
        .get()
        .unwrap()
        .get(version)
        .unwrap()
        .file_descriptor
        .message_by_package_relative_name(message_name)
    {
        None => {
            tracing::error!(
                "failed to find file_descriptor version:{} message_name:{}",
                version,
                message_name,
            );
            None
        }
        Some(message) => {
            let json_str = serde_json::to_string(&value).unwrap().clone();
            match protobuf_json_mapping::parse_dyn_from_str_with_options(
                &message,
                json_str.clone().as_str(),
                &protobuf_json_mapping::ParseOptions {
                    ignore_unknown_fields: true,
                    ..Default::default()
                },
            ) {
                Ok(parse_result) => {
                    let body = parse_result.write_to_bytes_dyn().unwrap();
                    Some(body)
                }
                Err(error) => {
                    tracing::error!(
                        "failed to write json version:{} message_name:{} error:{} json:{}",
                        version,
                        message_name,
                        error,
                        json_str.as_str()
                    );
                    None
                }
            }
        }
    }
}

pub fn decode_from_vec_by_name_version<T: Sized + Serialize + Default>(
    version: &str,
    message_name: &str,
    body: &[u8],
) -> Option<T>
where
    T: for<'a> de::Deserialize<'a>,
{
    match MULTI_VERSION_PROTOCOL
        .get()
        .unwrap()
        .get(version)
        .unwrap()
        .file_descriptor
        .message_by_package_relative_name(message_name)
    {
        None => {
            tracing::error!(
                "failed to find file_descriptor version:{} message_name:{}",
                version,
                message_name,
            );
            None
        }
        Some(message) => {
            let mut data = message.new_instance();

            match data.merge_from_bytes_dyn(body) {
                Ok(_) => {}
                Err(error) => {
                    tracing::error!(
                        "failed to merge_from_bytes_dyn version:{} message_name:{} error:{}",
                        version,
                        message_name,
                        error,
                    );
                    return None;
                }
            }

            match serde_json::from_str(
                &*protobuf_json_mapping::print_to_string_with_options(
                    data.as_ref(),
                    &protobuf_json_mapping::PrintOptions {
                        proto_field_name: true,
                        enum_values_int: true,
                        ..Default::default()
                    },
                )
                .unwrap(),
            ) {
                Ok(value) => Some(value),
                Err(error) => {
                    tracing::error!(
                        "failed to parse json version:{} message_name:{}error:{}",
                        version,
                        message_name,
                        error,
                    );
                    None
                }
            }
        }
    }
}

pub fn replace_out_u32(version: &str, field_name: &str, ov: u32) -> u32 {
    match MULTI_VERSION_PROTOCOL
        .get()
        .unwrap()
        .get(version)
        .unwrap()
        .replace_value_map
        .get(field_name)
    {
        None => ov,
        Some(config) => {
            let result;
            if config.const_out != 0 {
                result = config.const_out as u32;
            } else {
                result = (ov
                    .wrapping_add(config.n1 as u32)
                    .wrapping_sub(config.n2 as u32)
                    ^ (config.n3 as u32))
                        .wrapping_add(config.n4 as u32)
                        .wrapping_sub(config.n5 as u32);
            }
            tracing::debug!(
                "replace_out_u32 version:{} field_name:{} value1:{} value2:{}",
                version,
                field_name,
                ov,
                result
            );
            result
        }
    }
}

pub fn replace_in_u32(version: &str, field_name: &str, ov: u32) -> u32 {
    match MULTI_VERSION_PROTOCOL
        .get()
        .unwrap()
        .get(version)
        .unwrap()
        .replace_value_map
        .get(field_name)
    {
        None => ov,
        Some(config) => {
            let result;
            if config.const_in != 0 {
                result = config.const_in as u32;
            } else {
                result = (ov
                    .wrapping_add(config.n5 as u32)
                    .wrapping_sub(config.n4 as u32)
                    ^ (config.n3 as u32))
                        .wrapping_add(config.n2 as u32)
                        .wrapping_sub(config.n1 as u32);
            }
            tracing::debug!(
                "replace_in_u32 version:{} field_name:{} value1:{} value2:{}",
                version,
                field_name,
                ov,
                result
            );
            result
        }
    }
}


pub fn replace_out_i32(version: &str, field_name: &str, ov: i32) -> i32 {
    match MULTI_VERSION_PROTOCOL
        .get()
        .unwrap()
        .get(version)
        .unwrap()
        .replace_value_map
        .get(field_name)
    {
        None => ov,
        Some(config) => {
            let result;
            if config.const_out != 0 {
                result = config.const_out as i32;
            } else {
                result = (ov
                    .wrapping_add(config.n1 as i32)
                    .wrapping_sub(config.n2 as i32)
                    ^ (config.n3 as i32))
                    .wrapping_add(config.n4 as i32)
                    .wrapping_sub(config.n5 as i32);
            }
            tracing::debug!(
                "replace_out_i32 version:{} field_name:{} value1:{} value2:{}",
                version,
                field_name,
                ov,
                result
            );
            result
        }
    }
}

pub fn replace_in_i32(version: &str, field_name: &str, ov: i32) -> i32 {
    match MULTI_VERSION_PROTOCOL
        .get()
        .unwrap()
        .get(version)
        .unwrap()
        .replace_value_map
        .get(field_name)
    {
        None => ov,
        Some(config) => {
            let result;
            if config.const_in != 0 {
                result = config.const_in as i32;
            } else {
                result = (ov
                    .wrapping_add(config.n5 as i32)
                    .wrapping_sub(config.n4 as i32)
                    ^ (config.n3 as i32))
                    .wrapping_add(config.n2 as i32)
                    .wrapping_sub(config.n1 as i32);
            }
            tracing::debug!(
                "replace_in_i32 version:{} field_name:{} value1:{} value2:{}",
                version,
                field_name,
                ov,
                result
            );
            result
        }
    }
}


pub fn replace_out_u64(version: &str, field_name: &str, ov: u64) -> u64 {
    match MULTI_VERSION_PROTOCOL
        .get()
        .unwrap()
        .get(version)
        .unwrap()
        .replace_value_map
        .get(field_name)
    {
        None => ov,
        Some(config) => {
            let result;
            if config.const_out != 0 {
                result = config.const_out as u64;
            } else {
                result = (ov
                    .wrapping_add(config.n1 as u64)
                    .wrapping_sub(config.n2 as u64)
                    ^ (config.n3 as u64))
                    .wrapping_add(config.n4 as u64)
                    .wrapping_sub(config.n5 as u64);
            }
            tracing::debug!(
                "replace_out_u64 version:{} field_name:{} value1:{} value2:{}",
                version,
                field_name,
                ov,
                result
            );
            result
        }
    }
}

pub fn replace_in_u64(version: &str, field_name: &str, ov: u64) -> u64 {
    match MULTI_VERSION_PROTOCOL
        .get()
        .unwrap()
        .get(version)
        .unwrap()
        .replace_value_map
        .get(field_name)
    {
        None => ov,
        Some(config) => {
            let result;
            if config.const_in != 0 {
                result = config.const_in as u64;
            } else {
                result = (ov
                    .wrapping_add(config.n5 as u64)
                    .wrapping_sub(config.n4 as u64)
                    ^ (config.n3 as u64))
                    .wrapping_add(config.n2 as u64)
                    .wrapping_sub(config.n1 as u64);
            }
            tracing::debug!(
                "replace_in_u64 version:{} field_name:{} value1:{} value2:{}",
                version,
                field_name,
                ov,
                result
            );
            result
        }
    }
}