#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserUidRow {
    pub account_uid: String,
    pub uid: i32,
}