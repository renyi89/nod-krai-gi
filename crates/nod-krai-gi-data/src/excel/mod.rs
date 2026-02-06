pub mod common;

mod avatar_costume_excel_config;
mod avatar_curve_excel_config;
mod avatar_excel_config;
mod avatar_flycloak_excel_config;
mod avatar_promote_excel_config;
mod avatar_skill_depot_excel_config;
mod avatar_skill_excel_config;
mod avatar_talent_excel_config;
mod avatar_trace_effect_excel_config;
mod daily_dungeon_config;
mod dungeon_excel_config;
mod fetter_data_config;
mod gadget_excel_config;
mod map_layer_config;
mod map_layer_floor_config;
mod map_layer_group_config;
mod monster_curve_excel_config;
mod monster_excel_config;
mod open_state_config;

mod anecdote_excel_config;
mod proud_skill_excel_config;
mod scene_tag_config;
mod weapon_curve_excel_config;
mod weapon_excel_config;

pub use anecdote_excel_config::*;
pub use avatar_costume_excel_config::*;
pub use avatar_curve_excel_config::*;
pub use avatar_excel_config::*;
pub use avatar_flycloak_excel_config::*;
pub use avatar_promote_excel_config::*;
pub use avatar_skill_depot_excel_config::*;
pub use avatar_skill_excel_config::*;
pub use avatar_talent_excel_config::*;
pub use avatar_trace_effect_excel_config::*;
pub use daily_dungeon_config::*;
pub use dungeon_excel_config::*;
pub use fetter_data_config::*;
pub use gadget_excel_config::*;
pub use map_layer_config::*;
pub use map_layer_floor_config::*;
pub use map_layer_group_config::*;
pub use monster_curve_excel_config::*;
pub use monster_excel_config::*;
pub use open_state_config::*;
pub use proud_skill_excel_config::*;
pub use scene_tag_config::*;
pub use weapon_curve_excel_config::*;
pub use weapon_excel_config::*;

use paste::paste;

macro_rules! excel_loader {
    ($($name:ident;)*) => {
        $(paste! {
            pub mod [<$name:snake _collection>] {
                pub const PATH: &str = stringify!([<$name Data>]);

                static DATA: ::std::sync::OnceLock<std::sync::Arc<std::collections::HashMap<u32,super::$name>>> = ::std::sync::OnceLock::new();
                pub fn load_from_json(excel_bin_output_path: &'static str) -> std::io::Result<()> {
                        use crate::excel::[<$name:snake>]::*;
                        let data = [<$name>]::load(excel_bin_output_path);
                        let _ = DATA.set(std::sync::Arc::new(data));
                        Ok(())
                }

                pub fn get() -> &'static std::sync::Arc<std::collections::HashMap<u32,super::$name>> {
                    DATA.get().unwrap()
                }

            }
        })*

        pub fn load_all(excel_bin_output_path: &'static str) -> ::std::io::Result<()> {
            $(paste!{
                [<$name:snake _collection>]::load_from_json(excel_bin_output_path)?;
            })*

            Ok(())
        }

    };
}

excel_loader! {
    AvatarCostumeExcelConfig;
    AvatarCurveExcelConfig;
    AvatarExcelConfig;
    AvatarFlycloakExcelConfig;
    AvatarPromoteExcelConfig;
    AvatarSkillDepotExcelConfig;
    AvatarSkillExcelConfig;
    AvatarTalentExcelConfig;
    AvatarTraceEffectExcelConfig;
    DailyDungeonConfig;
    DungeonExcelConfig;
    FetterDataConfig;
    GadgetExcelConfig;
    MapLayerConfig;
    MapLayerFloorConfig;
    MapLayerGroupConfig;
    MonsterCurveExcelConfig;
    MonsterExcelConfig;
    OpenStateConfig;
    ProudSkillExcelConfig;
    SceneTagConfig;
    WeaponCurveExcelConfig;
    WeaponExcelConfig;
    AnecdoteExcelConfig;
}
