use crate::Vec2;
use benimator::{Play, SpriteSheetAnimation, SpriteSheetAnimationState};
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Clone, Default)]
pub struct Animation {
    handle: Handle<SpriteSheetAnimation>,
}

impl Animation {
    pub fn from(animation: Handle<SpriteSheetAnimation>) -> Self {
        Animation { handle: animation }
    }
}

#[derive(Clone)]
pub struct AnimationNode {
    animation: Animation,
    point: Vec2,
}

impl AnimationNode {
    pub fn from(animation: Animation, point: Vec2) -> Self {
        AnimationNode { animation, point }
    }
}

#[derive(Clone)]
pub struct AnimationNodeMachine {
    points: Vec<AnimationNode>,
    next: Option<String>,
}

impl AnimationNodeMachine {
    pub fn from(points: Vec<AnimationNode>) -> Self {
        AnimationNodeMachine { points, next: None }
    }

    pub fn next(mut self, next: String) -> Self {
        self.next = Some(next);
        self
    }

    pub(crate) fn play(&self, point: Vec2) -> Animation {
        let mut next_animation = Animation::default();
        let mut distance: f32 = f32::MAX;
        for p in self.points.iter() {
            let dis = point.distance(p.point);
            if dis < distance {
                distance = dis;
                next_animation = p.animation.clone();
            }
        }

        next_animation
    }
}

#[derive(Component, Clone)]
pub struct AnimationTree {
    current: String,
    next: String,
    pub finished: String,
    point: Vec2,
    pub nodes: HashMap<String, AnimationNodeMachine>,
}

impl Default for AnimationTree {
    fn default() -> Self {
        AnimationTree {
            current: "".to_string(),
            next: "".to_string(),
            finished: "".to_string(),
            point: Vec2::ZERO,
            nodes: HashMap::new(),
        }
    }
}

impl AnimationTree {
    pub fn from(nodes: HashMap<String, AnimationNodeMachine>, default: String) -> Self {
        AnimationTree {
            current: default,
            next: "".to_string(),
            finished: "".to_string(),
            point: Vec2::ZERO,
            nodes,
        }
    }

    pub fn travel(&mut self, point: Vec2, to_node: String) {
        let next_node = self.nodes.get(&to_node);
        if let Some(_) = next_node {
            self.next = to_node;
            self.point = point;
        }
    }
}

pub(crate) fn create_animation(
    mut commands: Commands,
    mut animation: Query<
        (Entity, &mut AnimationTree),
        (Without<Play>, Without<Handle<SpriteSheetAnimation>>),
    >,
) {
    for (entity, animation_tree) in animation.iter_mut() {
        if let Some(animation) = animation_tree.nodes.get(&animation_tree.current) {
            let animation = animation.play(animation_tree.point);
            commands
                .entity(entity)
                .insert(animation.handle)
                .insert(Play);
        }
    }
}

pub(crate) fn next_animation(
    mut query: Query<
        (
            &mut Handle<SpriteSheetAnimation>,
            &mut SpriteSheetAnimationState,
            &mut AnimationTree,
        ),
        With<Play>,
    >,
) {
    for (mut animation, mut animation_state, mut animation_tree) in query.iter_mut() {
        let next_node = animation_tree.nodes.get(&animation_tree.next);
        if let Some(next_node) = next_node {
            let next_animation = next_node.play(animation_tree.point);
            if *animation != next_animation.handle {
                *animation = next_animation.handle;
                animation_state.reset();
            }
            animation_tree.current = animation_tree.next.clone();
            animation_tree.next = "".to_string();
        }
    }
}

pub(crate) fn animation_tree(
    mut commands: Commands,
    removed: RemovedComponents<Play>,
    mut query: Query<(&mut Handle<SpriteSheetAnimation>, &mut AnimationTree)>,
) {
    for entity in removed.iter() {
        if let Ok((mut animation, mut animation_tree)) = query.get_mut(entity) {
            animation_tree.finished = animation_tree.current.clone();
            let next_node = animation_tree.nodes.get(&animation_tree.next);
            match next_node {
                Some(next_node) => {
                    let next_animation = next_node.play(animation_tree.point);
                    if *animation != next_animation.handle {
                        *animation = next_animation.handle;
                    }
                    animation_tree.current = animation_tree.next.clone();
                    animation_tree.next = "".to_string();
                    commands.entity(entity).insert(Play);
                }
                None => {
                    if let Some(current_node) = animation_tree.nodes.get(&animation_tree.current) {
                        match &current_node.next {
                            Some(next) => {
                                let next_node = animation_tree.nodes.get(next);
                                if let Some(next_node) = next_node {
                                    let next_animation = next_node.play(animation_tree.point);
                                    if *animation != next_animation.handle {
                                        *animation = next_animation.handle;
                                    }
                                    animation_tree.current = animation_tree.next.clone();
                                    animation_tree.next = "".to_string();
                                }
                            }
                            None => {}
                        }
                        commands.entity(entity).insert(Play);
                    }
                }
            }
        }
    }
}
