use std::fs::{self};
use std::path::Path;

pub fn main() {
    println!("cargo:rerun-if-changed=proto");

    let _ = fs::create_dir("gen/");

    let mut config = prost_build::Config::new();
    config.out_dir("gen");
    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    config.message_attribute(".", "#[serde(default)]");
    config.field_attribute(".", "#[serde(skip_serializing_if = \"crate::is_default\")]");

    // config.enum_attribute(".","#[serde(rename_all = \"SCREAMING_SNAKE_CASE\")]");

    let mut files: Vec<String> = Vec::new();

    let path = Path::new("./proto");
    for file in path.read_dir().unwrap() {
        if let Ok(file) = file {
            if file.file_type().unwrap().is_file() {
                let file_name = file.file_name().clone();
                let file_name_str = file_name.to_str().expect("REASON").to_string();
                files.push(file_name_str);
            }
        }
    }

    config.compile_protos(&*files, &["proto"]).unwrap();
    let path = "gen/_.rs"; // prost 生成的文件路径
    let content = fs::read_to_string(path).expect("无法读取文件");

    let mut is_enum = false;
    let mut is_oneof = false;
    let mut output1 = String::new();
    for line in content.lines() {
        if line.trim_start().starts_with("#[prost(bytes = \"vec\"") {
            output1.push_str("    #[serde(with=\"crate::base64\")]\n");
        }
        if line.trim_start().starts_with("#[prost(uint64, tag") {
            output1.push_str("    #[serde(with=\"crate::u64_string\")]\n");
        }
        if line.trim_start().starts_with("#[prost(uint64, repeated,") {
            output1.push_str("    #[serde(with=\"crate::u64_repeated_string\")]\n");
        }
        if line.trim_start().starts_with("#[prost(map = ") {
            let map_type = line
                .trim_start()
                .split("#[prost(map = \"")
                .nth(1)
                .unwrap_or("")
                .split("\"")
                .nth(0)
                .unwrap_or("");
            let parts: Vec<&str> = map_type.split(", ").collect();
            if parts.len() == 2 {
                let (key_type, value_type) = (parts[0], parts[1]);
                if key_type == "uint64" && value_type == "uint64" {
                    output1.push_str("    #[serde(with=\"crate::u64_map_both_string\")]\n");
                } else if key_type == "uint64" {
                    output1.push_str("    #[serde(with=\"crate::u64_map_key_string\")]\n");
                } else if value_type == "uint64" {
                    output1.push_str("    #[serde(with=\"crate::u64_map_value_string\")]\n");
                }
            }
        }
        if line.trim_start().starts_with("#[prost(oneof =") {
            output1.push_str("    #[serde(flatten)]\n");
        }
        if line.trim_start().starts_with("pub enum ") {
            is_enum = true;
        }
        if line.contains("::prost::Oneof)]") {
            is_oneof = true;
        }
        if is_enum && line.contains("}") {
            is_enum = false;
        }
        if is_oneof && line.contains("}") {
            is_oneof = false;
        }
        if is_enum && line.contains("skip_serializing_if") {
            continue;
        }
        if is_enum && is_oneof && line.trim_start().starts_with("pub enum ") {
            output1.push_str("    #[serde(rename_all = \"snake_case\")]\n");
        }
        output1.push_str(line);
        output1.push_str("\n");
    }

    fs::write(path, output1).expect("无法写入文件");
}
