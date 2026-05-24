mod combined_drop;
mod drop_sub_table_excel_config;
mod drop_table_excel_config;
mod gacha_banner;
mod gadget_mapping;
mod quest_encryption_key;

pub use combined_drop::*;
pub use drop_sub_table_excel_config::*;
pub use drop_table_excel_config::*;
pub use gacha_banner::*;
pub use gadget_mapping::*;
pub use quest_encryption_key::*;

use paste::paste;

macro_rules! custom_loader {
    ($($name:ident;)*) => {
        $(paste! {
            pub mod [<$name:snake _collection>] {
                pub const PATH: &str = stringify!([<$name Data>]);

                static DATA: ::std::sync::OnceLock<std::sync::Arc<std::collections::HashMap<u32,super::$name>>> = ::std::sync::OnceLock::new();
                pub fn load_from_json(custom_output_path: &'static str) -> std::io::Result<()> {
                        use crate::custom::[<$name:snake>]::*;
                        let data = [<$name>]::load(custom_output_path);
                        let _ = DATA.set(std::sync::Arc::new(data));
                        Ok(())
                }

                pub fn get() -> &'static std::sync::Arc<std::collections::HashMap<u32,super::$name>> {
                    DATA.get().unwrap()
                }

            }
        })*

        pub fn load_all(custom_output_path: &'static str) -> ::std::io::Result<()> {
            $(paste!{
                [<$name:snake _collection>]::load_from_json(custom_output_path)?;
            })*

            Ok(())
        }

    };
}

custom_loader! {
    GachaBanner;
    DropSubTableExcelConfig;
    DropTableExcelConfig;
    QuestEncryptionKey;
}
