pub(crate) mod player;

use bevy::prelude::*;
use bevy_input_actionmap::ActionPlugin;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "editor_window")]
        {
            use bevy_inspector_egui::RegisterInspectable;

            app.register_inspectable::<player::Player>();
        }
        app.add_plugin(ActionPlugin::<player::Action>::default())
            .add_startup_system(player::setup)
            .add_startup_system(player::spawn_player)
            .add_system_to_stage(CoreStage::Update, player::movement)
            .add_system_to_stage(CoreStage::Update, player::attack)
            .add_system_to_stage(CoreStage::Update, player::roll)
            .add_system_to_stage(CoreStage::PostUpdate, player::state);
    }
}
