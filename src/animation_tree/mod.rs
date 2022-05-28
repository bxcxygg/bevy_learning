use crate::App;
use bevy::prelude::*;

mod animation_tree;

pub use animation_tree::*;

pub struct AnimationTreePlugin;

impl Plugin for AnimationTreePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, create_animation)
            .add_system_to_stage(CoreStage::PostUpdate, next_animation)
            .add_system_to_stage(CoreStage::PostUpdate, animation_tree);
    }

    fn name(&self) -> &str {
        "animation_tree"
    }
}
