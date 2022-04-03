use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::{
    ldtk_pixel_coords_to_translation_pivoted, sprite_sheet_bundle_from_entity_info,
};
use heron::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: CollisionShape,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: RotationConstraints,
    pub physic_material: PhysicMaterial,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Climber {
    pub climbing: bool,
    pub intersecting_climbables: HashSet<Entity>,
}

#[derive(Clone, Default, Bundle)]
pub struct PlayerBundle {
    pub name: Name,
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    #[bundle]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    pub worldly: Worldly,
    pub climber: Climber,
}

impl LdtkEntity for PlayerBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Self {
        Self {
            name: Name::new("Player"),
            sprite_bundle: SpriteBundle::default(),
            collider_bundle: ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(6., 14., 0.),
                    border_radius: None,
                },
                rigid_body: RigidBody::Dynamic,
                rotation_constraints: RotationConstraints::lock(),
                ..Default::default()
            },
            player: Player::default(),
            worldly: Worldly::from_entity_info(entity_instance, layer_instance),
            climber: Climber::default(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Climbable;

#[derive(Clone, Debug, Default, Bundle)]
pub struct LadderBundle {
    #[bundle]
    pub collider_bundle: ColliderBundle,
    pub climbable: Climbable,
}

impl LdtkIntCell for LadderBundle {
    fn bundle_int_cell(int_grid_cell: IntGridCell, _: &LayerInstance) -> Self {
        let rotation_constraints = RotationConstraints::lock();
        let mut collider = ColliderBundle::default();

        if int_grid_cell.value == 2 {
            collider = ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(8., 8., 0.),
                    border_radius: None,
                },
                rigid_body: RigidBody::Sensor,
                rotation_constraints,
                ..Default::default()
            };
        }

        Self {
            collider_bundle: collider,
            climbable: Climbable::default(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Enemy;

#[derive(Clone, PartialEq, Debug, Default, Component)]
pub struct Patrol {
    pub points: Vec<Vec2>,
    pub index: usize,
    pub forward: bool,
}

impl LdtkEntity for Patrol {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Patrol {
        let mut points = Vec::new();
        points.push(ldtk_pixel_coords_to_translation_pivoted(
            entity_instance.px,
            layer_instance.c_hei * layer_instance.grid_size,
            IVec2::new(entity_instance.width, entity_instance.height),
            entity_instance.pivot,
        ));

        let ldtk_patrol = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == "patrol".to_string())
            .unwrap();
        if let FieldValue::Points(ldtk_points) = &ldtk_patrol.value {
            for ldtk_point in ldtk_points {
                if let Some(ldtk_point) = ldtk_point {
                    // The +1 is necessary here due to the pivot of the entities in the sample
                    // file.
                    // The patrols set up in the file look flat and grounded,
                    // but technically they're not if you consider the pivot,
                    // which is at the bottom-center for the skulls.
                    let pixel_coords = (ldtk_point.as_vec2() + Vec2::new(0.5, 1.))
                        * Vec2::splat(layer_instance.grid_size as f32);

                    points.push(ldtk_pixel_coords_to_translation_pivoted(
                        pixel_coords.as_ivec2(),
                        layer_instance.c_hei * layer_instance.grid_size,
                        IVec2::new(entity_instance.width, entity_instance.height),
                        entity_instance.pivot,
                    ));
                }
            }
        }

        Patrol {
            points,
            index: 1,
            forward: true,
        }
    }
}

#[derive(Clone, Default, Bundle)]
pub struct MobBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[bundle]
    pub collider_bundle: ColliderBundle,
    pub enemy: Enemy,
    pub patrol: Patrol,
}

impl LdtkEntity for MobBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        tileset: Option<&Handle<Image>>,
        tileset_definition: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        Self {
            sprite_sheet_bundle: sprite_sheet_bundle_from_entity_info(
                entity_instance,
                tileset,
                tileset_definition,
                texture_atlases,
            ),
            collider_bundle: ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(5., 5., 0.),
                    border_radius: None,
                },
                rigid_body: RigidBody::KinematicVelocityBased,
                rotation_constraints: RotationConstraints::lock(),
                ..Default::default()
            },
            enemy: Enemy::default(),
            patrol: Patrol::bundle_entity(
                entity_instance,
                layer_instance,
                tileset,
                tileset_definition,
                asset_server,
                texture_atlases,
            ),
        }
    }
}

#[derive(Clone, Default, Bundle)]
pub struct ChestBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[bundle]
    pub collider_bundle: ColliderBundle,
}

impl LdtkEntity for ChestBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        tileset: Option<&Handle<Image>>,
        tileset_definition: Option<&TilesetDefinition>,
        _: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        Self {
            sprite_sheet_bundle: sprite_sheet_bundle_from_entity_info(
                entity_instance,
                tileset,
                tileset_definition,
                texture_atlases,
            ),
            collider_bundle: ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(8., 8., 0.),
                    border_radius: None,
                },
                rigid_body: RigidBody::Dynamic,
                rotation_constraints: RotationConstraints::lock(),
                physic_material: PhysicMaterial {
                    friction: 0.5,
                    density: 15.0,
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}
