use crate::App;
use bevy::prelude::Plugin;
use bevy_ecs_ldtk::{LdtkPlugin, LevelSelection};

mod map;

pub(crate) struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .add_startup_system(map::setup)
            .insert_resource(LevelSelection::Index(0));
    }
}
