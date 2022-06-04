use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Default, Deref, DerefMut, Debug, Reflect, Inspectable)]
pub struct InputVector(pub Vec2);
