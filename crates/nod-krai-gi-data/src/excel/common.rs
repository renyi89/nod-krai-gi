use crate::prop_type::FightPropType;
use common::string_util::InternString;

pub trait LuaEnum {
    fn variants() -> &'static [(&'static str, u32)];
}

#[macro_export]
macro_rules! lua_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $(
                alias($alias:literal) $variant:ident = $value:expr,
            )*
        }
    ) => {
        $(#[$meta])*
        #[repr(u32)]
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        pub enum $name {
            $(
                $variant = $value,
            )*
        }

        impl Default for $name {
            fn default() -> Self {
                Self::from(0)
            }
        }

        impl From<u32> for $name {
            fn from(v: u32) -> Self {
                match v {
                    $(
                        $value => $name::$variant,
                    )*
                    _ => Self::default(),
                }
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use serde::de::{Error, Unexpected};

                #[derive(serde::Deserialize)]
                #[serde(untagged)]
                enum Repr {
                    Num(u32),
                    Str(String),
                }

                match Repr::deserialize(deserializer)? {
                    Repr::Num(n) => std::result::Result::Ok(Self::from(n)),
                    Repr::Str(s) => {
                        match s.as_str() {
                            $(
                                $alias | stringify!($variant) => std::result::Result::Ok(Self::$variant),
                            )*
                            _ => std::result::Result::Err(D::Error::invalid_value(
                                Unexpected::Str(&s),
                                &"valid alias or variant name",
                            )),
                        }
                    }
                }
            }
        }

        impl LuaEnum for $name {
            fn variants() -> &'static [(&'static str, u32)] {
                &[
                    $(
                        ($alias, $value),
                    )*
                ]
            }
        }
    };
}

lua_enum! {
    pub enum EntityType {
        alias("NONE")
        None = 0,
        alias("AVATAR")
        Avatar = 1,
        alias("MONSTER")
        Monster = 2,
        alias("BULLET")
        Bullet = 3,
        alias("ATTACK_PHYISICAL_UNIT")
        AttackPhyisicalUnit = 4,
        alias("AOE")
        AOE = 5,
        alias("CAMERA")
        Camera = 6,
        alias("ENVIRO_AREA")
        EnviroArea = 7,
        alias("EQUIP")
        Equip = 8,
        alias("MONSTER_EQUIP")
        MonsterEquip = 9,
        alias("GRASS")
        Grass = 10,
        alias("LEVEL")
        Level = 11,
        alias("NPC")
        NPC = 12,
        alias("TRANS_POINT_FIRST")
        TransPointFirst = 13,
        alias("TRANS_POINT_FIRST_GADGET")
        TransPointFirstGadget = 14,
        alias("TRANS_POINT_SECOND")
        TransPointSecond = 15,
        alias("TRANS_POINT_SECOND_GADGET")
        TransPointSecondGadget = 16,
        alias("DROP_ITEM")
        DropItem = 17,
        alias("FIELD")
        Field = 18,
        alias("GADGET")
        Gadget = 19,
        alias("WATER")
        Water = 20,
        alias("GATHER_POINT")
        GatherPoint = 21,
        alias("GATHER_OBJECT")
        GatherObject = 22,
        alias("AIRFLOW_FIELD")
        AirflowField = 23,
        alias("SPEEDUP_FIELD")
        SpeedupField = 24,
        alias("GEAR")
        Gear = 25,
        alias("CHEST")
        Chest = 26,
        alias("ENERGY_BALL")
        EnergyBall = 27,
        alias("ELEM_CRYSTAL")
        ElemCrystal = 28,
        alias("TIMELINE")
        Timeline = 29,
        alias("WORKTOP")
        Worktop = 30,
        alias("TEAM")
        Team = 31,
        alias("PLATFORM")
        Platform = 32,
        alias("AMBER_WIND")
        AmberWind = 33,
        alias("ENV_ANIMAL")
        EnvAnimal = 34,
        alias("SEAL_GADGET")
        SealGadget = 35,
        alias("TREE")
        Tree = 36,
        alias("BUSH")
        Bush = 37,
        alias("QUEST_GADGET")
        QuestGadget = 38,
        alias("LIGHTNING")
        Lightning = 39,
        alias("REWARD_POINT")
        RewardPoint = 40,
        alias("REWARD_STATUE")
        RewardStatue = 41,
        alias("MP_LEVEL")
        MPLevel = 42,
        alias("WIND_SEED")
        WindSeed = 43,
        alias("MP_PLAY_REWARD_POINT")
        MpPlayRewardPoint = 44,
        alias("VIEW_POINT")
        ViewPoint = 45,
        alias("REMOTE_AVATAR")
        RemoteAvatar = 46,
        alias("GENERAL_REWARD_POINT")
        GeneralRewardPoint = 47,
        alias("PLAY_TEAM")
        PlayTeam = 48,
        alias("OFFERING_GADGET")
        OfferingGadget = 49,
        alias("EYE_POINT")
        EyePoint = 50,
        alias("MIRACLE_RING")
        MiracleRing = 51,
        alias("FOUNDATION")
        Foundation = 52,
        alias("WIDGET_GADGET")
        WidgetGadget = 53,
        alias("VEHICLE")
        Vehicle = 54,
        alias("SUB_EQUIP")
        SubEquip = 55,
        alias("FISH_ROD")
        FishRod = 56,
        alias("CUSTOM_TILE")
        CustomTile = 57,
        alias("FISH_POOL")
        FishPool = 58,
        alias("CUSTOM_GADGET")
        CustomGadget = 59,
        alias("BLACK_MUD")
        BlackMud = 60,
        alias("ROGUELIKE_OPERATOR_GADGET")
        RoguelikeOperatorGadget = 61,
        alias("NIGHT_CROW_GADGET")
        NightCrowGadget = 62,
        alias("PROJECTOR")
        Projector = 63,
        alias("SCREEN")
        Screen = 64,
        alias("ECHO_SHELL")
        EchoShell = 65,
        alias("UI_INTERACT_GADGET")
        UIInteractGadget = 66,
        alias("CURVE_MOVE_GADGET")
        CurveMoveGadget = 67,
        alias("COIN_COLLECT_LEVEL_GADGET")
        CoinCollectLevelGadget = 68,
        alias("UGC_TOWER_LEVEL_UP_GADGET")
        UgcTowerLevelUpGadget = 69,
        alias("JOURNEY_GEAR_OPERATOR_GADGET")
        JourneyGearOperatorGadget = 70,
        alias("UGC_SPECIAL_GADGET")
        UgcSpecialGadget = 71,
        alias("DESHRET_OBELISK_GADGET")
        DeshretObeliskGadget = 72,
        alias("REGION")
        Region = 98,
        alias("PLACE_HOLDER")
        PlaceHolder = 99,
    }
}

lua_enum! {
    pub enum QuestState {
        alias("NONE")
        None = 0,
        alias("UNSTARTED")
        Unstarted = 1,
        alias("UNFINISHED")
        Unfinished = 2,
        alias("FINISHED")
        Finished = 3,
        alias("FAILED")
        Failed = 4,
    }
}

lua_enum! {
    pub enum VisionLevelType {
        alias("VISION_LEVEL_NORMAL")
        Normal = 0,
        alias("VISION_LEVEL_LITTLE_REMOTE")
        LittleRemote = 1,
        alias("VISION_LEVEL_REMOTE")
        Remote = 2,
        alias("VISION_LEVEL_SUPER")
        Super = 3,
        alias("VISION_LEVEL_NEARBY")
        Nearby = 4,
        alias("VISION_LEVEL_SUPER_NEARBY")
        SuperNearby = 5,
    }
}

#[repr(u32)]
#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
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
#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
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
#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum EquipType {
    #[serde(alias = "EQUIP_NONE")]
    #[default]
    None = 0,
    #[serde(alias = "EQUIP_BRACER")]
    Bracer = 1, //生之花
    #[serde(alias = "EQUIP_NECKLACE")]
    Necklace = 2, //死之羽
    #[serde(alias = "EQUIP_SHOES")]
    Shoes = 3, //时之沙
    #[serde(alias = "EQUIP_RING")]
    Ring = 4, //空之杯
    #[serde(alias = "EQUIP_DRESS")]
    Dress = 5, //理之冠
    #[serde(alias = "EQUIP_WEAPON")]
    Weapon = 6,
}

impl From<u32> for EquipType {
    fn from(value: u32) -> Self {
        match value {
            1 => EquipType::Bracer,
            2 => EquipType::Necklace,
            3 => EquipType::Shoes,
            4 => EquipType::Ring,
            5 => EquipType::Dress,
            6 => EquipType::Weapon,
            _ => EquipType::None,
        }
    }
}

#[repr(u32)]
#[derive(Debug, Default, Copy, Clone, serde::Deserialize, PartialEq, Eq)]
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
