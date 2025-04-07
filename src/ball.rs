use std::time::Duration;

use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::{
    ActiveCollisionTypes, ActiveEvents, Ccd, CoefficientCombineRule, Collider, CollisionEvent,
    Damping, Friction, GravityScale, LockedAxes, Restitution, RigidBody, Velocity,
};
use rand::Rng;

use crate::{
    app_state::AppState, asset_loading::GameImageAssets, blocks::BlockType,
    particles::BoxParticlesEvent,
};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            // TODO verify if we need to do any ordering here..
            FixedUpdate,
            process_collisions.run_if(in_state(AppState::Game)),
        )
        .add_systems(Update, spawn_trail.run_if(in_state(AppState::Game)));
    }
}

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
#[require(Velocity)]
pub struct PreviousVelocity {
    pub linvel: Vec2,
}

impl PreviousVelocity {
    pub fn zero() -> Self {
        Self { linvel: Vec2::ZERO }
    }
}

#[derive(Component, Debug)]
pub struct CollectedResources {
    pub counts: HashMap<BlockType, u32>,
}

impl CollectedResources {
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    pub fn add(&mut self, block_type: BlockType) {
        *self.counts.entry(block_type).or_insert(0) += 1;
    }

    pub fn clear(&mut self) {
        self.counts.clear();
    }

    pub fn combine(&mut self, other: &Self) {
        for (block_type, count) in &other.counts {
            *self.counts.entry(*block_type).or_insert(0) += count;
        }
    }
}

pub fn spawn_ball(mut commands: Commands, transform: Transform, assets: Res<GameImageAssets>) {
    let mut rng = rand::rng();
    commands.spawn((
        Ball,
        // Sprite::from_color(Color::srgb(0.5, 0.5 as f32, 0.5), Vec2 { x: 10.0, y: 10.0 }),
        Sprite {
            image: assets.ball.clone(),
            custom_size: Some(Vec2::new(10.0, 10.0)),
            ..Default::default()
        },
        Transform::from_xyz(transform.translation.x, transform.translation.y - 5.0, 0.0),
        Collider::ball(5.0),
        RigidBody::Dynamic,
        GravityScale(0.0),
        Friction::coefficient(0.0),
        Restitution::coefficient(1.0),
        // Friction {
        //     coefficient: 0.0,
        //     combine_rule: CoefficientCombineRule::Min,
        // },
        // Restitution {
        //     coefficient: 1.0,
        //     combine_rule: CoefficientCombineRule::Max,
        // },
        Damping {
            linear_damping: 0.0,
            angular_damping: 1.0,
        },
        // LockedAxes::ROTATION_LOCKED_Z,
        (
            ActiveCollisionTypes::all(),
            ActiveEvents::COLLISION_EVENTS,
            Ccd::enabled(),
        ),
        StateScoped(AppState::Game),
        Name::new("Ball"),
        Velocity::linear(
            transform
                .rotation
                .mul_vec3(Vec3::new(rng.random_range(-10.0..10.0), -100.0, 0.0))
                .truncate(),
        ),
        PreviousVelocity::zero(),
        CollectedResources::new(),
    ));
}

fn process_collisions(
    mut query: Query<
        (
            Entity,
            &mut Velocity,
            &mut PreviousVelocity,
            &CollectedResources,
        ),
        With<Ball>,
    >,
) {
    for (entity, mut velocity, mut previous_velocity, collected_resources) in query.iter_mut() {
        // Check if velocity has changed (can't use Changed<Velocity> since rapier updates it every time)
        if velocity.linvel == previous_velocity.linvel {
            continue;
        }

        println!("Collected resources: {:?}", collected_resources);

        // Velocity has changed (collision) so lets check if we need to modify it
        let mut new_vel = velocity.linvel;
        if new_vel.length_squared() < previous_velocity.linvel.length_squared() {
            // balls are not allowed to slow down
            new_vel = new_vel.normalize() * previous_velocity.linvel.length();
        }
        // TODO handle speed increasing here instead of using Restitution

        const MAX_VELOCITY: f32 = 500.0;
        if new_vel.length_squared() > MAX_VELOCITY * MAX_VELOCITY {
            // clamp the speed
            // TODO move this number to a component
            new_vel = new_vel.normalize() * MAX_VELOCITY;
        }

        if new_vel != velocity.linvel {
            velocity.linvel = new_vel;
        }
        previous_velocity.linvel = new_vel;
    }
}

fn spawn_trail(
    mut commands: Commands,
    ball_query: Query<(Entity, &Transform, &Velocity, &CollectedResources), With<Ball>>,
) {
    let mut rng = rand::rng();
    for (entity, transform, velocity, collected_resources) in ball_query.iter() {
        for (block_type, count) in &collected_resources.counts {
            let num_spawns: u32;
            match count {
                0 => continue,
                1..=10 => {
                    if rng.random_range(0..10) < *count {
                        num_spawns = 1;
                    } else {
                        continue;
                    }
                }
                11..=50 => num_spawns = 2,
                _ => num_spawns = 3,
            }
            for _ in 0..num_spawns {
                commands.trigger(BoxParticlesEvent {
                    init_position: transform.translation.truncate()
                        + Vec2::new(rng.random_range(-5.0..5.0), rng.random_range(-5.0..5.0)),
                    target_position: transform.translation.truncate()
                        - velocity.linvel.normalize().rotate(Vec2::from_angle(
                            rng.random_range(-25.0_f32.to_radians()..25.0_f32.to_radians()),
                        )),
                    z_index: -5.0,
                    color: block_type.colour(),
                    size: Vec2::new(3., 3.),
                    target_scale: Vec3::ONE,
                    duration: Duration::from_millis(500),
                });
            }
        }
    }
}
