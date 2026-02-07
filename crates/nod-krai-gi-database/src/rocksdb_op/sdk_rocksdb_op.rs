use rand::distributions::{Alphanumeric, DistString};

use crate::{
    data::{self, ComboToken, SdkAccount},
    DbConnection, DbError,
};

pub fn insert_sdk_account(
    conn: &DbConnection,
    username: data::Username,
    password: data::Password,
) -> Result<data::SdkAccount, DbError> {
    let token = Alphanumeric.sample_string(&mut rand::thread_rng(), 64);
    
    // 生成自增 ID
    let uid_counter_key = "sdk_account_uid_counter";
    let mut uid = 1;
    
    if let Some(value) = conn.0.get(uid_counter_key)? {
        uid = String::from_utf8_lossy(&value).parse().unwrap_or(1) + 1;
    }
    
    let sdk_account = SdkAccount {
        uid,
        token,
        username,
        password,
    };
    
    let username_key = format!("sdk_account:username:{}", sdk_account.username.as_str());
    let uid_key = format!("sdk_account:uid:{}", uid);
    
    let value = serde_json::to_vec(&sdk_account)?;
    
    crate::batch_write(conn, |batch| {
        batch.put(uid_counter_key, uid.to_string());
        batch.put(username_key, value.clone());
        batch.put(uid_key, value);
    })?;
    
    Ok(sdk_account)
}

pub enum SelectSdkAccount<'s> {
    ByUsername(&'s str),
    ByUid(i32),
}

impl SelectSdkAccount<'_> {
    pub fn fetch(self, conn: &DbConnection) -> Result<data::SdkAccount, DbError> {
        match self {
            Self::ByUsername(username) => {
                let key = format!("sdk_account:username:{}", username);
                match conn.0.get(key)? {
                    Some(value) => {
                        let account: SdkAccount = serde_json::from_slice(&value)?;
                        Ok(account)
                    }
                    None => Err(DbError::NotFound),
                }
            }
            Self::ByUid(uid) => {
                let key = format!("sdk_account:uid:{}", uid);
                match conn.0.get(key)? {
                    Some(value) => {
                        let account: SdkAccount = serde_json::from_slice(&value)?;
                        Ok(account)
                    }
                    None => Err(DbError::NotFound),
                }
            }
        }
    }
}

pub fn insert_combo_token(
    conn: &DbConnection,
    account_uid: &str,
    device_id: &str,
) -> Result<ComboToken, DbError> {
    let token = Alphanumeric.sample_string(&mut rand::thread_rng(), 64);
    
    let combo_token = ComboToken {
        account_uid: account_uid.to_string(),
        token,
        device_id: device_id.to_string(),
    };
    
    let key = format!("combo_token:{}", account_uid);
    let value = serde_json::to_vec(&combo_token)?;
    conn.0.put(key, value)?;
    
    Ok(combo_token)
}

pub fn select_combo_token_by_account(
    conn: &DbConnection,
    account_uid: &str,
) -> Result<ComboToken, DbError> {
    let key = format!("combo_token:{}", account_uid);
    match conn.0.get(key)? {
        Some(value) => {
            let token: ComboToken = serde_json::from_slice(&value)?;
            Ok(token)
        }
        None => Err(DbError::NotFound),
    }
}
