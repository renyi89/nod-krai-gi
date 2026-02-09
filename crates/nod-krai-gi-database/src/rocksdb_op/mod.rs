mod sdk_rocksdb_op;

pub use sdk_rocksdb_op::{
    insert_combo_token, insert_sdk_account, select_combo_token_by_account, SelectSdkAccount,
};

use crate::{data::UserUidRow, DbConnection, DbError};

pub fn insert_or_update_player_data(
    conn: &DbConnection,
    uid: i32,
    data: Vec<u8>,
) -> Result<(), DbError> {
    let key = format!("player_data:{}", uid);
    conn.0.put(key, data)?;
    Ok(())
}

pub fn select_player_data_by_uid(
    conn: &DbConnection,
    uid: i32,
) -> Result<Option<Vec<u8>>, DbError> {
    let key = format!("player_data:{}", uid);
    match conn.0.get(key)? {
        Some(value) => Ok(Some(value)),
        None => Ok(None),
    }
}

pub fn select_user_uid_by_account_uid(
    conn: &DbConnection,
    account_uid: &str,
) -> Result<Option<UserUidRow>, DbError> {
    let key = format!("user_uid:{}", account_uid);
    match conn.0.get(key)? {
        Some(value) => {
            let user_uid: UserUidRow = serde_json::from_slice(&value)?;
            Ok(Some(user_uid))
        }
        None => Ok(None),
    }
}

pub fn insert_user_uid(conn: &DbConnection, account_uid: &str) -> Result<UserUidRow, DbError> {
    let key = format!("user_uid:{}", account_uid);

    let uid_key = "uid_counter";
    let mut uid = 10000;

    if let Some(value) = conn.0.get(uid_key)? {
        uid = String::from_utf8_lossy(&value).parse().unwrap_or(10000) + 1;
    }

    let user_uid = UserUidRow {
        account_uid: account_uid.to_string(),
        uid,
    };

    let value = serde_json::to_vec(&user_uid)?;

    crate::batch_write(conn, |batch| {
        batch.put(uid_key, uid.to_string());
        batch.put(key, value);
    })?;

    Ok(user_uid)
}
