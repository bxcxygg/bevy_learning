pub(crate) mod player;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::RegisterLdtkObjects;
use bevy_input_actionmap::ActionPlugin;
use bevy_inspector_egui::RegisterInspectable;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ActionPlugin::<player::Action>::default())
            .add_startup_system(player::setup)
            .add_system(player::spawn_player)
            .add_system_to_stage(CoreStage::Update, player::movement)
            .add_system_to_stage(CoreStage::Update, player::attack)
            .add_system_to_stage(CoreStage::Update, player::roll)
            .add_system_to_stage(CoreStage::PostUpdate, player::state.after("animation_tree"))
            .register_ldtk_entity::<player::PlayerBundle>("Player")
            .register_type::<player::Player>()
            .register_type::<player::PlayerState>()
            .register_inspectable::<player::PlayerState>();
    }
}
