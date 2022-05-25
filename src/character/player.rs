use crate::animation_tree::*;
use crate::components::{AnimationTimer, InputVector};
use benimator::SpriteSheetAnimation;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

const RUN_ANIMATION_LEN: f32 = 0.6;
const ROLL_ANIMATION_LEN: f32 = 0.5;
const ATTACK_ANIMATION_LEN: f32 = 0.4;
const IDLE_ANIMATION_LEN: f32 = 0.1;

#[derive(Default, Eq, PartialEq, Clone, Inspectable)]
enum PlayerState {
    #[default]
    MOVE,
    ATTACK,
    ROLL,
}

#[derive(Component, Default, Inspectable)]
pub(crate) struct Player {
    state: PlayerState,
}

pub(crate) fn create_animate(mut assets: ResMut<Assets<SpriteSheetAnimation>>) -> AnimationTree {
    let run_right = Animation::from(
        assets.add(SpriteSheetAnimation::from_range(0..=5, Duration::from_secs_f32(0.1)).once()),
    );
    let run_up = Animation::from(
        assets.add(SpriteSheetAnimation::from_range(6..=11, Duration::from_secs_f32(0.1)).once()),
    );
    let run_left = Animation::from(
        assets.add(SpriteSheetAnimation::from_range(12..=17, Duration::from_secs_f32(0.1)).once()),
    );
    let run_down = Animation::from(
        assets.add(SpriteSheetAnimation::from_range(18..=23, Duration::from_secs_f32(0.1)).once()),
    );
    let attack_right = Animation::from(
        assets.add(SpriteSheetAnimation::from_range(24..=27, Duration::from_secs_f32(0.1)).once()),
    );
    let attack_up = Animation::from(
        assets.add(SpriteSheetAnimation::from_range(28..=31, Duration::from_secs_f32(0.1)).once()),
    );
    let attack_left = Animation::from(
        assets.add(SpriteSheetAnimation::from_range(32..=35, Duration::from_secs_f32(0.1)).once()),
    );
    let attack_down = Animation::from(
        assets.add(SpriteSheetAnimation::from_range(36..=39, Duration::from_secs_f32(0.1)).once()),
    );
    let roll_right = Animation::from(
        assets.add(SpriteSheetAnimation::from_range(40..=44, Duration::from_secs_f32(0.1)).once()),
    );
    let roll_up = Animation::from(
        assets.add(SpriteSheetAnimation::from_range(45..=49, Duration::from_secs_f32(0.1)).once()),
    );
    let roll_left = Animation::from(
        assets.add(SpriteSheetAnimation::from_range(50..=54, Duration::from_secs_f32(0.1)).once()),
    );
    let roll_down = Animation::from(
        assets.add(SpriteSheetAnimation::from_range(55..=59, Duration::from_secs_f32(0.1)).once()),
    );
    let idle_right = Animation::from(
        assets.add(SpriteSheetAnimation::from_range(0..=0, Duration::from_secs_f32(0.1)).once()),
    );
    let idle_up = Animation::from(
        assets.add(SpriteSheetAnimation::from_range(6..=6, Duration::from_secs_f32(0.1)).once()),
    );
    let idle_left = Animation::from(
        assets.add(SpriteSheetAnimation::from_range(12..=12, Duration::from_secs_f32(0.1)).once()),
    );
    let idle_down = Animation::from(
        assets.add(SpriteSheetAnimation::from_range(18..=18, Duration::from_secs_f32(0.1)).once()),
    );

    let run = AnimationNodeMachine::from(vec![
        AnimationNode::from(run_down, Vec2::new(0., -1.)),
        AnimationNode::from(run_up, Vec2::new(0., 1.)),
        AnimationNode::from(run_left, Vec2::new(-1.1, 0.)),
        AnimationNode::from(run_right, Vec2::new(1.1, 0.)),
    ]);
    let idle = AnimationNodeMachine::from(vec![
        AnimationNode::from(idle_down, Vec2::new(0., -1.)),
        AnimationNode::from(idle_up, Vec2::new(0., 1.)),
        AnimationNode::from(idle_left, Vec2::new(-1.1, 0.)),
        AnimationNode::from(idle_right, Vec2::new(1.1, 0.)),
    ]);
    let attack = AnimationNodeMachine::from(vec![
        AnimationNode::from(attack_down, Vec2::new(0., -1.)),
        AnimationNode::from(attack_up, Vec2::new(0., 1.)),
        AnimationNode::from(attack_left, Vec2::new(-1.1, 0.)),
        AnimationNode::from(attack_right, Vec2::new(1.1, 0.)),
    ]);
    let roll = AnimationNodeMachine::from(vec![
        AnimationNode::from(roll_down, Vec2::new(0., -1.)),
        AnimationNode::from(roll_up, Vec2::new(0., 1.)),
        AnimationNode::from(roll_left, Vec2::new(-1.1, 0.)),
        AnimationNode::from(roll_right, Vec2::new(1.1, 0.)),
    ]);

    let mut nodes = HashMap::new();
    nodes.insert("run".to_string(), run.next("idle".to_string()));
    nodes.insert("roll".to_string(), roll.next("idle".to_string()));
    nodes.insert("attack".to_string(), attack.next("idle".to_string()));
    nodes.insert("idle".to_string(), idle);

    AnimationTree::from(nodes, "idle".to_string())
}

pub(crate) fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: ResMut<Assets<SpriteSheetAnimation>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Spawn the player.
    commands
        // spawn player sprite bundle.
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("Player/Player.png"),
                Vec2::new(64.0, 64.0),
                60,
                1,
            )),
            visibility: Visibility { is_visible: true },
            ..default()
        })
        .insert_bundle((
            Name::from("Player"),
            Player::default(),
            InputVector::default(),
            // player animation.
            create_animate(assets),
            AnimationTimer(Timer::from_seconds(IDLE_ANIMATION_LEN, false)),
        ))
        // spawn player rigid body bundle.
        .insert_bundle((
            RigidBody::Dynamic,
            // lock the player body rotation.
            LockedAxes::ROTATION_LOCKED,
            Velocity::default(),
            // player acceleration.
            ExternalForce::default(),
            // player friction.
            Damping::default(),
        ))
        // spawn player collision bundle as children entity.
        .with_children(|children| {
            children.spawn_bundle((
                Name::from("PlayerCollider"),
                Collider::capsule_x(4., 4.),
                Transform::from_xyz(0., -8., 0.),
            ));
        });
}

pub(crate) fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut AnimationTree,
        &mut InputVector,
        &mut Velocity,
        &mut AnimationTimer,
        &mut Player,
    )>,
) {
    let (mut animation, mut vector, mut velocity, mut timer, player) = query.single_mut();

    if player.state == PlayerState::MOVE {
        let mut input_vector = Vec2::ZERO;
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            input_vector.x += 1.;
        }
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            input_vector.x -= 1.;
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            input_vector.y += 1.;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            input_vector.y -= 1.;
        }

        if input_vector != Vec2::ZERO {
            vector.0 = input_vector.normalize();

            animation.travel(vector.0, "run".to_string());
            timer
                .0
                .set_duration(Duration::from_secs_f32(RUN_ANIMATION_LEN));
            timer.0.reset();
            velocity.linvel = vector.0 * 80.;
        } else {
            animation.travel(vector.0, "idle".to_string());
            timer
                .0
                .set_duration(Duration::from_secs_f32(IDLE_ANIMATION_LEN));
            timer.0.reset();
            velocity.linvel = input_vector;
        }
    }
}

pub(crate) fn attack(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut AnimationTree,
        &InputVector,
        &mut Velocity,
        &mut AnimationTimer,
        &mut Player,
    )>,
) {
    let (mut animation, input_vector, mut velocity, mut timer, mut player) = query.single_mut();

    if player.state == PlayerState::MOVE && keyboard_input.just_pressed(KeyCode::J) {
        velocity.linvel = Vec2::ZERO;

        animation.travel(input_vector.0, "attack".to_string());

        timer
            .0
            .set_duration(Duration::from_secs_f32(ATTACK_ANIMATION_LEN));
        timer.0.reset();
        player.state = PlayerState::ATTACK;
    }
}

pub(crate) fn roll(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut AnimationTree,
        &InputVector,
        &mut Velocity,
        &mut AnimationTimer,
        &mut Player,
    )>,
) {
    let (mut animation, input_vector, mut velocity, mut timer, mut player) = query.single_mut();

    if player.state == PlayerState::MOVE && keyboard_input.just_pressed(KeyCode::K) {
        velocity.linvel = input_vector.0 * 120.;

        animation.travel(input_vector.0, "roll".to_string());

        timer
            .0
            .set_duration(Duration::from_secs_f32(ROLL_ANIMATION_LEN));
        timer.0.reset();
        player.state = PlayerState::ROLL;
    }
}

pub(crate) fn state(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut AnimationTimer, &mut Player)>,
) {
    for (mut velocity, mut timer, mut player) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() && player.state != PlayerState::MOVE {
            if player.state == PlayerState::ROLL {
                velocity.linvel = Vec2::ZERO;
            }
            player.state = PlayerState::MOVE;
        }
    }
}
