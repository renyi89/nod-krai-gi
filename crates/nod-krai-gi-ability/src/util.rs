use bevy_ecs::prelude::*;
use common::string_util::{InternCheck, InternString};
use nod_krai_gi_data::ability::AbilityTargettingEnum;
use nod_krai_gi_data::ability::{get_ability_name_by_hash, AbilityModifierAction};
use nod_krai_gi_data::dynamic_float::NumberOrInternString;
use nod_krai_gi_data::prop_type::FightPropType;
use nod_krai_gi_data::DynamicFloat;
use nod_krai_gi_entity::avatar::{AvatarQueryReadOnly, CurrentPlayerAvatarMarker};
use nod_krai_gi_entity::common::FightProperties;
use nod_krai_gi_entity::common::InstancedAbility;
use nod_krai_gi_persistence::Players;
use nod_krai_gi_proto::ability_string::Type;
use nod_krai_gi_proto::AbilityString;

#[derive(Debug, Clone)]
enum MathOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl MathOp {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "ADD" => Some(MathOp::Add),
            "SUB" => Some(MathOp::Sub),
            "MUL" => Some(MathOp::Mul),
            "DIV" => Some(MathOp::Div),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct OpInfo {
    i: usize,
    name: String,
}

fn get_math_op(input: &[NumberOrInternString]) -> Option<OpInfo> {
    let op_list: Vec<OpInfo> = input
        .iter()
        .enumerate()
        .filter_map(|(i, val)| {
            if let NumberOrInternString::InternString(s) = val {
                let name = s.as_str().to_uppercase();
                if MathOp::from_str(&name).is_some() {
                    return Some(OpInfo { i, name });
                }
            }
            None
        })
        .collect();

    for op_name in ["ADD", "SUB", "MUL", "DIV"] {
        if let Some(op) = op_list.iter().find(|o| o.name == op_name) {
            return Some(op.clone());
        }
    }

    None
}

fn do_math(state: &mut MathState, input: &mut Vec<NumberOrInternString>) {
    let op = match get_math_op(input) {
        Some(op) => op,
        None => {
            state.busy = false;
            return;
        }
    };

    let index = if op.i >= 2 { op.i - 2 } else { 0 };

    let args: Vec<f32> = input
        .drain(index..index + 3)
        .take(2)
        .map(|val| match val {
            NumberOrInternString::Number(n) => n as f32,
            NumberOrInternString::InternString(_) => 0.0, // This should never happen with preprocessed input
        })
        .collect();

    if args.iter().any(|a| a.is_nan()) {
        return;
    }

    let result = match op.name.as_str() {
        "ADD" => args[0] + args[1],
        "SUB" => -args[1] + args[0],
        "MUL" => args[0] * args[1],
        "DIV" => {
            if args[1] != 0.0 {
                (1.0 / args[1]) * args[0]
            } else {
                return;
            }
        }
        _ => return,
    };

    state.val = result;
    input.insert(index, NumberOrInternString::Number(result as f64));
}

#[derive(Clone)]
struct MathState {
    busy: bool,
    val: f32,
}

fn calc(input: &mut Vec<NumberOrInternString>) -> f32 {
    let mut state = MathState {
        busy: true,
        val: 0.0,
    };

    while state.busy {
        do_math(&mut state, input);
    }

    state.val
}

fn eval_number_or_string(
    ability: &InstancedAbility,
    props: Option<&FightProperties>,
    val: &NumberOrInternString,
    def_val: f32,
) -> f32 {
    match val {
        NumberOrInternString::Number(n) => *n as f32,
        NumberOrInternString::InternString(s) => {
            if let Ok(num) = s.as_str().parse::<f32>() {
                return num;
            }

            if s.as_str().starts_with("FIGHT_PROP_") {
                if let Some(prop_type) = FightPropType::from_str(s.as_str()) {
                    if let Some(props) = props {
                        let result = props.get_property(prop_type);
                        return result;
                    } else {
                        return 0.0;
                    }
                }
            }

            let result = ability.ability_specials.get(s).copied().unwrap_or(def_val);
            result
        }
    }
}

pub fn eval(
    ability: &InstancedAbility,
    props: Option<&FightProperties>,
    val: &DynamicFloat,
    def_val: f32,
) -> f32 {
    match val {
        DynamicFloat::Number(n) => {
            let result = *n as f32;
            result
        }
        DynamicFloat::InternString(s) => {
            if let Ok(num) = s.as_str().parse::<f32>() {
                return num;
            }

            if s.as_str().starts_with("FIGHT_PROP_") {
                if let Some(prop_type) = FightPropType::from_str(s.as_str()) {
                    if let Some(props) = props {
                        let result = props.get_property(prop_type);
                        return result;
                    } else {
                        return 0.0;
                    }
                }
            }

            let result = ability.ability_specials.get(s).copied().unwrap_or(def_val);
            result
        }
        DynamicFloat::Array(arr) => {
            tracing::debug!("eval Array: {:?}", arr);
            let input = arr.clone();
            // Preprocess array to convert all elements to numbers or operators
            let mut preprocessed = Vec::new();
            for item in input {
                match &item {
                    NumberOrInternString::InternString(s) => {
                        // Check if it's an operator
                        let upper = s.as_str().to_uppercase();
                        if MathOp::from_str(&upper).is_some() {
                            // Keep operators as strings
                            preprocessed.push(NumberOrInternString::InternString(upper.into()));
                        } else {
                            // Evaluate strings to numbers
                            let num = eval_number_or_string(ability, props, &item, def_val);
                            preprocessed.push(NumberOrInternString::Number(num as f64));
                        }
                    }
                    NumberOrInternString::Number(_) => {
                        // Keep numbers as they are
                        preprocessed.push(item);
                    }
                }
            }
            tracing::debug!("eval Array: {:?}", preprocessed);
            let result = calc(&mut preprocessed);
            result
        }
    }
}

pub fn calc_amount(
    ability: &InstancedAbility,
    caster_props: &FightProperties,
    target_props: &FightProperties,
    action: &AbilityModifierAction,
) -> f32 {
    let target_max_hp = target_props.get_property(FightPropType::FIGHT_PROP_MAX_HP);
    let target_current_hp = target_props.get_property(FightPropType::FIGHT_PROP_CUR_HP);

    let mut amount = eval_option(ability, Some(caster_props), &action.amount, 0.0);

    if let Some(ratio) = &action.amount_by_caster_max_hp_ratio {
        let caster_max_hp = caster_props.get_property(FightPropType::FIGHT_PROP_MAX_HP);
        let ratio_val = eval(ability, Some(caster_props), ratio, 0.0);
        let added = caster_max_hp * ratio_val;
        amount += added;
    }

    if let Some(ratio) = &action.amount_by_caster_attack_ratio {
        let caster_attack = caster_props.get_property(FightPropType::FIGHT_PROP_CUR_ATTACK);
        let ratio_val = eval(ability, Some(caster_props), ratio, 0.0);
        let added = caster_attack * ratio_val;
        amount += added;
    }

    if let Some(ratio) = &action.amount_by_caster_current_hp_ratio {
        let caster_current_hp = caster_props.get_property(FightPropType::FIGHT_PROP_CUR_HP);
        let ratio_val = eval(ability, Some(caster_props), ratio, 0.0);
        let added = caster_current_hp * ratio_val;
        amount += added;
    }

    if let Some(ratio) = &action.amount_by_target_max_hp_ratio {
        let ratio_val = eval(ability, Some(target_props), ratio, 0.0);
        let added = target_max_hp * ratio_val;
        amount += added;
    }

    if let Some(ratio) = &action.amount_by_target_current_hp_ratio {
        let ratio_val = eval(ability, Some(target_props), ratio, 0.0);
        let added = target_current_hp * ratio_val;
        amount += added;
    }

    if let Some(ratio) = &action.limbo_by_target_max_hp_ratio {
        let eval_float = eval(ability, Some(target_props), ratio, 0.0);
        if eval_float > f32::EPSILON {
            let limbo_amount = f32::max(eval_float * target_max_hp, 1.0);
            let capped_amount = f32::max(target_current_hp - limbo_amount, 0.0);
            amount = f32::min(capped_amount, amount);
        }
    }

    amount
}

pub fn eval_option(
    ability: &InstancedAbility,
    props: Option<&FightProperties>,
    val: &Option<DynamicFloat>,
    def_val: f32,
) -> f32 {
    match val {
        Some(v) => eval(ability, props, v, def_val),
        None => def_val,
    }
}

#[allow(dead_code)]
pub fn get_affected_entities(
    target: AbilityTargettingEnum,
    self_entity: Entity,
    target_entity: Option<Entity>,
    avatars: Query<(
        Entity,
        AvatarQueryReadOnly,
        Option<&CurrentPlayerAvatarMarker>,
    )>,
    players: &Players,
    player_uid: u32,
) -> Vec<Entity> {
    match target {
        AbilityTargettingEnum::Self_ => {
            vec![self_entity]
        }
        AbilityTargettingEnum::Caster => {
            vec![self_entity]
        }
        AbilityTargettingEnum::Target => target_entity.into_iter().collect(),
        AbilityTargettingEnum::SelfAttackTarget => target_entity.into_iter().collect(),
        AbilityTargettingEnum::Other => target_entity.into_iter().collect(),
        AbilityTargettingEnum::Applier => {
            vec![self_entity]
        }
        AbilityTargettingEnum::Owner => {
            vec![self_entity]
        }
        AbilityTargettingEnum::CurTeamAvatars => {
            get_cur_team_avatars(player_uid, &avatars, players)
        }
        AbilityTargettingEnum::CurLocalAvatar => {
            get_cur_local_avatar(player_uid, &avatars, players)
        }
        AbilityTargettingEnum::OriginOwner => get_cur_local_avatar(player_uid, &avatars, players),
        AbilityTargettingEnum::Team => Vec::new(),
        AbilityTargettingEnum::TargetOwner => Vec::new(),
        AbilityTargettingEnum::TargetOriginOwner => Vec::new(),
        AbilityTargettingEnum::AllPlayerAvatars => get_all_player_avatars(&avatars, players),
        AbilityTargettingEnum::AllTeams => Vec::new(),
        AbilityTargettingEnum::RemoteTeams => Vec::new(),
        AbilityTargettingEnum::TargetTeam => Vec::new(),
        AbilityTargettingEnum::CasterOwner => Vec::new(),
        AbilityTargettingEnum::CasterOriginOwner => Vec::new(),
        AbilityTargettingEnum::MPLevel => Vec::new(),
    }
}

fn get_cur_team_avatars(
    player_uid: u32,
    avatars: &Query<(
        Entity,
        AvatarQueryReadOnly,
        Option<&CurrentPlayerAvatarMarker>,
    )>,
    players: &Players,
) -> Vec<Entity> {
    let player = players.get(player_uid);
    avatars
        .iter()
        .filter(|(_, data, _)| {
            data.owner_player_uid.0 == player_uid
                && player
                    .avatar_module
                    .cur_avatar_guid_list
                    .contains(&data.guid.0)
        })
        .map(|(e, _, _)| e)
        .collect()
}

fn get_cur_local_avatar(
    player_uid: u32,
    avatars: &Query<(
        Entity,
        AvatarQueryReadOnly,
        Option<&CurrentPlayerAvatarMarker>,
    )>,
    players: &Players,
) -> Vec<Entity> {
    let player = players.get(player_uid);
    avatars
        .iter()
        .filter(|(_, data, is_cur)| {
            data.owner_player_uid.0 == player_uid
                && player
                    .avatar_module
                    .cur_avatar_guid_list
                    .contains(&data.guid.0)
                && is_cur.is_some()
        })
        .map(|(e, _, _)| e)
        .collect()
}

fn get_all_player_avatars(
    avatars: &Query<(
        Entity,
        AvatarQueryReadOnly,
        Option<&CurrentPlayerAvatarMarker>,
    )>,
    players: &Players,
) -> Vec<Entity> {
    let mut all_team_guids = Vec::new();

    for player_uid in players.keys() {
        let player = players.get(*player_uid);
        all_team_guids.extend(player.avatar_module.cur_avatar_guid_list.iter());
    }

    avatars
        .iter()
        .filter(|(_, data, _)| all_team_guids.contains(&data.guid.0))
        .map(|(e, _, _)| e)
        .collect()
}

pub fn get_ability_name(ability_name: Option<AbilityString>) -> Option<InternString> {
    match ability_name {
        Some(ability_name) => match ability_name.r#type.as_ref() {
            Some(Type::Str(s)) => {
                if s.is_interned() {
                    Some(s.clone().into())
                } else {
                    tracing::debug!("ability:{} is not interned", s);
                    None
                }
            }
            Some(Type::Hash(hash)) => match get_ability_name_by_hash(*hash) {
                Some(name) => Some(name),
                None => {
                    tracing::debug!("No ability found for hash {}", hash);
                    None
                }
            },
            None => {
                tracing::debug!("No ability name or hash provided");
                None
            }
        },
        None => {
            tracing::debug!("No ability name provided");
            None
        }
    }
}
