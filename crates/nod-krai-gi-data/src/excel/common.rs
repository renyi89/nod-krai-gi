use crate::prop_type::FightPropType;

#[derive(Debug, Default, Clone, serde::Deserialize)]
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

#[derive(Default, Clone, serde::Deserialize, Debug)]
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

#[derive(Default, Clone, serde::Deserialize, Debug)]
pub enum MaterialType {
    #[serde(alias = "MATERIAL_NONE", alias = "WEAPON_MATERIAL_NONE")]
    #[default]
    None = 0,
    #[serde(alias = "MATERIAL_FOOD")]
    Food = 1,
    #[serde(alias = "MATERIAL_QUEST")]
    Quest = 2,
    #[serde(alias = "MATERIAL_EXCHANGE")]
    Exchange = 4,
    #[serde(alias = "MATERIAL_CONSUME")]
    Consume = 5,
    #[serde(alias = "MATERIAL_EXP_FRUIT")]
    ExpFruit = 6,
    #[serde(alias = "MATERIAL_AVATAR")]
    Avatar = 7,
    #[serde(alias = "MATERIAL_ADSORBATE")]
    Adsorbate = 8,
    #[serde(alias = "MATERIAL_CRICKET")]
    Cricket = 9,
    #[serde(alias = "MATERIAL_ELEM_CRYSTAL")]
    ElemCrystal = 10,
    #[serde(alias = "MATERIAL_WEAPON_EXP_STONE")]
    WeaponExpStone = 11,
    #[serde(alias = "MATERIAL_CHEST")]
    Chest = 12,
    #[serde(alias = "MATERIAL_RELIQUARY_MATERIAL")]
    ReliquaryMaterial = 13,
    #[serde(alias = "MATERIAL_AVATAR_MATERIAL")]
    AvatarMaterial = 14,
    #[serde(alias = "MATERIAL_NOTICE_ADD_HP")]
    NoticeAddHp = 15,
    #[serde(alias = "MATERIAL_SEA_LAMP")]
    SeaLamp = 16,
    #[serde(alias = "MATERIAL_SELECTABLE_CHEST")]
    SelectableChest = 17,
    #[serde(alias = "MATERIAL_FLYCLOAK")]
    Flycloak = 18,
    #[serde(alias = "MATERIAL_NAMECARD")]
    Namecard = 19,
    #[serde(alias = "MATERIAL_TALENT")]
    Talent = 20,
    #[serde(alias = "MATERIAL_WIDGET")]
    Widget = 21,
    #[serde(alias = "MATERIAL_CHEST_BATCH_USE")]
    ChestBatchUse = 22,
    #[serde(alias = "MATERIAL_FAKE_ABSORBATE")]
    FakeAbsorbate = 23,
    #[serde(alias = "MATERIAL_CONSUME_BATCH_USE")]
    ConsumeBatchUse = 24,
    #[serde(alias = "MATERIAL_WOOD")]
    Wood = 25,
    #[serde(alias = "MATERIAL_FURNITURE_FORMULA")]
    FurnitureFormula = 27,
    #[serde(alias = "MATERIAL_CHANNELLER_SLAB_BUFF")]
    ChannellerSlabBuff = 28,
    #[serde(alias = "MATERIAL_FURNITURE_SUITE_FORMULA")]
    FurnitureSuiteFormula = 29,
    #[serde(alias = "MATERIAL_COSTUME")]
    Costume = 30,
    #[serde(alias = "MATERIAL_HOME_SEED")]
    HomeSeed = 31,
    #[serde(alias = "MATERIAL_FISH_BAIT")]
    FishBait = 32,
    #[serde(alias = "MATERIAL_FISH_ROD")]
    FishRod = 33,
    #[serde(alias = "MATERIAL_SUMO_BUFF")]
    SumoBuff = 34,
    #[serde(alias = "MATERIAL_FIREWORKS")]
    Fireworks = 35,
    #[serde(alias = "MATERIAL_BGM")]
    Bgm = 36,
    #[serde(alias = "MATERIAL_SPICE_FOOD")]
    SpiceFood = 37,
    #[serde(alias = "MATERIAL_ACTIVITY_ROBOT")]
    ActivityRobot = 38,
    #[serde(alias = "MATERIAL_ACTIVITY_GEAR")]
    ActivityGear = 39,
    #[serde(alias = "MATERIAL_ACTIVITY_JIGSAW")]
    ActivityJigsaw = 40,
    #[serde(alias = "MATERIAL_ARANARA")]
    Aranara = 41,
    #[serde(alias = "MATERIAL_DESHRET_MANUAL")]
    DeshretManual = 46,
}

#[derive(Default, Clone, serde::Deserialize, Debug)]
pub enum EquipType {
    #[serde(alias = "EQUIP_NONE")]
    #[default]
    None = 0,
    #[serde(alias = "EQUIP_BRACER")]
    Bracer = 1,
    #[serde(alias = "EQUIP_NECKLACE")]
    Necklace = 2,
    #[serde(alias = "EQUIP_SHOES")]
    Shoes = 3,
    #[serde(alias = "EQUIP_RING")]
    Ring = 4,
    #[serde(alias = "EQUIP_DRESS")]
    Dress = 5,
    #[serde(alias = "EQUIP_WEAPON")]
    Weapon = 6,
}

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
    pub grow_curve: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrowCurveInfo {
    pub r#type: String,
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
    pub prop_type: String,
    pub value: f32,
}