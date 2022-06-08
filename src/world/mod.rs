use crate::App;
use bevy::prelude::Plugin;
use bevy_ecs_ldtk::prelude::RegisterLdtkObjects;
use bevy_ecs_ldtk::{LdtkPlugin, LevelSelection};

mod map;

pub(crate) struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LevelSelection::Index(0))
            .add_startup_system(map::setup)
            .add_system(map::spawn_wall_collision)
            .add_system(map::camera_fit_inside_current_level)
            .register_ldtk_int_cell_for_layer::<map::WallBundle>("AutoCliffTiles", 1);
    }
}
