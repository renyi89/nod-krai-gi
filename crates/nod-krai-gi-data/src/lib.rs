pub mod config;
pub mod excel;
pub mod prop_type;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        excel::load_all("../../assets/ExcelBinOutput").unwrap();
    }
}
