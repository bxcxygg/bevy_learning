mod player;

use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<player::Player>()
            .add_startup_system(player::spawn_player)
            .add_system(player::state)
            .add_system(player::movement)
            .add_system(player::attack)
            .add_system(player::roll);
    }
}
