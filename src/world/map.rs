use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkWorldBundle;

pub(crate) fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("World/world.ldtk"),
        transform: Transform::from_xyz(-250., -150., 0.),
        ..Default::default()
    });
}
