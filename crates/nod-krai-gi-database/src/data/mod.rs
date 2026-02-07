pub mod password;
pub mod username;

pub use password::Password;
pub use username::Username;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SdkAccount {
    pub uid: i32,
    pub token: String,
    pub username: Username,
    pub password: Password,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ComboToken {
    pub account_uid: String,
    pub token: String,
    pub device_id: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserUidRow {
    pub account_uid: String,
    pub uid: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PlayerDataRow {
    pub uid: i32,
    pub data: serde_json::Value,
}
