use std::collections::HashMap;
use common::string_util::InternString;

#[repr(u32)]
#[derive(Debug, Default, Clone, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EntityType {
    #[default]
    None,
    Avatar,
    Monster,
    Bullet,
    AttackPhyisicalUnit,
    AOE,
    Camera,
    EnviroArea,
    Equip,
    MonsterEquip,
    Grass,
    Level,
    NPC,
    TransPointFirst,
    TransPointFirstGadget,
    TransPointSecond,
    TransPointSecondGadget,
    DropItem,
    Field,
    Gadget,
    Water,
    GatherPoint,
    GatherObject,
    AirflowField,
    SpeedupField,
    Gear,
    Chest,
    EnergyBall,
    ElemCrystal,
    Timeline,
    Worktop,
    Team,
    Platform,
    AmberWind,
    EnvAnimal,
    SealGadget,
    Tree,
    Bush,
    QuestGadget,
    Lightning,
    RewardPoint,
    RewardStatue,
    MPLevel,
    WindSeed,
    MpPlayRewardPoint,
    ViewPoint,
    RemoteAvatar,
    GeneralRewardPoint,
    PlayTeam,
    OfferingGadget,
    EyePoint,
    MiracleRing,
    Foundation,
    WidgetGadget,
    Vehicle,
    SubEquip,
    FishRod,
    CustomTile,
    FishPool,
    CustomGadget,
    BlackMud,
    RoguelikeOperatorGadget,
    CurveMoveGadget,
    NightCrowGadget,
    CoinCollectLevelGadget,
    UgcTowerLevelUpGadget,
    JourneyGearOperatorGadget,
    UgcSpecialGadget,
    DeshretObeliskGadget,
    Projector,
    Screen,
    EchoShell,
    UIInteractGadget,
    Region,
    PlaceHolder,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GadgetExcelConfig {
    pub id: u32,
    pub r#type: EntityType,
    pub json_name: InternString,
    pub is_interactive: bool,
    pub tags: Vec<InternString>,
    pub item_json_name: InternString,
    #[serde(alias = "campID")]
    #[serde(default)]
    pub camp_id: u32,
    pub vision_level: InternString,
    pub name_text_map_hash: u64,
}

pub trait GadgetExcelConfigKeyed<K> {
    fn key(&self) -> K;

    fn load(excel_bin_output_path: &str) -> HashMap<K, GadgetExcelConfig>;
}

impl GadgetExcelConfigKeyed<u32> for GadgetExcelConfig {
    fn key(&self) -> u32 {
        self.id
    }

    fn load(excel_bin_output_path: &str) -> HashMap<u32, GadgetExcelConfig> {
        let json =  std::fs::read(&format!(
            "{excel_bin_output_path}/GadgetExcelConfigData.json"
        ))
        .unwrap();
        let list: Vec<GadgetExcelConfig> = serde_json::from_slice(&*json).unwrap();
        let data = list
            .iter()
            .map(|item| (item.key().clone(), item.clone()))
            .collect();
        data
    }
}
