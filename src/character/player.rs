use std::time::Duration;

use benimator::{Play, SpriteSheetAnimation};
use bevy::{prelude::*, reflect::Reflect, utils::HashMap};
use bevy_ecs_ldtk::prelude::*;
use bevy_input_actionmap::InputMap;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::{
    animation_tree::{Animation, AnimationNode, AnimationNodeMachine, AnimationTree},
    components::InputVector,
    ysort::YSort,
};

#[derive(Default, Eq, PartialEq, Clone, Reflect, Inspectable)]
#[reflect_value(PartialEq)]
pub(crate) enum PlayerState {
    #[default]
    MOVE,
    ATTACK,
    ROLL,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum Action {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    ATTACK,
    ROLL,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub(crate) struct Player {
    state: PlayerState,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    name: Name,
    player: Player,
    ysort: YSort,
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
}

impl LdtkEntity for PlayerBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        PlayerBundle {
            sprite_sheet: SpriteSheetBundle {
                texture_atlas: texture_atlases.add(TextureAtlas::from_grid(
                    asset_server.load("Player/Player.png"),
                    Vec2::new(64.0, 64.0),
                    60,
                    1,
                )),
                transform: Transform::from_xyz(
                    entity_instance.px.x as f32,
                    entity_instance.px.y as f32,
                    0.,
                ),
                visibility: Visibility { is_visible: true },
                ..default()
            },
            name: Name::from("Player"),
            player: Player::default(),
            ysort: YSort(4),
        }
    }
}

/// Create Player Animation Tree.
pub(crate) fn create_animate(assets: &mut ResMut<Assets<SpriteSheetAnimation>>) -> AnimationTree {
    let run_right = Animation::from(assets.add(SpriteSheetAnimation::from_range(
        0..=5,
        Duration::from_secs_f32(0.1),
    )));
    let run_up = Animation::from(assets.add(SpriteSheetAnimation::from_range(
        6..=11,
        Duration::from_secs_f32(0.1),
    )));
    let run_left = Animation::from(assets.add(SpriteSheetAnimation::from_range(
        12..=17,
        Duration::from_secs_f32(0.1),
    )));
    let run_down = Animation::from(assets.add(SpriteSheetAnimation::from_range(
        18..=23,
        Duration::from_secs_f32(0.1),
    )));
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
    let idle_right = Animation::from(assets.add(SpriteSheetAnimation::from_range(
        0..=0,
        Duration::from_secs_f32(0.1),
    )));
    let idle_up = Animation::from(assets.add(SpriteSheetAnimation::from_range(
        6..=6,
        Duration::from_secs_f32(0.1),
    )));
    let idle_left = Animation::from(assets.add(SpriteSheetAnimation::from_range(
        12..=12,
        Duration::from_secs_f32(0.1),
    )));
    let idle_down = Animation::from(assets.add(SpriteSheetAnimation::from_range(
        18..=18,
        Duration::from_secs_f32(0.1),
    )));

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

pub(crate) fn setup(mut input: ResMut<InputMap<Action>>) {
    // Binding button.
    input
        .bind(Action::UP, KeyCode::Up)
        .bind(Action::UP, KeyCode::W)
        .bind(Action::LEFT, KeyCode::Left)
        .bind(Action::LEFT, KeyCode::A)
        .bind(Action::DOWN, KeyCode::Down)
        .bind(Action::DOWN, KeyCode::S)
        .bind(Action::RIGHT, KeyCode::Right)
        .bind(Action::RIGHT, KeyCode::D)
        .bind(Action::ATTACK, KeyCode::J)
        .bind(Action::ROLL, KeyCode::K);
}

pub(crate) fn spawn_player(
    mut commands: Commands,
    mut assets: ResMut<Assets<SpriteSheetAnimation>>,
    query: Query<(Entity, &Transform), Added<Player>>,
) {
    query.for_each(|(entity, transform)| {
        // Spawn the player.
        commands
            .entity(entity)
            .insert_bundle((
                InputVector::default(),
                // player animation.
                create_animate(&mut assets),
            ))
            // spawn player rigid body bundle.
            .insert_bundle((
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED,
                Velocity::default(),
            ))
            // spawn player collision bundle as children entity.
            .with_children(|children| {
                children
                    .spawn_bundle((Name::from("PlayerCollider"), Collider::capsule_x(4., 4.)))
                    .insert_bundle(TransformBundle::from(Transform::from_xyz(
                        -transform.translation.x,
                        -transform.translation.y - 8.,
                        transform.translation.z,
                    )));
            });
    });
}

pub(crate) fn movement(
    keyboard_input: Res<InputMap<Action>>,
    mut query: Query<(
        &mut AnimationTree,
        &mut InputVector,
        &mut Velocity,
        &mut Player,
    )>,
) {
    for (mut animation, mut vector, mut velocity, player) in query.iter_mut() {
        if player.state == PlayerState::MOVE {
            let input_vector = Vec2::new(
                keyboard_input.strength(Action::RIGHT) - keyboard_input.strength(Action::LEFT),
                keyboard_input.strength(Action::UP) - keyboard_input.strength(Action::DOWN),
            );

            if input_vector != Vec2::ZERO {
                vector.0 = input_vector.normalize();

                animation.travel(vector.0, "run".to_string());
                velocity.linvel = vector.0 * 80.;
            } else {
                animation.travel(vector.0, "idle".to_string());
                velocity.linvel = input_vector;
            }
        }
    }
}

pub(crate) fn attack(
    keyboard_input: Res<InputMap<Action>>,
    mut query: Query<(&mut AnimationTree, &InputVector, &mut Velocity, &mut Player)>,
) {
    for (mut animation, input_vector, mut velocity, mut player) in query.iter_mut() {
        if player.state == PlayerState::MOVE && keyboard_input.just_active(Action::ATTACK) {
            velocity.linvel = Vec2::ZERO;

            animation.travel(input_vector.0, "attack".to_string());
            player.state = PlayerState::ATTACK;
        }
    }
}

pub(crate) fn roll(
    keyboard_input: Res<InputMap<Action>>,
    mut query: Query<(&mut AnimationTree, &InputVector, &mut Velocity, &mut Player)>,
) {
    for (mut animation, input_vector, mut velocity, mut player) in query.iter_mut() {
        if player.state == PlayerState::MOVE && keyboard_input.just_active(Action::ROLL) {
            velocity.linvel = input_vector.0 * 120.;

            animation.travel(input_vector.0, "roll".to_string());
            player.state = PlayerState::ROLL;
        }
    }
}

pub(crate) fn state(
    removed: RemovedComponents<Play>,
    mut query: Query<(&mut Velocity, &AnimationTree, &mut Player)>,
) {
    for entity in removed.iter() {
        for (mut velocity, animation_tree, mut player) in query.get_mut(entity) {
            match animation_tree.finished.as_str() {
                "attack" => {
                    player.state = PlayerState::MOVE;
                }
                "roll" => {
                    velocity.linvel = Vec2::ZERO;
                    player.state = PlayerState::MOVE;
                }
                _ => {}
            }
        }
    }
}
