use bevy_ecs::prelude::*;
use nod_krai_gi_data::{
    excel::{avatar_excel_config_collection, weapon_excel_config_collection},
    prop_type::FightPropType,
};
use nod_krai_gi_entity::{
    avatar::{AvatarEquipChangeEvent, AvatarQueryReadOnly, Equipment},
    common::{
        create_fight_props_with_weapon, EntityCounter, FightProperties, GadgetID, Guid, Level,
        OwnerPlayerUID, ProtocolEntityID, ToBeRemovedMarker,
    },
    util::to_protocol_entity_id,
    weapon::{AffixMap, WeaponPromoteLevel, WeaponBundle, WeaponID, WeaponQueryReadOnly},
};
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_persistence::{player_information::ItemBin, Players};
use nod_krai_gi_proto::normal::{
    AbilitySyncStateInfo, AvatarEquipChangeNotify, EntityRendererChangedInfo, ProtEntityType,
    SceneWeaponInfo,
};

pub fn notify_avatar_equip_change(
    avatars: Query<AvatarQueryReadOnly, Changed<Equipment>>,
    weapons: Query<WeaponQueryReadOnly>,
    message_output: Res<MessageOutput>,
) {
    for avatar_data in avatars.iter() {
        let Ok(weapon_data) = weapons.get(avatar_data.equipment.weapon) else {
            tracing::debug!(
                "weapon config {} doesn't exist",
                avatar_data.equipment.weapon
            );
            continue;
        };

        message_output.send_to_all(
            "AvatarEquipChangeNotify",
            AvatarEquipChangeNotify {
                avatar_guid: avatar_data.guid.0,
                equip_guid: weapon_data.guid.0,
                item_id: weapon_data.weapon_id.0,
                equip_type: 6,
                weapon: Some(SceneWeaponInfo {
                    guid: weapon_data.guid.0,
                    entity_id: weapon_data.entity_id.0,
                    gadget_id: weapon_data.gadget_id.0,
                    item_id: weapon_data.weapon_id.0,
                    level: weapon_data.level.0,
                    promote_level: weapon_data.promote_level.0,
                    affix_map: weapon_data.affix_map.0.clone(),
                    ability_info: Some(AbilitySyncStateInfo::default()),
                    renderer_changed_info: Some(EntityRendererChangedInfo::default()),
                    ..Default::default()
                }),
                reliquary: None,
            },
        )
    }
}

pub fn apply_equip_change_to_avatar_entity(
    mut events: MessageReader<AvatarEquipChangeEvent>,
    mut commands: Commands,
    mut avatars: Query<(
        &ProtocolEntityID,
        &Guid,
        &OwnerPlayerUID,
        &mut Equipment,
        &mut FightProperties,
    )>,
    mut entity_counter: ResMut<EntityCounter>,
    players: Res<Players>,
) {
    let avatar_excel_config_collection_clone =
        std::sync::Arc::clone(avatar_excel_config_collection::get());

    let weapon_excel_config_collection_clone =
        std::sync::Arc::clone(weapon_excel_config_collection::get());

    for avatar_equip_change in events.read() {
        let Some((_, _, _, mut equipment, mut fight_props)) =
            avatars.iter_mut().find(|(_, guid, owner_uid, _, _)| {
                owner_uid.0 == avatar_equip_change.player_uid
                    && guid.0 == avatar_equip_change.avatar_guid
            })
        else {
            continue;
        };

        commands.entity(equipment.weapon).insert(ToBeRemovedMarker);

        let Some(player_info) = players.get(avatar_equip_change.player_uid) else {
            continue;
        };
        let Some(avatar) = player_info
            .avatar_bin
            .avatar_map
            .get(&avatar_equip_change.avatar_guid)
        else {
            tracing::debug!(
                "avatar guid {} doesn't exist",
                avatar_equip_change.avatar_guid
            );
            continue;
        };

        let Some(ItemBin::Weapon {
            weapon_id,
            level,
            exp: _,
            promote_level,
            affix_map,
            is_locked: _,
        }) = player_info.item_bin.get_item(&avatar_equip_change.weapon_guid)
        else {
            tracing::debug!(
                "weapon guid {} doesn't exist",
                avatar_equip_change.weapon_guid
            );
            continue;
        };

        let Some(weapon_config) = weapon_excel_config_collection_clone.get(weapon_id) else {
            tracing::debug!("weapon config {} doesn't exist", weapon_id);
            continue;
        };

        let weapon_entity = commands
            .spawn(WeaponBundle {
                weapon_id: WeaponID(*weapon_id),
                entity_id: to_protocol_entity_id(
                    ProtEntityType::ProtEntityWeapon,
                    entity_counter.inc(),
                ),
                level: Level(*level),
                guid: Guid(avatar_equip_change.weapon_guid),
                gadget_id: GadgetID(weapon_config.gadget_id),
                affix_map: AffixMap(affix_map.clone()),
                promote_level: WeaponPromoteLevel(*promote_level),
            })
            .id();

        equipment.weapon = weapon_entity;

        let cur_hp = fight_props.get_property(FightPropType::FIGHT_PROP_CUR_HP);
        let Some(avatar_data) = avatar_excel_config_collection_clone.get(&avatar.avatar_id) else {
            tracing::debug!("avatar config {} doesn't exist", avatar.avatar_id);
            continue;
        };
        *fight_props = create_fight_props_with_weapon(
            avatar_data,
            cur_hp,
            avatar.level,
            avatar.promote_level,
            weapon_config,
            *level,
        );
    }
}
