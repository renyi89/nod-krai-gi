use nod_krai_gi_database::{rocksdb_op, DbConnection, DbError};
use nod_krai_gi_persistence::player_information::PlayerDataBin;
use tokio::{
    select,
    sync::{mpsc, oneshot},
};

use crate::player_info_util;

enum DbOperation {
    Fetch(u32, oneshot::Sender<Option<PlayerDataBin>>),
    FetchUserUid(String, oneshot::Sender<Result<u32, DbError>>),
}

pub struct DbWorkerHandle(mpsc::Sender<DbOperation>);

impl DbWorkerHandle {
    pub async fn fetch(&self, uid: u32) -> Option<PlayerDataBin> {
        let (tx, rx) = oneshot::channel();
        let _ = self.0.send(DbOperation::Fetch(uid, tx)).await;

        rx.await.ok().flatten()
    }

    pub async fn fetch_user_uid(&self, account_uid: String) -> Result<u32, DbError> {
        let (tx, rx) = oneshot::channel();
        let _ = self
            .0
            .send(DbOperation::FetchUserUid(account_uid, tx))
            .await;

        rx.await.ok().unwrap()
    }
}

pub fn start(connection: DbConnection) -> (DbWorkerHandle, mpsc::Sender<(u32, serde_json::Value)>) {
    let (op_tx, op_rx) = mpsc::channel(32);
    let (save_data_tx, save_data_rx) = mpsc::channel(32);

    tokio::spawn(async move {
        db_work_loop(connection, op_rx, save_data_rx).await;
    });

    (DbWorkerHandle(op_tx), save_data_tx)
}

async fn db_work_loop(
    connection: DbConnection,
    mut op_rx: mpsc::Receiver<DbOperation>,
    mut save_data_rx: mpsc::Receiver<(u32, serde_json::Value)>,
) {
    loop {
        select! {
            op = op_rx.recv() => {
                match op {
                    Some(DbOperation::Fetch(uid, tx)) => {
                        let result = match rocksdb_op::select_player_data_by_uid(&connection, uid as i32)
                        {
                            Ok(Some(row)) => Some(serde_json::from_value(row.data).unwrap_or_else(|err| {
                                // as of early development state, player info schema will change from time to time
                                // it's better to replace it with default one everytime it changes, for now
                                tracing::warn!("failed to deserialize player data (uid: {uid}), replacing with default, error: {err}");
                                player_info_util::create_default_player_information(uid, String::from("nod-krai-gi-rs"))
                            })),
                            Ok(None) => Some(player_info_util::create_default_player_information(
                                uid,
                                String::from("nod-krai-gi-rs"),
                            )),
                            Err(_) => None,
                        };
                        let _ = tx.send(result);
                    }
                    Some(DbOperation::FetchUserUid(account_uid, tx)) => {
                          let result =   match rocksdb_op::select_user_uid_by_account_uid(&connection, &*account_uid).inspect_err(|err| tracing::error!("failed to select user uid: {err}")).unwrap()
                        {
                            Some(uid) => Ok(uid.uid as u32),
                            None => Ok(rocksdb_op::insert_user_uid(&connection, &*account_uid).inspect_err(|err| tracing::error!("failed to insert user uid: {err}")).unwrap().uid as u32)
                        };
                        let _ = tx.send(result);
                    }
                    _ => {}
                }
            },
            save_data = save_data_rx.recv() => {
                if let Some((uid, data)) = save_data {
                    if let Err(err) =
                        rocksdb_op::insert_or_update_player_data(&connection, uid as i32, data)
                    {
                        tracing::error!("failed to save player data: {err}");
                    }
                }
            }
        }
    }
}
