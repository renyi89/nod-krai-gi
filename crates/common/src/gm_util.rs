use std::collections::HashMap;

// ============================================================================
// 内部辅助结构
// ============================================================================

struct FieldMap {
    map: HashMap<String, String>,
}

impl FieldMap {
    fn take<T: std::str::FromStr>(&mut self, key: &str) -> Result<T, String> {
        let v = self
            .map
            .remove(key)
            .ok_or_else(|| format!("缺少参数: {}", key))?;
        v.parse::<T>()
            .map_err(|_| format!("参数 {} 的值无效: \"{}\"", key, v))
    }

    fn take_opt<T: std::str::FromStr>(&mut self, key: &str) -> Option<T> {
        self.map.remove(key)?.parse().ok()
    }

    fn take_map_u32(&mut self, key: &str) -> Result<HashMap<u32, u32>, String> {
        let raw = match self.map.remove(key) {
            Some(v) => v,
            None => return Ok(HashMap::new()),
        };

        let mut out = HashMap::new();
        for (i, pair) in raw.split(';').enumerate() {
            let mut kv = pair.split(',');
            let k = kv.next().ok_or_else(|| {
                format!("参数 {} 第 {} 组缺少 key", key, i + 1)
            })?;
            let v = kv.next().ok_or_else(|| {
                format!("参数 {} 第 {} 组缺少 value", key, i + 1)
            })?;
            let parsed_k: u32 = k.parse().map_err(|_| {
                format!("参数 {} 第 {} 组的 key 无效: \"{}\"", key, i + 1, k)
            })?;
            let parsed_v: u32 = v.parse().map_err(|_| {
                format!("参数 {} 第 {} 组的 value 无效: \"{}\"", key, i + 1, v)
            })?;
            out.insert(parsed_k, parsed_v);
        }
        Ok(out)
    }

    #[allow(unused)]
    fn take_map_f32(&mut self, key: &str) -> Result<HashMap<u32, f32>, String> {
        let raw = match self.map.remove(key) {
            Some(v) => v,
            None => return Ok(HashMap::new()),
        };

        let mut out = HashMap::new();
        for (i, pair) in raw.split(';').enumerate() {
            let mut kv = pair.split(',');
            let k = kv.next().ok_or_else(|| {
                format!("参数 {} 第 {} 组缺少 key", key, i + 1)
            })?;
            let v = kv.next().ok_or_else(|| {
                format!("参数 {} 第 {} 组缺少 value", key, i + 1)
            })?;
            let parsed_k: u32 = k.parse().map_err(|_| {
                format!("参数 {} 第 {} 组的 key 无效: \"{}\"", key, i + 1, k)
            })?;
            let parsed_v: f32 = v.parse().map_err(|_| {
                format!("参数 {} 第 {} 组的 value 无效: \"{}\"", key, i + 1, v)
            })?;
            out.insert(parsed_k, parsed_v);
        }
        Ok(out)
    }
}

// ============================================================================
// 内部解析辅助函数
// ============================================================================

fn parse_single<'a, I>(mut parts: I, help: &str) -> Result<String, String>
where
    I: Iterator<Item = &'a str>,
{
    parts
        .next()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("参数错误\n用法: {}", help))
}

fn parse_single_u32<'a, I>(mut parts: I, param_name: &str, help: &str) -> Result<u32, String>
where
    I: Iterator<Item = &'a str>,
{
    let raw = parts
        .next()
        .ok_or_else(|| format!("缺少参数: {}\n用法: {}", param_name, help))?;
    raw.parse::<u32>()
        .map_err(|_| format!("参数 {} 的值无效: \"{}\"\n用法: {}", param_name, raw, help))
}

fn parse_struct<'a, I, F, T>(
    mut parts: I,
    first_param: &str,
    help: &str,
    build: F,
) -> Result<T, String>
where
    I: Iterator<Item = &'a str>,
    F: FnOnce(FieldMap) -> Result<T, String>,
{
    let mut map = HashMap::new();

    if let Some(first_val) = parts.next() {
        map.insert(first_param.to_string(), first_val.to_string());
    } else {
        return Err(format!("缺少参数: {}\n用法: {}", first_param, help));
    }

    while let Some(key) = parts.next() {
        let value = parts
            .next()
            .ok_or_else(|| format!("参数 {} 缺少值\n用法: {}", key, help))?;
        map.insert(key.to_string(), value.to_string());
    }

    let fm = FieldMap { map };
    build(fm).map_err(|e| format!("{}\n用法: {}", e, help))
}

// ============================================================================
// 公共类型定义
// ============================================================================

#[allow(unused)]
#[derive(Debug)]
pub enum Command {
    // 角色相关
    Avatar(AvatarAction),
    // buff 操作
    Buff(BuffAction),
    // 物品相关
    Item(ItemAction),
    Weapon(WeaponAction),
    // 任务相关
    Quest(QuestAction),
    // 装置相关
    Gadget(GadgetAction),
    // 群组相关
    Group(GroupAction),
    // 天气与环境
    Weather(u32),
    Climate(u32),
    // 传送相关
    Tp(TpAction),
    // 祈愿相关
    Gacha(GachaAction),
    // 其他
    Prop(String, String),
    SendPacket(String),
    Dun(Option<u32>),
    Pos,
}

// ----------------------------------------------------------------------------
// 角色相关
// ----------------------------------------------------------------------------

#[allow(unused)]
#[derive(Debug)]
pub enum AvatarAction {
    Add { id: u32 },
    Remove { id: u32 },
    Rename { id: u32, name: String },
    Level { level: u32 },
    Break { break_level: u32 },
    AddTalent { talent_id: u32 },
    Skill { skill_id: u32, level: u32 },
    Elem { element_type: u32 },
    FightProp { key: String, value: f32 },
}

// ----------------------------------------------------------------------------
// buff 操作
// ----------------------------------------------------------------------------

#[allow(unused)]
#[derive(Debug)]
pub enum BuffAction {
    Add { id: u32, level: Option<u32> },
    Clear,
    List,
}

// ----------------------------------------------------------------------------
// 物品相关
// ----------------------------------------------------------------------------

#[allow(unused)]
#[derive(Debug)]
pub enum ItemAction {
    Add {
        id: u32,
        num: Option<u32>,
        level: Option<u32>,
        refinement: Option<u32>,
        main_prop_id: Option<u32>,
        append_prop_id_list: HashMap<u32, u32>,
    },
    AddMaterial { max_count: Option<u32> },
    AddFurniture { max_count: Option<u32> },
    AddWeapon,
    Clear { target: Option<String> },
    Drop { id: u32 },
}

#[allow(unused)]
#[derive(Debug)]
pub enum WeaponAction {
    Level { level: u32 },
    Break { break_level: u32 },
    Promote { promote_level: u32 },
}

// ----------------------------------------------------------------------------
// 任务相关
// ----------------------------------------------------------------------------

#[allow(unused)]
#[derive(Debug)]
pub enum QuestAction {
    Accept { id: u32 },
    Finish { id: u32 },
    Cancel { id: u32 },
    Clear { id: Option<u32> },
    State { id: u32, state: u32 },
    Restart { id: u32 },
    RestartAll,
    Var { parent_id: u32, index: Option<u32>, value: Option<u32> },
}

// ----------------------------------------------------------------------------
// 装置相关
// ----------------------------------------------------------------------------

#[allow(unused)]
#[derive(Debug)]
pub enum GadgetAction {
    Create {
        id: u32,
        num: Option<u32>,
        drop_id: Option<u32>,
        level: Option<u32>,
        interact_id: Option<u32>,
        x: Option<f32>,
        y: Option<f32>,
        z: Option<f32>,
    },
    Remove { id: u32 },
    State { id: u32, state: u32 },
    SetStateByEntityId { entity_id: u32, state: u32 },
}

// ----------------------------------------------------------------------------
// 群组相关
// ----------------------------------------------------------------------------

#[allow(unused)]
#[derive(Debug)]
pub enum GroupAction {
    Refresh { id: u32 },
    Unload { id: u32 },
    Reload { id: u32 },
    Clear { id: u32 },
    SuiteAddExtra { id: u32, suite_id: u32 },
    SuiteRemoveExtra { id: u32, suite_id: u32 },
    SuiteKillExtra { id: u32, suite_id: u32 },
    SuiteGoto { id: u32, suite_id: u32 },
}

// ----------------------------------------------------------------------------
// 传送相关
// ----------------------------------------------------------------------------

#[allow(unused)]
#[derive(Debug)]
pub enum TpAction {
    A {
        id: u32,
        x: Option<f32>,
        y: Option<f32>,
        z: Option<f32>,
    },
    R {
        id: u32,
        x: Option<f32>,
        y: Option<f32>,
        z: Option<f32>,
    },
}

// ----------------------------------------------------------------------------
// 祈愿相关
// ----------------------------------------------------------------------------

#[allow(unused)]
#[derive(Debug)]
pub enum GachaAction {
    Add { id: u32 },
    Clear {},
}

// ============================================================================
// 公共解析函数
// ============================================================================

pub fn parse_command(input: &str) -> Result<Command, String> {
    let input = input.trim();
    let input = input.strip_prefix('/').unwrap_or(input);
    let mut parts = input.split_whitespace().peekable();

    let first = parts.next().ok_or("命令不能为空")?;

    // ------------------------------------------------------------------------
    // 单级命令处理
    // ------------------------------------------------------------------------
    match first {
        "prop" => {
            let k = parse_single(&mut parts, "prop <key> <value>")?;
            let v = parse_single(&mut parts, "prop <key> <value>")?;
            return Ok(Command::Prop(k, v));
        }

        "send_packet" => {
            let k = parse_single(&mut parts, "send_packet <key>")?;
            return Ok(Command::SendPacket(k));
        }

        "dun" => {
            let id = parts
                .next()
                .map(|v| {
                    v.parse::<u32>()
                        .map_err(|_| format!("副本 ID 无效: \"{}\"\n用法: dun [id]", v))
                })
                .transpose()?;
            return Ok(Command::Dun(id));
        }

        "pos" => {
            return Ok(Command::Pos);
        }

        "weather" => {
            let id = parse_single_u32(&mut parts, "weather_id", "weather <weather_id>")?;
            return Ok(Command::Weather(id));
        }

        "climate" => {
            let climate_type =
                parse_single_u32(&mut parts, "climate_type", "climate <climate_type>")?;
            return Ok(Command::Climate(climate_type));
        }

        _ => {}
    }

    let second = parts.next().ok_or_else(|| format!("unknown command: {}", first))?;

    // ------------------------------------------------------------------------
    // 双级命令处理
    // ------------------------------------------------------------------------
    match (first, second) {
        // --------------------------------------------------------------------
        // 角色相关
        // --------------------------------------------------------------------
        ("avatar", "add") => parse_struct(
            parts,
            "avatar_id",
            "avatar add <avatar_id>",
            |mut map| Ok(Command::Avatar(AvatarAction::Add { id: map.take("avatar_id")? })),
        ),

        ("avatar", "remove") => parse_struct(
            parts,
            "avatar_id",
            "avatar remove <avatar_id>",
            |mut map| Ok(Command::Avatar(AvatarAction::Remove { id: map.take("avatar_id")? })),
        ),

        ("avatar", "rename") => {
            let help = "avatar rename <avatar_id> <new_name>";
            let id = parse_single_u32(&mut parts, "avatar_id", help)?;
            let name = parse_single(&mut parts, help)?;
            Ok(Command::Avatar(AvatarAction::Rename { id, name }))
        }

        ("avatar", "level") => parse_struct(
            parts,
            "level",
            "avatar level <level>",
            |mut map| Ok(Command::Avatar(AvatarAction::Level { level: map.take("level")? })),
        ),

        ("avatar", "break") => parse_struct(
            parts,
            "break_level",
            "avatar break <break_level>",
            |mut map| Ok(Command::Avatar(AvatarAction::Break { break_level: map.take("break_level")? })),
        ),

        ("avatar", "add_talent") => parse_struct(
            parts,
            "talent_id",
            "avatar add_talent <talent_id>",
            |mut map| Ok(Command::Avatar(AvatarAction::AddTalent { talent_id: map.take("talent_id")? })),
        ),

        ("avatar", "skill") => {
            let help = "avatar skill <skill_id> <level>";
            let skill_id = parse_single_u32(&mut parts, "skill_id", help)?;
            let level = parse_single_u32(&mut parts, "level", help)?;
            Ok(Command::Avatar(AvatarAction::Skill { skill_id, level }))
        }

        ("avatar", "elem") => parse_struct(
            parts,
            "element_type",
            "avatar elem <element_type>",
            |mut map| Ok(Command::Avatar(AvatarAction::Elem { element_type: map.take("element_type")? })),
        ),

        ("avatar", "fight_prop") => {
            let help = "avatar fight_prop <key> <value>";
            let key = parse_single(&mut parts, help)?;
            let value = parse_single(&mut parts, help)?.parse::<f32>()
                .map_err(|_| format!("value err: {}", help))?;
            Ok(Command::Avatar(AvatarAction::FightProp { key, value }))
        }

        // --------------------------------------------------------------------
        // buff 操作
        // --------------------------------------------------------------------
        ("buff", "add") => parse_struct(
            parts,
            "buff_id",
            "buff add <buff_id> [level <num>]",
            |mut map| Ok(Command::Buff(BuffAction::Add {
                id: map.take("buff_id")?,
                level: map.take_opt("level"),
            })),
        ),

        ("buff", "clear") => Ok(Command::Buff(BuffAction::Clear)),

        ("buff", "list") => Ok(Command::Buff(BuffAction::List)),

        // --------------------------------------------------------------------
        // 物品相关
        // --------------------------------------------------------------------
        ("item", "add") => {
            let next = parts.peek();
            match next {
                Some(&"material") => {
                    parts.next();
                    let max_count = parts.next().and_then(|v| v.parse::<u32>().ok());
                    Ok(Command::Item(ItemAction::AddMaterial { max_count }))
                }
                Some(&"furniture") => {
                    parts.next();
                    let max_count = parts.next().and_then(|v| v.parse::<u32>().ok());
                    Ok(Command::Item(ItemAction::AddFurniture { max_count }))
                }
                Some(&"weapon") => {
                    parts.next();
                    Ok(Command::Item(ItemAction::AddWeapon))
                }
                _ => parse_struct(
                    parts,
                    "item_id",
                    "item add <item_id> [n <num>] [lv <num>] [r <num>] [m <num>] [p k,v;k,v]",
                    |mut map| {
                        Ok(Command::Item(ItemAction::Add {
                            id: map.take("item_id")?,
                            num: map.take_opt("n"),
                            level: map.take_opt("lv"),
                            refinement: map.take_opt("r"),
                            main_prop_id: map.take_opt("m"),
                            append_prop_id_list: map.take_map_u32("p")?,
                        }))
                    },
                ),
            }
        }

        ("item", "clear") => {
            let target = parts.next().map(|v| v.to_string());
            Ok(Command::Item(ItemAction::Clear { target }))
        }

        ("item", "drop") => parse_struct(
            parts,
            "item_id",
            "item drop <item_id>",
            |mut map| Ok(Command::Item(ItemAction::Drop { id: map.take("item_id")? })),
        ),

        ("weapon", "level") => parse_struct(
            parts,
            "level",
            "weapon level <level>",
            |mut map| Ok(Command::Weapon(WeaponAction::Level { level: map.take("level")? })),
        ),

        ("weapon", "break") => parse_struct(
            parts,
            "break_level",
            "weapon break <break_level>",
            |mut map| Ok(Command::Weapon(WeaponAction::Break { break_level: map.take("break_level")? })),
        ),

        ("weapon", "promote") => parse_struct(
            parts,
            "promote_level",
            "weapon promote <promote_level>",
            |mut map| Ok(Command::Weapon(WeaponAction::Promote { promote_level: map.take("promote_level")? })),
        ),

        // --------------------------------------------------------------------
        // 任务相关
        // --------------------------------------------------------------------
        ("quest", "accept") => parse_struct(
            parts,
            "quest_id",
            "quest accept <quest_id>",
            |mut map| Ok(Command::Quest(QuestAction::Accept { id: map.take("quest_id")? })),
        ),

        ("quest", "finish") => parse_struct(
            parts,
            "quest_id",
            "quest finish <quest_id>",
            |mut map| Ok(Command::Quest(QuestAction::Finish { id: map.take("quest_id")? })),
        ),

        ("quest", "cancel") => parse_struct(
            parts,
            "quest_id",
            "quest cancel <quest_id>",
            |mut map| Ok(Command::Quest(QuestAction::Cancel { id: map.take("quest_id")? })),
        ),

        ("quest", "clear") => {
            let id = parts.next().and_then(|v| v.parse::<u32>().ok());
            Ok(Command::Quest(QuestAction::Clear { id }))
        }

        ("quest", "state") => {
            let help = "quest state <quest_id> <state>";
            let id = parse_single_u32(&mut parts, "quest_id", help)?;
            let state = parse_single_u32(&mut parts, "state", help)?;
            Ok(Command::Quest(QuestAction::State { id, state }))
        }

        ("quest", "restart") => parse_struct(
            parts,
            "quest_id",
            "quest restart <quest_id>",
            |mut map| Ok(Command::Quest(QuestAction::Restart { id: map.take("quest_id")? })),
        ),

        ("quest", "restart_all") => Ok(Command::Quest(QuestAction::RestartAll)),

        ("quest", "var") => {
            let help = "quest var <parent_id> [index] [value]";
            let parent_id = parse_single_u32(&mut parts, "parent_id", help)?;
            let index = parts.next().and_then(|v| v.parse::<u32>().ok());
            let value = parts.next().and_then(|v| v.parse::<u32>().ok());
            Ok(Command::Quest(QuestAction::Var {
                parent_id,
                index,
                value,
            }))
        }

        // --------------------------------------------------------------------
        // 装置相关
        // --------------------------------------------------------------------
        ("gadget", "create") => parse_struct(
            parts,
            "gadget_id",
            "gadget create <gadget_id> [num <n>] [drop_id <id>] [level <n>] [interact_id <id>] [x <n>] [y <n>] [z <n>]",
            |mut map| {
                Ok(Command::Gadget(GadgetAction::Create {
                    id: map.take("gadget_id")?,
                    num: map.take_opt("num"),
                    drop_id: map.take_opt("drop_id"),
                    level: map.take_opt("level"),
                    interact_id: map.take_opt("interact_id"),
                    x: map.take_opt("x"),
                    y: map.take_opt("y"),
                    z: map.take_opt("z"),
                }))
            },
        ),

        ("gadget", "remove") => parse_struct(
            parts,
            "gadget_id",
            "gadget remove <gadget_id>",
            |mut map| Ok(Command::Gadget(GadgetAction::Remove { id: map.take("gadget_id")? })),
        ),

        ("gadget", "state") => {
            let help = "gadget state <gadget_id> <state>";
            let id = parse_single_u32(&mut parts, "gadget_id", help)?;
            let state = parse_single_u32(&mut parts, "state", help)?;
            Ok(Command::Gadget(GadgetAction::State { id, state }))
        }

        ("gadget", "set_state_by_entity_id") => {
            let help = "gadget set_state_by_entity_id <entity_id> <state>";
            let entity_id = parse_single_u32(&mut parts, "entity_id", help)?;
            let state = parse_single_u32(&mut parts, "state", help)?;
            Ok(Command::Gadget(GadgetAction::SetStateByEntityId { entity_id, state }))
        }

        // --------------------------------------------------------------------
        // 群组相关
        // --------------------------------------------------------------------
        ("group", "refresh") => parse_struct(
            parts,
            "group_id",
            "group refresh <group_id>",
            |mut map| Ok(Command::Group(GroupAction::Refresh { id: map.take("group_id")? })),
        ),

        ("group", "unload") => parse_struct(
            parts,
            "group_id",
            "group unload <group_id>",
            |mut map| Ok(Command::Group(GroupAction::Unload { id: map.take("group_id")? })),
        ),

        ("group", "reload") => parse_struct(
            parts,
            "group_id",
            "group reload <group_id>",
            |mut map| Ok(Command::Group(GroupAction::Reload { id: map.take("group_id")? })),
        ),

        ("group", "clear") => parse_struct(
            parts,
            "group_id",
            "group clear <group_id>",
            |mut map| Ok(Command::Group(GroupAction::Clear { id: map.take("group_id")? })),
        ),

        ("group_suite", "add_extra") => {
            let help = "group_suite add_extra <group_id> <suite_id>";
            let id = parse_single_u32(&mut parts, "group_id", help)?;
            let suite_id = parse_single_u32(&mut parts, "suite_id", help)?;
            Ok(Command::Group(GroupAction::SuiteAddExtra { id, suite_id }))
        }

        ("group_suite", "remove_extra") => {
            let help = "group_suite remove_extra <group_id> <suite_id>";
            let id = parse_single_u32(&mut parts, "group_id", help)?;
            let suite_id = parse_single_u32(&mut parts, "suite_id", help)?;
            Ok(Command::Group(GroupAction::SuiteRemoveExtra { id, suite_id }))
        }

        ("group_suite", "kill_extra") => {
            let help = "group_suite kill_extra <group_id> <suite_id>";
            let id = parse_single_u32(&mut parts, "group_id", help)?;
            let suite_id = parse_single_u32(&mut parts, "suite_id", help)?;
            Ok(Command::Group(GroupAction::SuiteKillExtra { id, suite_id }))
        }

        ("group_suite", "goto") => {
            let help = "group_suite goto <group_id> <suite_id>";
            let id = parse_single_u32(&mut parts, "group_id", help)?;
            let suite_id = parse_single_u32(&mut parts, "suite_id", help)?;
            Ok(Command::Group(GroupAction::SuiteGoto { id, suite_id }))
        }

        // --------------------------------------------------------------------
        // 传送相关
        // --------------------------------------------------------------------
        ("tp", "a") => parse_struct(
            parts,
            "id",
            "tp a <id> [x <num>] [y <num>] [z <num>]",
            |mut map| {
                Ok(Command::Tp(TpAction::A {
                    id: map.take("id")?,
                    x: map.take_opt("x"),
                    y: map.take_opt("y"),
                    z: map.take_opt("z"),
                }))
            },
        ),

        ("tp", "r") => parse_struct(
            parts,
            "id",
            "tp r <id> [x <num>] [y <num>] [z <num>]",
            |mut map| {
                Ok(Command::Tp(TpAction::R {
                    id: map.take("id")?,
                    x: map.take_opt("x"),
                    y: map.take_opt("y"),
                    z: map.take_opt("z"),
                }))
            },
        ),

        // --------------------------------------------------------------------
        // 祈愿相关
        // --------------------------------------------------------------------
        ("gacha", "add") => parse_struct(
            parts,
            "gacha_id",
            "gacha add <gacha_id>",
            |mut map| Ok(Command::Gacha(GachaAction::Add { id: map.take("gacha_id")? })),
        ),

        ("gacha", "clear") => Ok(Command::Gacha(GachaAction::Clear {})),

        // --------------------------------------------------------------------
        // 未知命令
        // --------------------------------------------------------------------
        _ => Err(format!("unknown command: {} {}", first, second)),
    }
}
