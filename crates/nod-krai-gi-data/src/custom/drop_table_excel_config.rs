use crate::custom::{DropItemConfig, DropSubTableExcelConfig, DropTableLike};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DropTableExcelConfig {
    pub id: u32,
    pub random_type: u32,
    pub drop_level: u32,
    pub drop_vec: Vec<DropItemConfig>,
    pub node_type: u32,
    pub fall_to_ground: Option<bool>,
    pub source_type: Option<u32>,
    pub everyday_limit: Option<u32>,
    pub history_limit: Option<u32>,
    pub activity_limit: Option<u32>,
}

impl DropTableLike for DropTableExcelConfig {
    fn id(&self) -> u32 {
        self.id
    }
    fn random_type(&self) -> u32 {
        self.random_type
    }
    fn drop_vec(&self) -> &Vec<DropItemConfig> {
        &self.drop_vec
    }
}

enum DropTableRef<'a> {
    Table(&'a DropTableExcelConfig),
    SubTable(&'a DropSubTableExcelConfig),
}

pub fn resolve_drop(drop_table_id: u32, count: u32) -> Vec<(u32, u32)> {
    let mut rng = SmallRng::from_entropy();

    let drop_table_map = super::drop_table_excel_config_collection::get();
    let drop_sub_table_map = super::drop_sub_table_excel_config_collection::get();

    let table = drop_table_map
        .get(&drop_table_id)
        .map(DropTableRef::Table)
        .or_else(|| {
            drop_sub_table_map
                .get(&drop_table_id)
                .map(DropTableRef::SubTable)
        });

    if table.is_none() {
        return Vec::new();
    }

    let mut out = Vec::new();

    match table.unwrap() {
        DropTableRef::Table(t) => {
            process_drop_internal(
                t,
                count,
                &mut rng,
                drop_table_map,
                drop_sub_table_map,
                &mut out,
            );
        }
        DropTableRef::SubTable(t) => {
            process_drop_internal(
                t,
                count,
                &mut rng,
                drop_table_map,
                drop_sub_table_map,
                &mut out,
            );
        }
    }

    out
}

fn process_drop_internal<T: DropTableLike>(
    drop_data: &T,
    count: u32,
    rng: &mut SmallRng,
    drop_table_map: &HashMap<u32, DropTableExcelConfig>,
    drop_sub_table_map: &HashMap<u32, DropSubTableExcelConfig>,
    out: &mut Vec<(u32, u32)>,
) {
    //if count > 1, call recursively count times
    if count > 1 {
        for _ in 0..count {
            process_drop_internal(drop_data, 1, rng, drop_table_map, drop_sub_table_map, out);
        }
        return;
    }

    let drop_vec = drop_data.drop_vec();

    match drop_data.random_type() {
        // randomType == 0 : weighted single selection
        0 => {
            let mut weight_sum = 0u32;

            for i in drop_vec {
                if i.item_id == 0 {
                    continue;
                }
                weight_sum += i.weight;
            }

            if weight_sum == 0 {
                return;
            }

            let roll = rng.gen_range(0..weight_sum);
            let mut sum = 0u32;

            for i in drop_vec {
                let id = i.item_id;
                if id == 0 {
                    continue;
                }

                sum += i.weight;

                if roll < sum {
                    let amount = calculate_drop_amount(i, rng);
                    if amount == 0 {
                        break;
                    }

                    process_or_add(id, amount, rng, drop_table_map, drop_sub_table_map, out);

                    break;
                }
            }
        }

        // randomType == 1 : independent probability for each item
        1 => {
            for i in drop_vec {
                let id = i.item_id;
                if id == 0 {
                    continue;
                }

                let roll = rng.gen_range(0..10000);
                if roll < i.weight {
                    let amount = calculate_drop_amount(i, rng);
                    if amount == 0 {
                        continue;
                    }

                    process_or_add(id, amount, rng, drop_table_map, drop_sub_table_map, out);
                }
            }
        }

        _ => {}
    }
}

fn process_or_add(
    id: u32,
    amount: u32,
    rng: &mut SmallRng,
    drop_table_map: &HashMap<u32, DropTableExcelConfig>,
    drop_sub_table_map: &HashMap<u32, DropSubTableExcelConfig>,
    out: &mut Vec<(u32, u32)>,
) {
    if let Some(t) = drop_table_map.get(&id) {
        process_drop_internal(t, amount, rng, drop_table_map, drop_sub_table_map, out);
    } else if let Some(t) = drop_sub_table_map.get(&id) {
        process_drop_internal(t, amount, rng, drop_table_map, drop_sub_table_map, out);
    } else {
        add_item(out, id, amount);
    }
}

fn calculate_drop_amount(i: &DropItemConfig, rng: &mut SmallRng) -> u32 {
    let s = i.count_range.as_str();

    if let Some(pos) = s.find(';') {
        let min: u32 = s[..pos].parse().unwrap_or(0);
        let max: u32 = s[pos + 1..].parse().unwrap_or(min);
        return rng.gen_range(min..=max);
    }

    if s.contains('.') {
        let expect: f64 = s.parse().unwrap_or(0.0);
        let base = expect.floor() as u32;
        let frac = expect - base as f64;

        if rng.gen::<f64>() < frac {
            return base + 1;
        } else {
            return base;
        }
    }

    s.parse().unwrap_or(0)
}

fn add_item(out: &mut Vec<(u32, u32)>, id: u32, amount: u32) {
    for (item_id, cnt) in out.iter_mut() {
        if *item_id == id {
            *cnt += amount;
            return;
        }
    }
    out.push((id, amount));
}

pub trait DropTableExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(custom_output_path: &str) -> HashMap<K, DropTableExcelConfig>;
}

impl DropTableExcelConfigKeyed<u32> for DropTableExcelConfig {
    fn key(&self) -> u32 {
        self.id
    }

    fn load(custom_output_path: &str) -> HashMap<u32, DropTableExcelConfig> {
        let json = std::fs::read(&format!(
            "{custom_output_path}/DropTableExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<DropTableExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list.iter().map(|item| (item.key(), item.clone())).collect();
        data
    }
}
