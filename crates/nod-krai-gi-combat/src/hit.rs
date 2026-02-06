use bevy_ecs::prelude::*;
use nod_krai_gi_entity::common::{EntityById, FightProperties, OwnerPlayerUID, ProtocolEntityID};
use nod_krai_gi_event::combat::*;
use nod_krai_gi_proto::ProtEntityType;
use tracing::{debug, instrument};

#[instrument(skip_all)]
pub fn deal_damage_on_hit(
    index: Res<EntityById>,
    mut events: MessageReader<EntityBeingHitEvent>,
    mut entities: Query<(
        &mut FightProperties,
        &ProtocolEntityID,
        Option<&OwnerPlayerUID>,
    )>,
) {
    for EntityBeingHitEvent(originator_uid, attack_result) in events.read() {
        let entity_type = attack_result.attacker_id >> 22;
        if entity_type < ProtEntityType::ProtEntityMax as u32
            && entity_type != ProtEntityType::ProtEntityMpLevel as u32
        {
            let attacker_entity = match index.0.get(&attack_result.attacker_id) {
                Some(e) => *e,
                None => continue,
            };

            let Ok((_, _, attacker_owner)) = entities.get(attacker_entity) else {
                debug!("attacker with id {} not found", attack_result.attacker_id);
                continue;
            };

            if let Some(owner_uid) = attacker_owner {
                if owner_uid.0 != *originator_uid {
                    debug!(
                        "fail: entity owner uid mismatch! owner uid: {}, event originator uid: {}",
                        owner_uid.0, originator_uid
                    );
                    continue;
                }
            }
        }

        let defense_entity = match index.0.get(&attack_result.defense_id) {
            Some(e) => *e,
            None => continue,
        };

        let Ok((mut defender_props, _, _)) = entities.get_mut(defense_entity) else {
            debug!("defender with id {} not found", attack_result.defense_id);
            continue;
        };

        defender_props.change_cur_hp(-attack_result.damage);
        debug!(
            "attacker (id: {}) dealt {} dmg to defender (id: {})",
            attack_result.attacker_id, attack_result.damage, attack_result.defense_id
        );
    }
}
