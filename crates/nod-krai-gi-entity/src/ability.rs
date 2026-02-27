use bevy_ecs::component::Component;
use common::string_util;
use common::string_util::InternString;
use indexmap::IndexMap;
use nod_krai_gi_data::{config, excel::avatar_excel_config_collection};
use nod_krai_gi_proto::normal::{AbilityControlBlock, AbilityEmbryo};
use std::collections::HashMap;

#[derive(Component, Default)]
pub struct Ability {
    pub target_ability_map: IndexMap<InternString, AbilityData>,
}

pub struct AbilityData {
    pub ability_name_hash: u32,
    pub ability_override_name_hash: u32,
}

static TEMP_ABILITIES: std::sync::LazyLock<HashMap<u32, Vec<u32>>> =
    std::sync::LazyLock::new(|| {
        let mut temp_abilities = HashMap::new();

        temp_abilities.insert(
            //Aino
            10000121,
            vec![
                2948022120, 2261577000, 1899635397, 1433722468, 1928816237, 1167583242, 1541171842,
                3889765114, 3829597473, 1871806254, 4156234471, 1135998417, 3523369895, 1411398567,
                3598058965, 66762623, 2970456499, 3659950384, 3149354192, 3355785453, 1097416365,
                1198047057,
            ],
        );

        temp_abilities
    });

const COMMON_AVATAR_ABILITIES: [&str; 26] = [
    "Absorb_SealEcho_Bullet_01",
    "Absorb_SealEcho_Bullet_02",
    "Ability_Avatar_Dive_CrabShield",
    "Ability_Avatar_Dive_SealEcho",
    "Ability_Avatar_Dive_Team",
    "ActivityAbility_Absorb_Shoot",
    "Avatar_Absorb_SwordFishSlash",
    "Avatar_Absorb_TrackingMissile",
    "Avatar_ArkheGrade_CD_Controller",
    "Avatar_Attack_ReviveEnergy",
    "Avatar_Component_Initializer",
    "Avatar_DefaultAbility_AvartarInShaderChange",
    "Avatar_DefaultAbility_ManualClearSurfaceTypeInSpecificState",
    "Avatar_DefaultAbility_VisionReplaceDieInvincible",
    "Avatar_FallAnthem_Achievement_Listener",
    "Avatar_FluidAgitator",
    "Avatar_Freeze_Duration_Reducer",
    "Avatar_HDMesh_Controller",
    "Avatar_NyxState_Listener",
    "Avatar_PlayerBoy_DiveStamina_Reduction",
    "Avatar_PlayerGirl_DiveStamina_Reduction",
    "Avatar_SprintBS_Invincible",
    "Avatar_TriggerNyxInstant",
    "Avatar_Trampoline_Jump_Controller",
    "GrapplingHookSkill_Ability",
    "SceneAbility_DiveVolume",
];

const DEFAULT_TEAM_ABILITIES: [&str; 16] = [
    "DynamicAbility_ArcLight_Predicate",
    "DynamicAbility_ArcLight_Wathcer",
    "DynamicAbility_CommonArcLight_Invincible_V5_0",
    "DynamicAbility_Phlogiston",
    "DynamicAbility_NightsoulBlessing",
    "DynamicAbility_CitlaliQuest_PhlogistonInfinite",
    "SceneObj_Area_Nt_Property_Prop_YouLieDragon_Collision_DynamicAbility",
    "Team_TeamChargeMark",
    "TeamAbility_MoonPhase",
    "TeamAbility_Natsaurus_Preload",
    "TeamAbility_Natsaurus_Transfer_Vehicle_Skill",
    "TeamAbility_Natsaurus_Vehicle_PaintSeelie_AreaLimit",
    "TeamAbility_Natsaurus_Vehicle_State_Listener",
    "TeamAbility_NightsoulBurst",
    "TeamAbility_Reset_Crystal_Mark",
    "TeamAbility_Reset_MoonOvergrow",
];

impl Ability {
    fn add_common_avatar_abilities(ability_map: &mut IndexMap<InternString, AbilityData>) {
        for name in COMMON_AVATAR_ABILITIES.iter() {
            let data = AbilityData::new(name, "Default");
            ability_map.insert((*name).into(), data);
        }
    }

    fn process_open_configs(
        open_configs: Vec<InternString>,
        ability_map: &mut IndexMap<InternString, AbilityData>,
    ) {
        for open_config in open_configs {
            if let Some(talent_action) = config::get_avatar_talent_config(&open_config.into()) {
                for action in talent_action {
                    if let config::TalentAction::AddAbility { ability_name } = action {
                        let data = AbilityData::new(ability_name.as_str(), "Default");
                        ability_map.insert(*ability_name, data);
                    }
                }
            }
        }
    }

    pub fn new_for_avatar(id: u32, open_configs: Vec<InternString>) -> Self {
        let avatar_excel_config_collection_clone =
            std::sync::Arc::clone(avatar_excel_config_collection::get());
        let Some(avatar_config) = avatar_excel_config_collection_clone.get(&id) else {
            tracing::debug!("avatar config {} doesn't exist", id);
            return Self {
                target_ability_map: Default::default(),
            };
        };
        let avatar_name = avatar_config
            .icon_name
            .as_str()
            .replace("UI_AvatarIcon_", "");

        if let Some(config) = config::get_avatar_config(&avatar_name.into()) {
            let mut ability_map: IndexMap<InternString, AbilityData> = IndexMap::new();
            for ability in config.abilities.iter() {
                let data = AbilityData::new(
                    ability.ability_name.as_str(),
                    ability.ability_override.as_str(),
                );
                ability_map.insert(ability.ability_name, data);
            }

            Self::add_common_avatar_abilities(&mut ability_map);
            Self::process_open_configs(open_configs, &mut ability_map);

            Self {
                target_ability_map: ability_map,
            }
        } else {
            tracing::warn!("missing ConfigAvatar for {}", avatar_config.icon_name);
            let mut ability_map: IndexMap<InternString, AbilityData> = IndexMap::new();
            match TEMP_ABILITIES.get(&id) {
                None => {}
                Some(temp_abilities) => {
                    temp_abilities.iter().for_each(|ability| {
                        // 这里暂时使用空字符串作为键，因为我们没有实际的能力名称
                        ability_map.insert(
                            ability.to_string().into(),
                            AbilityData {
                                ability_name_hash: *ability,
                                ability_override_name_hash: string_util::get_string_hash("Default"),
                            },
                        );
                    });
                }
            }
            Self::add_common_avatar_abilities(&mut ability_map);
            Self::process_open_configs(open_configs, &mut ability_map);

            Self {
                target_ability_map: ability_map,
            }
        }
    }

    pub fn new_for_team() -> Self {
        let mut ability_map: IndexMap<InternString, AbilityData> = IndexMap::new();
        for name in DEFAULT_TEAM_ABILITIES.iter() {
            let data = AbilityData::new(name, "Default");
            ability_map.insert((*name).into(), data);
        }
        Self {
            target_ability_map: ability_map,
        }
    }

    pub fn new_for_gadget(json_name: &InternString) -> Self {
        if let Some(config) = config::get_gadget_config(json_name) {
            let mut ability_map: IndexMap<InternString, AbilityData> = IndexMap::new();
            for ability in config.abilities.iter() {
                let data = AbilityData::new(
                    ability.ability_name.as_str(),
                    ability.ability_override.as_str(),
                );
                ability_map.insert(ability.ability_name, data);
            }

            Self {
                target_ability_map: ability_map,
            }
        } else {
            tracing::warn!("missing GadgetConfig for {json_name}");
            let ability_map: IndexMap<InternString, AbilityData> = IndexMap::new();
            Self {
                target_ability_map: ability_map,
            }
        }
    }

    pub fn build_control_block(&self) -> AbilityControlBlock {
        AbilityControlBlock {
            ability_embryo_list: self
                .target_ability_map
                .iter()
                .enumerate()
                .map(|(idx, (_, data))| AbilityEmbryo {
                    ability_id: idx as u32 + 1,
                    ability_name_hash: data.ability_name_hash,
                    ability_override_name_hash: data.ability_override_name_hash,
                })
                .collect(),
        }
    }
}

impl AbilityData {
    pub fn new(name: &str, override_name: &str) -> Self {
        if override_name.is_empty() {
            Self {
                ability_name_hash: string_util::get_string_hash(name),
                ability_override_name_hash: string_util::get_string_hash("Default"),
            }
        } else {
            Self {
                ability_name_hash: string_util::get_string_hash(name),
                ability_override_name_hash: string_util::get_string_hash(override_name),
            }
        }
    }
}
