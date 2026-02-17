use indexmap::IndexMap;
use std::collections::{HashMap, HashSet};

use crate::fight_props;
use bevy_ecs::prelude::*;
use common::string_util::InternString;
use nod_krai_gi_data::ability::{get_ability_data, AbilityData, AbilityModifier};
use nod_krai_gi_data::excel::common::EquipType;
use nod_krai_gi_data::excel::{
    reliquary_affix_excel_config_collection, reliquary_excel_config_collection,
    reliquary_level_excel_config_collection, reliquary_main_prop_excel_config_collection,
    weapon_excel_config_collection,
};
use nod_krai_gi_data::{
    excel::{
        avatar_curve_excel_config_collection, avatar_promote_excel_config_collection,
        common::PropGrowCurve, monster_curve_excel_config_collection,
        weapon_curve_excel_config_collection, AvatarExcelConfig,
    },
    prop_type::FightPropType,
};
use nod_krai_gi_proto::normal::ProtEntityType;
use nod_krai_gi_proto::server_only::{equip_bin, item_bin, AvatarBin};

#[derive(Component)]
pub struct BlockId(pub u32);
#[derive(Component)]
pub struct GroupId(pub u32);
#[derive(Component)]
pub struct ConfigId(pub u32);

#[derive(Component)]
pub struct Level(pub u32);

#[derive(Component)]
pub struct CoreProudSkillLevel(pub u32);

#[derive(Component)]
pub struct Guid(pub u64);

#[derive(Component)]
pub struct OwnerPlayerUID(pub u32);

#[derive(Component)]
pub struct ProtocolEntityID(pub u32);

#[derive(Component)]
pub struct OwnerProtocolEntityID(pub Option<u32>);

#[derive(Component)]
pub struct FightProperties(pub HashMap<FightPropType, f32>, pub HashSet<FightPropType>);

#[derive(Component, Default)]
pub struct GlobalAbilityValues(pub HashMap<InternString, f32>);

#[derive(Component, Default)]
pub struct InstancedAbilities {
    pub list: Vec<InstancedAbility>,
    by_id: HashMap<u32, usize>,
    by_name: HashMap<InternString, usize>,
}

#[derive(Component, Default)]
pub struct InstancedModifiers(pub HashMap<u32, AbilityModifierController>);

#[derive(Default, Clone)]
pub struct InstancedAbility {
    pub instanced_ability_id: Option<u32>,
    pub ability_data: Option<&'static AbilityData>,
    pub modifiers: IndexMap<InternString, &'static AbilityModifier>,
    pub ability_specials: HashMap<InternString, f32>,
}

impl InstancedAbilities {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            by_id: HashMap::new(),
            by_name: HashMap::new(),
        }
    }

    #[inline]
    fn check_len(&self) -> bool {
        if self.list.len() > 100 {
            tracing::warn!("InstancedAbilities len > 100");
            return false;
        }
        true
    }

    pub fn add_or_replace_by_instanced_ability_id(
        &mut self,
        instanced_ability_id: u32,
        ability_name: InternString,
    ) -> Option<(u32, &InstancedAbility)> {
        if !self.check_len() {
            return None;
        }

        let ability_data = get_ability_data(&ability_name);

        match self.by_id.get(&instanced_ability_id).copied() {
            Some(index) => {
                let inst = &mut self.list[index];
                inst.ability_data = ability_data;
                Some((index as u32, inst))
            }

            None => {
                let inst = InstancedAbility::new(Some(instanced_ability_id), ability_data);

                let index = self.list.len();
                self.list.push(inst);

                self.by_id.insert(instanced_ability_id, index);
                if let Some(data) = &self.list[index].ability_data {
                    self.by_name.insert(data.ability_name.clone(), index);
                }

                Some((index as u32, &self.list[index]))
            }
        }
    }

    pub fn find_or_add_by_ability_name(
        &mut self,
        ability_name: InternString,
        instanced_ability_id: u32,
    ) -> Option<(u32, &InstancedAbility)> {
        if !self.check_len() {
            return None;
        }

        if let Some(&index) = self.by_name.get(&ability_name) {
            return Some((index as u32, &self.list[index]));
        }

        self.add_or_replace_by_instanced_ability_id(instanced_ability_id, ability_name)
    }

    pub fn find_by_instanced_ability_id_mut(
        &mut self,
        instanced_ability_id: u32,
    ) -> Option<(u32, &mut InstancedAbility)> {
        if !self.check_len() {
            return None;
        }

        match self.by_id.get(&instanced_ability_id).copied() {
            Some(index) => Some((index as u32, &mut self.list[index])),

            None => {
                let inst = InstancedAbility::new(Some(instanced_ability_id), None);

                let index = self.list.len();
                self.list.push(inst);

                self.by_id.insert(instanced_ability_id, index);

                Some((index as u32, &mut self.list[index]))
            }
        }
    }

    pub fn find_by_instanced_ability_id(
        &self,
        instanced_ability_id: u32,
    ) -> Option<(u32, &InstancedAbility)> {
        self.by_id
            .get(&instanced_ability_id)
            .map(|&index| (index as u32, &self.list[index]))
    }
}

impl InstancedAbility {
    pub fn new(
        instanced_ability_id: Option<u32>,
        ability_data: Option<&'static AbilityData>,
    ) -> Self {
        Self {
            instanced_ability_id,
            ability_data: ability_data,
            modifiers: IndexMap::new(),
            ability_specials: ability_data
                .and_then(|ability_data| Some(ability_data.ability_specials.clone()))
                .unwrap_or_default(),
        }
    }
}

pub struct AbilityModifierController {
    pub target_entity: Option<Entity>,
    pub ability_index: Option<u32>,
    pub modifier_data: Option<&'static AbilityModifier>,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum LifeState {
    Alive = 1,
    Dead = 2,
}

#[derive(Component)]
pub struct Visible;

#[derive(Component)]
pub struct ToBeRemovedMarker;

#[derive(Component)]
pub struct GadgetID(pub u32);

#[derive(Resource, Default)]
pub struct EntityCounter(u32);

#[derive(Resource, Default)]
pub struct EntityById(pub HashMap<u32, Entity>);

impl EntityCounter {
    pub fn inc(&mut self) -> u32 {
        self.0 += 1;
        self.0
    }
}

pub enum GrowCurveConfigType {
    Avatar,
    Monster,
}

impl FightProperties {
    pub fn apply_grow_curve(
        &mut self,
        level: u32,
        prop_grow_curve: &PropGrowCurve,
        config_type: GrowCurveConfigType,
    ) {
        let avatar_curve_excel_config_collection_clone =
            std::sync::Arc::clone(avatar_curve_excel_config_collection::get());
        let monster_curve_excel_config_collection_clone =
            std::sync::Arc::clone(monster_curve_excel_config_collection::get());
        let curve_info = match config_type {
            GrowCurveConfigType::Avatar => {
                let Some(avatar_curve_config) =
                    avatar_curve_excel_config_collection_clone.get(&level)
                else {
                    tracing::debug!("avatar curve config {} doesn't exist", level);
                    return;
                };
                avatar_curve_config
                    .curve_infos
                    .iter()
                    .find(|c| c.r#type == prop_grow_curve.grow_curve)
            }
            GrowCurveConfigType::Monster => {
                let Some(monster_curve) = monster_curve_excel_config_collection_clone.get(&level)
                else {
                    tracing::debug!("monster curve config {} doesn't exist", level);
                    return;
                };
                monster_curve
                    .curve_infos
                    .iter()
                    .find(|c| c.r#type == prop_grow_curve.grow_curve)
            }
        };

        if let Some(curve_info) = curve_info {
            let val = curve_info.apply(self.get_property(prop_grow_curve.r#type));
            self.set_property(prop_grow_curve.r#type, val);
        }
    }

    pub fn apply_base_values(&mut self) {
        use FightPropType::*;

        let base_hp = self.get_property(FIGHT_PROP_BASE_HP);
        let base_attack = self.get_property(FIGHT_PROP_BASE_ATTACK);
        let base_defense = self.get_property(FIGHT_PROP_BASE_DEFENSE);

        self.set_property(
            FIGHT_PROP_MAX_HP,
            base_hp
                + base_hp * self.get_property(FIGHT_PROP_HP_PERCENT)
                + self.get_property(FIGHT_PROP_HP),
        );

        self.set_property(FIGHT_PROP_CUR_HP, self.get_property(FIGHT_PROP_MAX_HP));

        self.set_property(
            FIGHT_PROP_CUR_ATTACK,
            base_attack
                + base_attack * self.get_property(FIGHT_PROP_ATTACK_PERCENT)
                + self.get_property(FIGHT_PROP_ATTACK),
        );

        self.set_property(
            FIGHT_PROP_CUR_DEFENSE,
            base_defense
                + base_defense * self.get_property(FIGHT_PROP_DEFENSE_PERCENT)
                + self.get_property(FIGHT_PROP_DEFENSE),
        );
    }

    pub fn get_property(&self, ty: FightPropType) -> f32 {
        self.0.get(&ty).copied().unwrap_or_default()
    }

    pub fn set_property(&mut self, ty: FightPropType, val: f32) {
        self.0.insert(ty, val);
        self.1.insert(ty);
    }

    pub fn flush_property(&mut self) {
        self.1.extend(self.0.keys());
    }

    pub fn change_property(&mut self, ty: FightPropType, delta: f32) {
        *self.0.entry(ty).or_default() += delta;
        self.1.insert(ty);
    }

    pub fn change_cur_hp(&mut self, delta: f32) {
        self.change_property(FightPropType::FIGHT_PROP_CUR_HP, delta);
        self.clamp_property(
            FightPropType::FIGHT_PROP_CUR_HP,
            FightPropType::FIGHT_PROP_MAX_HP,
        );
    }

    pub fn clamp_property(&mut self, ty: FightPropType, max_ty: FightPropType) {
        let max = self.0.get(&max_ty).copied().unwrap_or_default();
        let cur = self.0.entry(ty).or_default();

        *cur = cur.clamp(0.0, max);
    }
}

impl ProtocolEntityID {
    pub fn entity_type(&self) -> ProtEntityType {
        ProtEntityType::try_from((self.0 >> 22) as i32).unwrap_or_default()
    }
}

pub fn create_fight_props(
    config: &AvatarExcelConfig,
    level: u32,
    break_level: u32,
) -> FightProperties {
    let mut props = fight_props! {
        FIGHT_PROP_BASE_HP: config.hp_base,
        FIGHT_PROP_HP: config.hp_base,
        FIGHT_PROP_HP_PERCENT: 0.0,
        FIGHT_PROP_CUR_HP: config.hp_base,
        FIGHT_PROP_MAX_HP: config.hp_base,

        FIGHT_PROP_BASE_ATTACK: config.attack_base,
        FIGHT_PROP_ATTACK: config.attack_base,
        FIGHT_PROP_ATTACK_PERCENT: 0.0,
        FIGHT_PROP_CUR_ATTACK: config.attack_base,

        FIGHT_PROP_BASE_DEFENSE: config.defense_base,
        FIGHT_PROP_DEFENSE: config.defense_base,
        FIGHT_PROP_DEFENSE_PERCENT: 0.0,
        FIGHT_PROP_CUR_DEFENSE: config.defense_base,

        FIGHT_PROP_ELEMENT_MASTERY: config.element_mastery,
        FIGHT_PROP_CRITICAL: config.critical,
        FIGHT_PROP_CRITICAL_HURT: config.critical_hurt,
        FIGHT_PROP_CUR_WIND_ENERGY: 100.0,
        FIGHT_PROP_CUR_GRASS_ENERGY: 100.0,
        FIGHT_PROP_CUR_ICE_ENERGY: 100.0,
        FIGHT_PROP_CUR_FIRE_ENERGY: 100.0,
        FIGHT_PROP_CUR_ELEC_ENERGY: 100.0,
        FIGHT_PROP_CUR_WATER_ENERGY: 100.0,
        FIGHT_PROP_CUR_ROCK_ENERGY: 100.0,
        FIGHT_PROP_CUR_SPECIAL_ENERGY: 100.0,
        FIGHT_PROP_START_SPECIAL_ENERGY: 100.0,
        FIGHT_PROP_MAX_WIND_ENERGY: 100.0,
        FIGHT_PROP_MAX_GRASS_ENERGY: 100.0,
        FIGHT_PROP_MAX_ICE_ENERGY: 100.0,
        FIGHT_PROP_MAX_FIRE_ENERGY: 100.0,
        FIGHT_PROP_MAX_ELEC_ENERGY: 100.0,
        FIGHT_PROP_MAX_WATER_ENERGY: 100.0,
        FIGHT_PROP_MAX_ROCK_ENERGY: 100.0,
        FIGHT_PROP_MAX_SPECIAL_ENERGY: 100.0,

        FIGHT_PROP_CHARGE_EFFICIENCY: 0.0,
        FIGHT_PROP_HEAL_ADD: 0.0,
        FIGHT_PROP_HEALED_ADD: 0.0,
        FIGHT_PROP_CRITICAL: 0.0,
        FIGHT_PROP_CRITICAL_HURT: 0.0,

        FIGHT_PROP_ADD_HURT: 0.0,
        FIGHT_PROP_PHYSICAL_ADD_HURT: 0.0,
        FIGHT_PROP_GRASS_ADD_HURT: 0.0,
        FIGHT_PROP_ROCK_ADD_HURT: 0.0,
        FIGHT_PROP_WIND_ADD_HURT: 0.0,
        FIGHT_PROP_WATER_ADD_HURT: 0.0,
        FIGHT_PROP_ICE_ADD_HURT: 0.0,
        FIGHT_PROP_ELEC_ADD_HURT: 0.0,
        FIGHT_PROP_FIRE_ADD_HURT: 0.0,

        FIGHT_PROP_SUB_HURT: 0.0,
        FIGHT_PROP_WIND_SUB_HURT: 0.0,
        FIGHT_PROP_ICE_SUB_HURT: 0.0,
        FIGHT_PROP_GRASS_SUB_HURT: 0.0,
        FIGHT_PROP_ELEC_SUB_HURT: 0.0,
        FIGHT_PROP_ROCK_SUB_HURT: 0.0,
        FIGHT_PROP_WATER_SUB_HURT: 0.0,
        FIGHT_PROP_FIRE_SUB_HURT: 0.0,
        FIGHT_PROP_PHYSICAL_SUB_HURT: 0.0,

        FIGHT_PROP_SKILL_CD_MINUS_RATIO: 0.0,
        FIGHT_PROP_SHIELD_COST_MINUS_RATIO: 0.0,
        FIGHT_PROP_DEFENCE_IGNORE_RATIO: 0.0,

        FIGHT_PROP_CUR_HP_DEBTS: 0.0,
        FIGHT_PROP_CUR_HP_PAID_DEBTS: 0.0

    };

    for prop_grow_curve in config.prop_grow_curves.iter() {
        props.apply_grow_curve(level, prop_grow_curve, GrowCurveConfigType::Avatar);
    }

    let avatar_promote_excel_config_collection_clone =
        std::sync::Arc::clone(avatar_promote_excel_config_collection::get());

    if let Some(promote_config) = avatar_promote_excel_config_collection_clone
        .get(&(config.avatar_promote_id << 8 + break_level))
    {
        if promote_config.promote_level == break_level {
            for add_prop in promote_config.add_props.iter() {
                props.change_property(add_prop.prop_type, add_prop.value);
            }
        }
    }

    props.apply_base_values();
    props
}

pub fn create_fight_props_with_equip(
    avatar_bin: &AvatarBin,
    avatar_config: &AvatarExcelConfig,
) -> FightProperties {
    let mut props = create_fight_props(avatar_config, avatar_bin.level, avatar_bin.promote_level);
    add_fight_props_from_weapon(&mut props, avatar_bin);
    add_fight_props_from_reliquary(&mut props, avatar_bin);
    props.apply_base_values();
    props.flush_property();
    props
}

pub fn add_fight_props_from_weapon(props: &mut FightProperties, avatar_bin: &AvatarBin) {
    let mut is_add_weapon = false;
    for (equip_type_id, item) in avatar_bin.equip_map.iter() {
        let equip_type = EquipType::from(*equip_type_id);
        if equip_type != EquipType::Weapon {
            continue;
        }
        let Some(item_bin::Detail::Equip(ref equip)) = item.detail else {
            tracing::warn!("item with guid {} is not equip", item.guid);
            break;
        };
        let Some(equip_bin::Detail::Weapon(ref weapon)) = equip.detail else {
            tracing::warn!("equip with guid {} is not weapon", item.guid);
            break;
        };

        let weapon_excel_config_collection_clone =
            std::sync::Arc::clone(weapon_excel_config_collection::get());

        let weapon_curve_excel_config_collection_clone =
            std::sync::Arc::clone(weapon_curve_excel_config_collection::get());

        if let Some(weapon_curve_config) =
            weapon_curve_excel_config_collection_clone.get(&weapon.level)
        {
            let Some(weapon_config) = weapon_excel_config_collection_clone.get(&item.item_id)
            else {
                tracing::debug!("weapon config {} doesn't exist", item.item_id);
                continue;
            };

            for weapon_property in weapon_config.weapon_prop.iter() {
                if let Some(curve_info) = weapon_curve_config
                    .curve_infos
                    .iter()
                    .find(|c| c.r#type == weapon_property.r#type)
                {
                    is_add_weapon = true;
                    let val = curve_info.apply(weapon_property.init_value);
                    props.change_property(weapon_property.prop_type, val);
                }
            }
        }
    }

    if !is_add_weapon {
        tracing::warn!(
            "avatar no weapon avatar_bin.equip_map:{:#?}",
            avatar_bin.equip_map
        );
    }
}

pub fn add_fight_props_from_reliquary(props: &mut FightProperties, avatar_bin: &AvatarBin) {
    let reliquary_excel_config_collection_clone =
        std::sync::Arc::clone(reliquary_excel_config_collection::get());

    let reliquary_main_prop_excel_config_collection_clone =
        std::sync::Arc::clone(reliquary_main_prop_excel_config_collection::get());

    let reliquary_level_excel_config_collection_clone =
        std::sync::Arc::clone(reliquary_level_excel_config_collection::get());

    let reliquary_affix_excel_config_collection_clone =
        std::sync::Arc::clone(reliquary_affix_excel_config_collection::get());

    let mut append_prop_id_list = vec![];

    for (equip_type_id, item) in avatar_bin.equip_map.iter() {
        let equip_type = EquipType::from(*equip_type_id);
        if equip_type == EquipType::None || equip_type == EquipType::Weapon {
            continue;
        }
        let Some(item_bin::Detail::Equip(ref equip)) = item.detail else {
            tracing::warn!("item with guid {} is not equip", item.guid);
            break;
        };
        let Some(equip_bin::Detail::Reliquary(ref reliquary)) = equip.detail else {
            tracing::warn!("equip with guid {} is not reliquary", item.guid);
            break;
        };

        append_prop_id_list.extend(reliquary.append_prop_id_list.clone());

        let Some(reliquary_config) = reliquary_excel_config_collection_clone.get(&item.item_id)
        else {
            tracing::debug!("reliquary config {} doesn't exist", item.item_id);
            break;
        };

        let Some(reliquary_main_prop_config) =
            reliquary_main_prop_excel_config_collection_clone.get(&reliquary.main_prop_id)
        else {
            tracing::debug!(
                "reliquary main prop config {} doesn't exist",
                reliquary.main_prop_id
            );
            break;
        };

        let Some(reliquary_level_config) = reliquary_level_excel_config_collection_clone
            .get(&(reliquary_config.rank << 8 + reliquary.level))
        else {
            tracing::debug!(
                "reliquary level config {} {} doesn't exist",
                reliquary_config.rank,
                reliquary.level
            );
            break;
        };

        for add_prop in reliquary_level_config.add_props.iter() {
            if reliquary_main_prop_config.prop_type == add_prop.prop_type {
                props.change_property(add_prop.prop_type, add_prop.value);
            }
        }
    }

    for append_prop_id in append_prop_id_list {
        let Some(reliquary_affix_config) =
            reliquary_affix_excel_config_collection_clone.get(&append_prop_id)
        else {
            tracing::debug!("reliquary affix config {} doesn't exist", append_prop_id);
            break;
        };

        props.change_property(
            reliquary_affix_config.prop_type,
            reliquary_affix_config.prop_value,
        );
    }
}
