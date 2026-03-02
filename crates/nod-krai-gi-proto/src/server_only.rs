// disable lints on generated code
#![allow(clippy::all)]
#![allow(unused)]
#![allow(missing_docs)]
include!("../gen/server_only/_.rs");

impl VectorBin {
    pub fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
    }

    pub fn is_zero(&self) -> bool {
        self.x.abs() < 0.0000001 && self.y.abs() < 0.0000001 && self.z.abs() < 0.0000001
    }

    pub fn is_valid(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    pub fn is_valid_rot(&self) -> bool {
        const VALID_RANGE: std::ops::Range<f32> = 0.0..361.0;

        self.is_valid()
            && VALID_RANGE.contains(&self.x)
            && VALID_RANGE.contains(&self.y)
            && VALID_RANGE.contains(&self.z)
    }
}

impl From<(f32, f32, f32)> for VectorBin {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Self { x, y, z }
    }
}

impl From<VectorBin> for (f32, f32, f32) {
    fn from(value: VectorBin) -> Self {
        (value.x, value.y, value.z)
    }
}

impl From<crate::normal::Vector> for VectorBin {
    fn from(value: crate::normal::Vector) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<VectorBin> for crate::normal::Vector {
    fn from(value: VectorBin) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl std::fmt::Display for VectorBin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector3({},{},{})", self.x, self.y, self.z)
    }
}

impl PlayerDataBin {
    pub fn next_guid(&mut self) -> u64 {
        self.guid_counter += 1;
        ((self.uid as u64) << 32) | self.guid_counter as u64
    }
}

impl AvatarBin {
    pub fn get_scene_reliquary_info_list(&self) -> Vec<crate::normal::SceneReliquaryInfo> {
        let mut result = vec![];
        self.equip_map.iter().for_each(|(_, item)| {
            let Some(item_bin::Detail::Equip(ref equip)) = item.detail else {
                return;
            };
            let Some(equip_bin::Detail::Reliquary(ref reliquary)) = equip.detail else {
                return;
            };

            result.push(crate::normal::SceneReliquaryInfo {
                guid: item.guid.clone(),
                level: reliquary.level,
                item_id: item.item_id,
                promote_level: 0,
            });
        });
        result
    }
}

impl PlayerItemCompBin {
    pub fn has_material(&self, item_id: u32) -> Option<u64> {
        let Some(ref pack_store) = self.pack_store else {
            return None;
        };
        for (guid, mut item_bin) in pack_store.item_map.iter() {
            if item_bin.item_id == item_id {
                return Some((*guid));
            }
        }
        None
    }

    pub fn add_item(&mut self, guid: u64, item: ItemBin) -> Option<ItemBin> {
        if let Some(store) = self.pack_store.as_mut() {
            return store.item_map.insert(guid, item);
        }
        None
    }

    pub fn remove_item(&mut self, guid: &u64) -> Option<ItemBin> {
        if let Some(store) = self.pack_store.as_mut() {
            return store.item_map.remove(guid);
        }
        None
    }

    pub fn get_item(&self, guid: &u64) -> Option<&ItemBin> {
        self.pack_store
            .as_ref()
            .and_then(|store| store.item_map.get(guid))
    }

    pub fn get_mut_item(&mut self, guid: &u64) -> Option<&mut ItemBin> {
        let Some(ref mut pack_store) = self.pack_store else {
            return None;
        };
        pack_store.item_map.get_mut(guid)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&u64, &ItemBin)> {
        self.pack_store
            .as_ref()
            .map(|store| store.item_map.iter())
            .into_iter()
            .flatten()
    }
}

impl ItemBin {
    pub fn to_normal_proto(&self) -> Option<crate::normal::Item> {
        match self.detail {
            Some(item_bin::Detail::Material(ref material)) => Some(crate::normal::Item {
                item_id: self.item_id,
                guid: self.guid,
                detail: Some(crate::normal::item::Detail::Material(
                    crate::normal::Material {
                        delete_info: None,
                        count: material.count,
                    },
                )),
            }),
            Some(item_bin::Detail::Equip(ref equip)) => match equip.detail {
                Some(equip_bin::Detail::Reliquary(ref reliquary)) => Some(crate::normal::Item {
                    item_id: self.item_id,
                    guid: self.guid,
                    detail: Some(crate::normal::item::Detail::Equip(crate::normal::Equip {
                        is_locked: equip.is_locked,
                        detail: Some(crate::normal::equip::Detail::Reliquary(
                            crate::normal::Reliquary {
                                level: reliquary.level,
                                exp: reliquary.exp,
                                main_prop_id: reliquary.main_prop_id,
                                append_prop_id_list: reliquary.append_prop_id_list.clone(),
                                ..Default::default()
                            },
                        )),
                    })),
                }),
                Some(equip_bin::Detail::Weapon(ref weapon)) => Some(crate::normal::Item {
                    item_id: self.item_id,
                    guid: self.guid,
                    detail: Some(crate::normal::item::Detail::Equip(crate::normal::Equip {
                        is_locked: equip.is_locked,
                        detail: Some(crate::normal::equip::Detail::Weapon(
                            crate::normal::Weapon {
                                level: weapon.level,
                                exp: weapon.exp,
                                promote_level: weapon.promote_level,
                                affix_map: weapon.affix_map.clone(),
                                ..Default::default()
                            },
                        )),
                    })),
                }),
                _ => None,
            },

            Some(item_bin::Detail::Furniture(ref furniture)) => Some(crate::normal::Item {
                item_id: self.item_id,
                guid: self.guid,
                detail: Some(crate::normal::item::Detail::Furniture(
                    crate::normal::Furniture {
                        count: furniture.count,
                    },
                )),
            }),
            _ => None,
        }
    }
}
