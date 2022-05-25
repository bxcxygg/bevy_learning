use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Default, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Default, Deref, DerefMut, Inspectable, Debug)]
pub struct InputVector(pub Vec2);
