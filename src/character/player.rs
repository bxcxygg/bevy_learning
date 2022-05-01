use benimator::{Play, SpriteSheetAnimation};
use bevy::prelude::*;
use std::time::Duration;

#[derive(Default)]
pub(crate) struct PlayerAnimations {
    run_right: Handle<SpriteSheetAnimation>,
    run_up: Handle<SpriteSheetAnimation>,
    run_left: Handle<SpriteSheetAnimation>,
    run_down: Handle<SpriteSheetAnimation>,
    attack_right: Handle<SpriteSheetAnimation>,
    attack_up: Handle<SpriteSheetAnimation>,
    attack_left: Handle<SpriteSheetAnimation>,
    attack_down: Handle<SpriteSheetAnimation>,
    roll_right: Handle<SpriteSheetAnimation>,
    roll_up: Handle<SpriteSheetAnimation>,
    roll_left: Handle<SpriteSheetAnimation>,
    roll_down: Handle<SpriteSheetAnimation>,
    idle: Handle<SpriteSheetAnimation>,
}

#[derive(Component, Default)]
struct Player;

pub(crate) fn create_animate(
    mut handles: ResMut<PlayerAnimations>,
    mut assets: ResMut<Assets<SpriteSheetAnimation>>,
) {
    handles.run_right = assets.add(SpriteSheetAnimation::from_range(
        0..=5,
        Duration::from_secs_f32(0.1),
    ));
    handles.run_up = assets.add(SpriteSheetAnimation::from_range(
        6..=11,
        Duration::from_secs_f32(0.1),
    ));
    handles.run_left = assets.add(SpriteSheetAnimation::from_range(
        12..=17,
        Duration::from_secs_f32(0.1),
    ));
    handles.run_down = assets.add(SpriteSheetAnimation::from_range(
        18..=23,
        Duration::from_secs_f32(0.1),
    ));
    handles.attack_right = assets.add(SpriteSheetAnimation::from_range(
        24..=27,
        Duration::from_secs_f32(0.1),
    ));
    handles.attack_up = assets.add(SpriteSheetAnimation::from_range(
        28..=31,
        Duration::from_secs_f32(0.1),
    ));
    handles.attack_left = assets.add(SpriteSheetAnimation::from_range(
        32..=35,
        Duration::from_secs_f32(0.1),
    ));
    handles.attack_down = assets.add(SpriteSheetAnimation::from_range(
        36..=39,
        Duration::from_secs_f32(0.1),
    ));
    handles.roll_right = assets.add(SpriteSheetAnimation::from_range(
        40..=44,
        Duration::from_secs_f32(0.1),
    ));
    handles.roll_up = assets.add(SpriteSheetAnimation::from_range(
        45..=49,
        Duration::from_secs_f32(0.1),
    ));
    handles.roll_left = assets.add(SpriteSheetAnimation::from_range(
        50..=54,
        Duration::from_secs_f32(0.1),
    ));
    handles.roll_left = assets.add(SpriteSheetAnimation::from_range(
        55..=59,
        Duration::from_secs_f32(0.1),
    ));
    handles.idle = assets.add(SpriteSheetAnimation::from_range(
        59..=59,
        Duration::from_secs_f32(0.1),
    ));
}

pub(crate) fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    animations: Res<PlayerAnimations>,
) {
    // Spawn the player.
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("Player/Player.png"),
                Vec2::new(64.0, 64.0),
                60,
                1,
            )),
            transform: Transform::from_scale(Vec3::splat(4.0)),
            ..default()
        })
        .insert(Player)
        .insert(animations.idle.clone())
        .insert(Play)
        .insert(Name::from("Player"));
}
