pub mod ability;
pub mod config;
pub mod dynamic_float;
pub mod excel;
pub mod quest;
pub mod scene;
pub mod prop_type;

pub use dynamic_float::DynamicFloat;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        excel::load_all("../../assets/ExcelBinOutput").unwrap();
    }

    #[test]
    fn test_load_ability() {
        ability::load_ability_configs_from_bin("../../assets/BinOutput").unwrap();
    }
}
