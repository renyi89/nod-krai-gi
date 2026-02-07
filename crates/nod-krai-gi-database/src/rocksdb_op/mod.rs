mod sdk_rocksdb_op;

pub use sdk_rocksdb_op::{
    insert_combo_token, insert_sdk_account, select_combo_token_by_account, SelectSdkAccount,
};

use crate::{
    data::{PlayerDataRow, UserUidRow},
    DbConnection, DbError,
};

pub fn insert_or_update_player_data(
    conn: &DbConnection,
    uid: i32,
    data: serde_json::Value,
) -> Result<(), DbError> {
    let key = format!("player_data:{}", uid);
    let player_data = PlayerDataRow { uid, data };
    let value = serde_json::to_vec(&player_data)?;
    conn.0.put(key, value)?;
    Ok(())
}

pub fn select_player_data_by_uid(
    conn: &DbConnection,
    uid: i32,
) -> Result<Option<PlayerDataRow>, DbError> {
    let key = format!("player_data:{}", uid);
    match conn.0.get(key)? {
        Some(value) => {
            let player_data: PlayerDataRow = serde_json::from_slice(&value)?;
            Ok(Some(player_data))
        }
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
