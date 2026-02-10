use crate::prop_type::FightPropType;
use common::string_util::InternString;

#[repr(u32)]
#[derive(Debug, Default, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum WeaponType {
    #[default]
    #[serde(alias = "WEAPON_NONE")]
    None = 0,
    #[serde(alias = "WEAPON_SWORD_ONE_HAND")]
    SwordOneHand = 1,
    #[serde(alias = "WEAPON_CROSSBOW")]
    Crossbow = 2,
    #[serde(alias = "WEAPON_STAFF")]
    Staff = 3,
    #[serde(alias = "WEAPON_DOUBLE_DAGGER")]
    DoubleDagger = 4,
    #[serde(alias = "WEAPON_KATANA")]
    Katana = 5,
    #[serde(alias = "WEAPON_SHURIKEN")]
    Shuriken = 6,
    #[serde(alias = "WEAPON_STICK")]
    Stick = 7,
    #[serde(alias = "WEAPON_SPEAR")]
    Spear = 8,
    #[serde(alias = "WEAPON_SHIELD_SMALL")]
    ShieldSmall = 9,
    #[serde(alias = "WEAPON_CATALYST")]
    Catalyst = 10,
    #[serde(alias = "WEAPON_CLAYMORE")]
    Claymore = 11,
    #[serde(alias = "WEAPON_BOW")]
    Bow = 12,
    #[serde(alias = "WEAPON_POLE")]
    Pole = 13,
}

#[repr(u32)]
#[derive(Default, Clone, serde::Deserialize, Debug, PartialEq, Eq)]
pub enum ItemType {
    #[default]
    #[serde(alias = "ITEM_NONE")]
    NONE = 0,
    #[serde(alias = "ITEM_VIRTUAL")]
    VIRTUAL = 1,
    #[serde(alias = "ITEM_MATERIAL")]
    MATERIAL = 2,
    #[serde(alias = "ITEM_RELIQUARY")]
    RELIQUARY = 3,
    #[serde(alias = "ITEM_WEAPON")]
    WEAPON = 4,
    #[serde(alias = "ITEM_DISPLAY")]
    DISPLAY = 5,
    #[serde(alias = "ITEM_FURNITURE")]
    FURNITURE = 6,
}

#[derive(Default, Clone, serde::Deserialize, Debug, PartialEq, Eq)]
pub enum MaterialType {
    #[serde(alias = "MATERIAL_FOOD")]
    Food,
    #[serde(alias = "MATERIAL_QUEST")]
    Quest,
    #[serde(alias = "MATERIAL_EXCHANGE")]
    Exchange,
    #[serde(alias = "MATERIAL_CONSUME")]
    Consume,
    #[serde(alias = "MATERIAL_EXP_FRUIT")]
    ExpFruit,
    #[serde(alias = "MATERIAL_AVATAR")]
    Avatar,
    #[serde(alias = "MATERIAL_ADSORBATE")]
    Adsorbate,
    #[serde(alias = "MATERIAL_CRICKET")]
    Cricket,
    #[serde(alias = "MATERIAL_ELEM_CRYSTAL")]
    ElemCrystal,
    #[serde(alias = "MATERIAL_WEAPON_EXP_STONE")]
    WeaponExpStone,
    #[serde(alias = "MATERIAL_CHEST")]
    Chest,
    #[serde(alias = "MATERIAL_RELIQUARY_MATERIAL")]
    ReliquaryMaterial,
    #[serde(alias = "MATERIAL_AVATAR_MATERIAL")]
    AvatarMaterial,
    #[serde(alias = "MATERIAL_NOTICE_ADD_HP")]
    NoticeAddHp,
    #[serde(alias = "MATERIAL_SEA_LAMP")]
    SeaLamp,
    #[serde(alias = "MATERIAL_SELECTABLE_CHEST")]
    SelectableChest,
    #[serde(alias = "MATERIAL_FLYCLOAK")]
    Flycloak,
    #[serde(alias = "MATERIAL_NAMECARD")]
    Namecard,
    #[serde(alias = "MATERIAL_TALENT")]
    Talent,
    #[serde(alias = "MATERIAL_WIDGET")]
    Widget,
    #[serde(alias = "MATERIAL_CHEST_BATCH_USE")]
    ChestBatchUse,
    #[serde(alias = "MATERIAL_FAKE_ABSORBATE")]
    FakeAbsorbate,
    #[serde(alias = "MATERIAL_CONSUME_BATCH_USE")]
    ConsumeBatchUse,
    #[serde(alias = "MATERIAL_WOOD")]
    Wood,
    #[serde(alias = "MATERIAL_FURNITURE_FORMULA")]
    FurnitureFormula,
    #[serde(alias = "MATERIAL_CHANNELLER_SLAB_BUFF")]
    ChannellerSlabBuff,
    #[serde(alias = "MATERIAL_FURNITURE_SUITE_FORMULA")]
    FurnitureSuiteFormula,
    #[serde(alias = "MATERIAL_COSTUME")]
    Costume,
    #[serde(alias = "MATERIAL_AVATAR_TRACE")]
    AvatarTrace,
    #[serde(alias = "MATERIAL_HOME_SEED")]
    HomeSeed,
    #[serde(alias = "MATERIAL_FISH_BAIT")]
    FishBait,
    #[serde(alias = "MATERIAL_FISH_ROD")]
    FishRod,
    #[serde(alias = "MATERIAL_SUMO_BUFF")]
    SumoBuff,
    #[serde(alias = "MATERIAL_FIREWORKS")]
    Fireworks,
    #[serde(alias = "MATERIAL_BGM")]
    Bgm,
    #[serde(alias = "MATERIAL_SPICE_FOOD")]
    SpiceFood,
    #[serde(alias = "MATERIAL_ARANARA")]
    Aranara,
    #[serde(alias = "MATERIAL_DESHRET_MANUAL")]
    DeshretManual = 46,
    #[serde(alias = "MATERIAL_FIRE_MASTER_AVATAR_TALENT_ITEM")]
    FireMasterAvatarTalentItem = 47,
    #[serde(alias = "MATERIAL_RENAME_ITEM")]
    RenameItem = 48,
    #[serde(alias = "MATERIAL_AVATAR_TALENT_MATERIAL")]
    AvatarTalentMaterial,
    #[serde(alias = "MATERIAL_NONE", alias = "WEAPON_MATERIAL_NONE")]
    #[serde(other)]
    #[default]
    None,
}

#[repr(u32)]
#[derive(Default, Clone, serde::Deserialize, Debug, PartialEq, Eq)]
pub enum EquipType {
    #[serde(alias = "EQUIP_NONE")]
    #[default]
    None = 0,
    #[serde(alias = "EQUIP_BRACER")]
    Bracer = 1,//生之花
    #[serde(alias = "EQUIP_NECKLACE")]
    Necklace = 2,//死之羽
    #[serde(alias = "EQUIP_SHOES")]
    Shoes = 3,//时之沙
    #[serde(alias = "EQUIP_RING")]
    Ring = 4,//空之杯
    #[serde(alias = "EQUIP_DRESS")]
    Dress = 5,//理之冠
    #[serde(alias = "EQUIP_WEAPON")]
    Weapon = 6,
}

#[repr(u32)]
#[derive(Default, Debug, serde::Deserialize, Clone, Copy)]
pub enum GrowCurveArith {
    #[default]
    None = 0,
    #[serde(alias = "ARITH_ADD")]
    Add = 1,
    #[serde(alias = "ARITH_MULTI")]
    Multi = 2,
    #[serde(alias = "ARITH_SUB")]
    Sub = 3,
    #[serde(alias = "ARITH_DIVIDE")]
    Divide = 4,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdCountConfig {
    pub id: u32,
    pub count: u32,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropValConfig {
    pub prop_type: FightPropType,
    pub value: f32,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropGrowCurve {
    pub r#type: FightPropType,
    pub grow_curve: InternString,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrowCurveInfo {
    pub r#type: InternString,
    pub arith: GrowCurveArith,
    pub value: f32,
}

impl GrowCurveInfo {
    pub fn apply(&self, val: f32) -> f32 {
        use GrowCurveArith::*;
        match self.arith {
            None => val,
            Add => val + self.value,
            Multi => val * self.value,
            Sub => val - self.value,
            Divide => val / self.value,
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddProp {
    pub prop_type: FightPropType,
    pub value: f32,
}
