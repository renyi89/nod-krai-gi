use crate::excel;
use std::collections::HashMap;
use std::sync::Arc;
use common::string_util::InternString;

static FETTER_DATA_ENTRIES: std::sync::OnceLock<
    Arc<HashMap<u32, Vec<FetterDataConfig>>>,
> = std::sync::OnceLock::new();

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenCondData {
    pub cond_type: InternString,
    pub param_list: Vec<u32>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetterDataConfig {
    pub avatar_id: u32,
    pub fetter_id: u32,
    pub open_conds: Vec<OpenCondData>,
}

pub trait FetterDataConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, FetterDataConfig>;

    fn get_fetter_data_entries() -> &'static Arc<HashMap<u32, Vec<FetterDataConfig>>>;
}

impl FetterDataConfigKeyed<u32> for FetterDataConfig {
    fn key(&self) -> u32 {
        self.fetter_id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, FetterDataConfig> {
        let mut list = vec![];

        for file in vec![
            "FetterInfoExcelConfigData.json",
            "FettersExcelConfigData.json",
            "FetterStoryExcelConfigData.json",
            "PhotographExpressionExcelConfigData.json",
            "PhotographPosenameExcelConfigData.json",
        ] {
            let file = std::fs::File::open(&format!("{excel_bin_output_path}/{file}")).unwrap();
            let content = std::io::BufReader::new(file);
            let mut sub_list: Vec<FetterDataConfig> = serde_json::from_reader(content).unwrap();
            list.append(&mut sub_list);
        }

        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();

        data
    }

    fn get_fetter_data_entries() -> &'static Arc<HashMap<u32, Vec<FetterDataConfig>>> {
        if FETTER_DATA_ENTRIES.get().is_some() {
            FETTER_DATA_ENTRIES.get().unwrap()
        } else {
            let fetter_data_config_collection_clone =
                std::sync::Arc::clone(excel::fetter_data_config_collection::get());

            let mut fetter_data_entries: HashMap<u32, Vec<FetterDataConfig>> = HashMap::new();

            for x in fetter_data_config_collection_clone.values() {
                if fetter_data_entries.contains_key(&x.avatar_id) {
                    let mut list = fetter_data_entries.get(&x.avatar_id).unwrap().clone();
                    list.push(x.clone());
                    fetter_data_entries.insert(x.avatar_id, list);
                } else {
                    fetter_data_entries.insert(x.avatar_id, vec![x.clone()]);
                }
            }

            let _ = FETTER_DATA_ENTRIES.set(Arc::new(fetter_data_entries));
            FETTER_DATA_ENTRIES.get().unwrap()
        }
    }
}
