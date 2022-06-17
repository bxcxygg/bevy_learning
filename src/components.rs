use bevy::prelude::*;

#[derive(Component, Default, Deref, DerefMut, Debug, Reflect)]
#[reflect(Component)]
pub struct InputVector(pub Vec2);

#[derive(Component, Debug)]
pub struct LevelCamera;
