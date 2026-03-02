use bevy_app::prelude::*;

mod equip;
mod item;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, equip::change_avatar_equip)
            .add_systems(Update, item::item_command_handler)
            .add_systems(Update, item::item_add_handler)
            .add_systems(Update, item::item_drop_handler)
            .add_systems(Update, item::update_player_store);
    }
}
