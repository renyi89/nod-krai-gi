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

impl PlayerItemCompBin {
    pub fn add_item(&mut self, guid: u64, item: ItemBin) {
        if let Some(store) = self.pack_store.as_mut() {
            store.item_map.insert(guid, item);
        }
    }

    pub fn get_item(&self, guid: &u64) -> Option<&ItemBin> {
        self.pack_store
            .as_ref()
            .and_then(|store| store.item_map.get(guid))
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
            Some(item_bin::Detail::Equip(ref equip)) => match equip.detail {
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
                Some(equip_bin::Detail::Reliquary(ref weapon)) => None,
                _ => None,
            },
            Some(item_bin::Detail::Material(ref material)) => None,
            _ => None,
        }
    }
}
