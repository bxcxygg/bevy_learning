mod character;
mod components;

use crate::character::CharacterPlugin;
use benimator::AnimationPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

/// This example illustrates how to create a custom diagnostic
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(AnimationPlugin::default())
        .add_plugin(CharacterPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
