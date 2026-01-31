use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub db_file: String,
}
