use std::collections::HashMap;

struct FieldMap {
    map: HashMap<String, String>,
}

impl FieldMap {
    fn take<T: std::str::FromStr>(&mut self, key: &str) -> Result<T, String> {
        let v = self
            .map
            .remove(key)
            .ok_or(format!("miss field `{}`", key))?;
        v.parse()
            .map_err(|_| format!("field `{}` error:{}", key, v))
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
        for pair in raw.split(';') {
            let mut kv = pair.split(',');
            let k = kv.next().ok_or("props error")?;
            let v = kv.next().ok_or("props error")?;
            out.insert(
                k.parse().map_err(|_| "props key error")?,
                v.parse().map_err(|_| "props value error")?,
            );
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
        for pair in raw.split(';') {
            let mut kv = pair.split(',');
            let k = kv.next().ok_or("props error")?;
            let v = kv.next().ok_or("props error")?;
            out.insert(
                k.parse().map_err(|_| "props key error")?,
                v.parse().map_err(|_| "props value error")?,
            );
        }
        Ok(out)
    }
}

fn parse_struct<'a, I, F, T>(mut parts: I, help: &str, build: F) -> Result<T, String>
where
    I: Iterator<Item = &'a str>,
    F: FnOnce(FieldMap) -> Result<T, String>,
{
    let mut map = HashMap::new();

    if let Some(id) = parts.next() {
        map.insert("id".to_string(), id.to_string());
    } else {
        return Err(format!("miss id. usage:{}", help));
    }

    while let Some(key) = parts.next() {
        let value = parts.next().ok_or(format!("field {} error", key))?;
        map.insert(key.to_string(), value.to_string());
    }

    let fm = FieldMap { map };
    build(fm).map_err(|e| format!("{}\n usage:{}", e, help))
}

#[allow(unused)]
#[derive(Debug)]
pub enum Command {
    Avatar(AvatarAction),
    Tp(TpAction),
    Quest(QuestAction),
    Item(ItemAction),
    Prop(String, String),
    Dun(Option<u32>),
    Pos,
}

#[allow(unused)]
#[derive(Debug)]
pub enum AvatarAction {
    Add {
        id: u32,
        c: Option<u32>,
        lv: Option<u32>,
        sl: Option<u32>,
    },
    Lv {
        id: u32,
    },
}

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

#[allow(unused)]
#[derive(Debug)]
pub enum QuestAction {
    Begin { id: u32 },
    Finish { id: u32 },
}

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
    Drop {
        id: u32,
        pos: Option<(f32, f32, f32)>,
    },
}

pub fn parse_command(input: &str) -> Result<Command, String> {
    let input = input.trim();

    let input = input.strip_prefix('/').unwrap_or(input);

    let mut parts = input.split_whitespace().peekable();

    let first = parts.next().ok_or("none")?;

    match first {
        "prop" => {
            let k = parts.next().ok_or("prop <key> <value>")?.to_string();
            let v = parts.next().ok_or("prop <key> <value>")?.to_string();
            return Ok(Command::Prop(k, v));
        }

        "dun" => {
            let id = parts
                .next()
                .map(|v| v.parse::<u32>())
                .transpose()
                .map_err(|_| "dun <num>".to_string())?;
            return Ok(Command::Dun(id));
        }

        "pos" => {
            return Ok(Command::Pos);
        }

        _ => {}
    }

    let second = parts.next().ok_or(format!("unknown command:{}", first))?;

    match (first, second) {
        ("avatar", "add") => parse_struct(
            parts,
            "avatar add <id> [level <num>] [sl <num>]",
            |mut map| {
                Ok(Command::Avatar(AvatarAction::Add {
                    id: map.take("id")?,
                    c: map.take_opt("c"),
                    lv: map.take_opt("lv"),
                    sl: map.take_opt("sl"),
                }))
            },
        ),

        ("avatar", "lv") => parse_struct(parts, "avatar lv <id>", |mut map| {
            Ok(Command::Avatar(AvatarAction::Lv {
                id: map.take("id")?,
            }))
        }),

        ("tp", "a") => parse_struct(
            parts,
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

        ("quest", "begin") => parse_struct(parts, "quest begin <id>", |mut map| {
            Ok(Command::Quest(QuestAction::Begin {
                id: map.take("id")?,
            }))
        }),

        ("quest", "finish") => parse_struct(parts, "quest finish <id>", |mut map| {
            Ok(Command::Quest(QuestAction::Finish {
                id: map.take("id")?,
            }))
        }),

        ("item", "add") => {
            parse_struct(parts, "item add <id> [p k,v;k,v] [lv <num>]", |mut map| {
                Ok(Command::Item(ItemAction::Add {
                    id: map.take("id")?,
                    num: map.take_opt("n"),
                    level: map.take_opt("lv"),
                    refinement: map.take_opt("r"),
                    main_prop_id: map.take_opt("m"),
                    append_prop_id_list: map.take_map_u32("p")?,
                }))
            })
        }

        ("item", "drop") => parse_struct(parts, "item drop <id>", |mut map| {
            Ok(Command::Item(ItemAction::Drop {
                id: map.take("id")?,
                pos: None,
            }))
        }),

        _ => Err(format!("unknown command:{} {}", first, second)),
    }
}
