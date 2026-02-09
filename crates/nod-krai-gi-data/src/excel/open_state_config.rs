use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenStateConfig {
    pub id: u32,
    pub default_state: bool,
    pub allow_client_open: bool,
    pub cond: Vec<OpenStateCond>,
    pub system_open_ui_id: u32,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenStateCond {
    pub cond_type: OpenStateCondType,
    pub param: u32,
    pub param_2: u32,
}

#[derive(Debug, Default, Clone, serde::Deserialize)]
pub enum OpenStateCondType {
    #[serde(alias = "OPEN_STATE_COND_NONE")]
    #[default]
    None,
    #[serde(alias = "OPEN_STATE_COND_PLAYER_LEVEL")]
    PlayerLevel,
    #[serde(alias = "OPEN_STATE_COND_QUEST")]
    Quest,
    #[serde(alias = "OPEN_STATE_COND_PARENT_QUEST")]
    ParentQuest,
    #[serde(alias = "OPEN_STATE_COND_GCG_LEVEL")]
    GcgLevel,
    #[serde(alias = "OPEN_STATE_OFFERING_LEVEL")]
    OfferingLevel,
    #[serde(alias = "OPEN_STATE_CITY_REPUTATION_LEVEL")]
    CityReputationLevel,
    #[serde(alias = "OPEN_STATE_TRIBAL_REPUTATION_FINISH_NUM")]
    TribalReputationFinishNum,
    #[serde(alias = "OPEN_STATE_COND_PLAYER_BEYOND_LEVEL")]
    PlayerBeyondLevel,
}

pub trait OpenStateConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, OpenStateConfig>;
}

impl OpenStateConfigKeyed<u32> for OpenStateConfig {
    fn key(&self) -> u32 {
        self.id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, OpenStateConfig> {
        let json =
            std::fs::read(&format!("{excel_bin_output_path}/OpenStateConfigData.json"))
                .unwrap();
        let list: Vec<OpenStateConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();
        data
    }
}
