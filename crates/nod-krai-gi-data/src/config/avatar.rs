#[derive(Debug, serde::Deserialize)]
pub struct AvatarConfig {
    #[serde(default)]
    pub abilities: Vec<AvatarAbility>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarAbility {
    #[serde(default)]
    pub ability_id: String,
    pub ability_name: String,
    #[serde(default)]
    pub ability_override: String,
}

impl AvatarAbility {
    pub const TYPE_IDENTIFIER: u32 = 7;
    pub const DEFAULT_OVERRIDE: &str = "Default";
}
