use bevy::prelude::*;

mod animation_tree;

pub use animation_tree::*;

pub struct AnimationTreePlugin;

impl Plugin for AnimationTreePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, create_animation)
            .add_system_to_stage(CoreStage::Update, next_animation.label("animation_next"))
            .add_system_to_stage(
                CoreStage::Update,
                animation_tree
                    .label("animation_tree")
                    .after("animation_next"),
            );
    }

    fn name(&self) -> &str {
        "animation_tree"
    }
}
