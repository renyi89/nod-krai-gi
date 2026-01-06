CREATE TABLE IF NOT EXISTS t_sdk_account
(
    uid      INTEGER PRIMARY KEY AUTOINCREMENT,
    token    TEXT NOT NULL,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    UNIQUE (username)
);

CREATE TABLE IF NOT EXISTS t_combo_token
(
    account_uid TEXT PRIMARY KEY,
    token       TEXT NOT NULL,
    device_id   TEXT NOT NULL,
    UNIQUE (account_uid, device_id)
);

CREATE TABLE IF NOT EXISTS t_user_uid
(
    uid         INTEGER PRIMARY KEY AUTOINCREMENT,
    account_uid TEXT UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS t_player_data
(
    uid  INTEGER PRIMARY KEY,
    data TEXT NOT NULL
);

INSERT INTO sqlite_sequence (name, seq)
SELECT 't_user_uid', 10000
    WHERE NOT EXISTS (
    SELECT 1 FROM sqlite_sequence WHERE name = 't_user_uid'
);