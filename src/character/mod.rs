mod player;

use bevy::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<player::PlayerAnimations>()
            .add_startup_system_to_stage(StartupStage::PreStartup, player::create_animate)
            .add_startup_system(player::spawn_player);
    }
}
