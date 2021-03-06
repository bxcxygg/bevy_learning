mod animation_tree;
mod character;
mod common;
mod components;
mod world;
mod ysort;

use animation_tree::AnimationTreePlugin;
use benimator::AnimationPlugin;
use bevy::{prelude::*, winit::WinitSettings};
use bevy_rapier2d::prelude::*;
use character::CharacterPlugin;
use components::InputVector;
use world::WorldPlugin;
use ysort::YSortPlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(WinitSettings::game())
        .insert_resource(WindowDescriptor {
            title: "RPG".to_string(),
            width: 320. * common::SCALE,
            height: 180. * common::SCALE,
            resizable: true,
            ..default()
        })
        .add_plugins(DefaultPlugins);

    #[cfg(feature = "editor_window")]
    {
        use bevy_editor_pls::EditorPlugin;

        app.add_plugin(EditorPlugin)
            .add_plugin(RapierDebugRenderPlugin::default());
    }

    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(YSortPlugin)
        .add_plugin(AnimationPlugin::default())
        .add_plugin(AnimationTreePlugin)
        .add_plugin(GamePlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(CharacterPlugin)
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_startup_system(set_gravity)
            .register_type::<InputVector>();
    }
}

fn spawn_camera(mut commands: Commands) {
    // Camera
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Name::new("Level Camera"))
        .insert(components::LevelCamera);
}

fn set_gravity(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}
