use heck::{ToPascalCase, ToSnakeCase, ToKebabCase, ToShoutySnakeCase};
use chrono::{FixedOffset, Utc};
use std::fs::File;
use std::io::Read;
use std::process::Command;
use serde_json::Value;

pub fn get_user_info() -> (String, String) {
    let mut username = String::new();
    let mut email = String::new();
    if let Ok(output) = Command::new("git").args(["config", "--get", "user.email"]).output() {
        if output.status.success() {
            if let Ok(utf8_str) = String::from_utf8(output.stdout) {
                email = utf8_str.trim().to_string();
            }
        }
    }

    if let Ok(output) = Command::new("git").args(["config", "--get", "user.name"]).output() {
        if output.status.success() {
            if let Ok(utf8_str) = String::from_utf8(output.stdout) {
                username = utf8_str.trim().to_string();
            }
        }
    }

    if username.is_empty() {
        if let Ok(output) = Command::new("whoami").output() {
            if output.status.success() {
                if let Ok(utf8_str) = String::from_utf8(output.stdout) {
                    username = utf8_str.trim().to_string();
                }
            }
        }
    }

    (username, email)
}

pub fn get_version() -> String {
    let mut version = "0.0.0".to_string();
    if let Ok(mut file) = File::open("package.json") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            if let Ok(json) = serde_json::from_str::<Value>(&contents) {
                if let Some(ver) = json["version"].as_str() {
                    version = ver.to_string();
                }
            }
        }
    }
    version
}

pub fn get_date() -> String {
    let offset = FixedOffset::east_opt(8 * 3600).unwrap();
    Utc::now().with_timezone(&offset).format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn get_name_variants(name: &str) -> (String, String, String, String) {
    let pascal_name = name.to_pascal_case();
    let snake_name = name.to_snake_case();
    let kebab_name = name.to_kebab_case();
    let constant_name = name.to_shouty_snake_case();
    (pascal_name, snake_name, kebab_name, constant_name)
}