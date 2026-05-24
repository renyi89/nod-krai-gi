use common::string_util::InternString;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BannerWeight {
    pub pulls: u32,
    pub weight: u32,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GachaBanner {
    pub comment: InternString,
    pub gacha_type: u32,
    pub schedule_id: u32,
    pub prefab_path: InternString,
    pub preview_prefab_path: Option<InternString>,
    pub title_path: InternString,
    pub cost_item_id: u32,
    pub cost_item_amount_10: Option<u32>,
    pub sort_id: u32,
    pub rate_up_items_4: Vec<u32>,
    pub rate_up_items_5: Vec<u32>,
    pub fallback_items_3: Option<Vec<u32>>,
    pub fallback_items_4_pool_1: Option<Vec<u32>>,
    pub fallback_items_4_pool_2: Option<Vec<u32>>,
    pub fallback_items_5_pool_1: Option<Vec<u32>>,
    pub fallback_items_5_pool_2: Option<Vec<u32>>,
    pub weights_4: Option<Vec<BannerWeight>>,
    pub weights_5: Option<Vec<BannerWeight>>,
    pub pool_balance_weights_4: Option<Vec<BannerWeight>>,
    pub pool_balance_weights_5: Option<Vec<BannerWeight>>,
    pub event_chance_4: Option<u32>,
    pub event_chance_5: Option<u32>,
    pub gacha_times_limit: Option<u32>,
}

pub trait GachaBannersKeyed<K> {
    fn key(&self) -> K;

    fn load(custom_output_path: &str) -> HashMap<K, GachaBanner>;
}

impl GachaBannersKeyed<u32> for GachaBanner {
    fn key(&self) -> u32 {
        self.schedule_id
    }

    fn load(custom_output_path: &str) -> HashMap<u32, GachaBanner> {
        let json = std::fs::read(&format!("{custom_output_path}/GachaBanners.json")).unwrap();
        let list: Vec<GachaBanner> = serde_json::from_slice(&*json).unwrap();
        let data = list.iter().map(|item| (item.key(), item.clone())).collect();
        data
    }
}
