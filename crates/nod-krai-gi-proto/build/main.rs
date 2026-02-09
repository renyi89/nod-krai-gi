use std::fs::{self};
use std::path::Path;

pub fn main() {
    println!("cargo:rerun-if-changed=proto");
    println!("cargo:rerun-if-changed=server_only");

    let _ = fs::create_dir("gen/");

    process_proto_dir("./proto", &["proto"],"normal");
    
    process_proto_dir("./server_only", &["server_only"],"server_only");
}

fn process_proto_dir(dir: &str, includes: &[&str],target:&str) {
    let mut config = prost_build::Config::new();
    config.out_dir(format!("gen/{}",target).as_str());
    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    config.message_attribute(".", "#[serde(default)]");
    config.field_attribute(".", "#[serde(skip_serializing_if = \"crate::is_default\")]");

    let mut files: Vec<String> = Vec::new();

    let path = Path::new(dir);
    if path.read_dir().unwrap().count() == 0 {
        return;
    }
    for file in path.read_dir().unwrap() {
        if let Ok(file) = file {
            if file.file_type().unwrap().is_file() {
                let file_name = file.file_name().clone();
                let file_name_str = file_name.to_str().expect("REASON").to_string();
                files.push(file_name_str);
            }
        }
    }

    config.compile_protos(&*files, includes).unwrap();
    let path = format!("gen/{}/_.rs",target);
    let content = fs::read_to_string(path.as_str()).expect("can not read");

    let mut is_enum = false;
    let mut is_oneof = false;
    let mut output1 = String::new();
    let lines: Vec<&str> = content.lines().collect();
    for (i, line) in lines.iter().enumerate() {
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
        if line.trim_start().starts_with("#[prost(oneof =") || (line.trim_start().starts_with("#[prost(") && i + 1 < lines.len() && lines[i + 1].trim_start().starts_with("oneof =")) {
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

    fs::write(path, output1).expect("can not write");
}