use bevy::prelude::*;
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    prelude::*,
    render::RapierDebugRenderPlugin,
};

use crate::ball::{Ball, PreviousVelocity};
use crate::blocks::{Block, HitPoints};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.0))
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(FixedUpdate, process_collisions);
    }
}

fn process_collisions(
    mut reader: EventReader<CollisionEvent>,
    mut ball_query: Query<
        (
            Entity,
            &mut Velocity,
            &mut PreviousVelocity,
            &Transform,
            &Collider,
        ),
        With<Ball>,
    >,
    mut block_query: Query<
        (Entity, &mut HitPoints, &Transform, &Collider),
        (With<Block>, Without<Ball>),
    >,
    mut commands: Commands,
) {
    for &collision in reader.read() {
        match collision {
            CollisionEvent::Started(lhs, rhs, collision_event_flags) => {
                if let Ok((entity, mut hitpoints, transform, collider)) = block_query.get_mut(lhs) {
                    on_block_hit(
                        hitpoints.as_mut(),
                        transform,
                        collider,
                        entity,
                        &mut commands,
                    );
                } else if let Ok((entity, mut hitpoints, transform, collider)) =
                    block_query.get_mut(rhs)
                {
                    on_block_hit(
                        hitpoints.as_mut(),
                        transform,
                        collider,
                        entity,
                        &mut commands,
                    );
                }

                // handle ball collisions
                if let Ok((entity, mut velocity, mut previous_velocity, transform, collider)) =
                    ball_query.get_mut(lhs)
                {
                    on_ball_hit(
                        entity,
                        &mut velocity,
                        &mut previous_velocity,
                        transform,
                        collider,
                        &mut commands,
                    );
                } else if let Ok((
                    entity,
                    mut velocity,
                    mut previous_velocity,
                    transform,
                    collider,
                )) = ball_query.get_mut(rhs)
                {
                    on_ball_hit(
                        entity,
                        &mut velocity,
                        &mut previous_velocity,
                        transform,
                        collider,
                        &mut commands,
                    );
                }
            }
            _ => {}
        }
    }
}

fn on_block_hit(
    hitpoints: &mut HitPoints,
    transform: &Transform,
    collider: &Collider,
    entity: Entity,
    commands: &mut Commands,
) {
    // info!("block hit");
    match hitpoints.damage(1) {
        Ok(_) => {}
        Err(_) => commands.entity(entity).despawn(),
    }
}

fn on_ball_hit(
    entity: Entity,
    velocity: &mut Velocity,
    previous_velocity: &mut PreviousVelocity,
    transform: &Transform,
    collider: &Collider,
    commands: &mut Commands,
) {
    // info!(
    //     "ball hit {:?} velocity: {:?} previous_velocity: {:?}",
    //     entity, velocity.linvel, previous_velocity.linvel
    // );

    if velocity.linvel.length_squared() < previous_velocity.linvel.length_squared() {
        // balls are not allowed to slow down
        velocity.linvel = velocity.linvel.normalize() * previous_velocity.linvel.length();
    }
    // TODO handle speed increasing here instead of using Restitution

    const MAX_VELOCITY: f32 = 500.0;
    if velocity.linvel.length_squared() > MAX_VELOCITY * MAX_VELOCITY {
        velocity.linvel = velocity.linvel.normalize() * MAX_VELOCITY;
    }

    previous_velocity.linvel = velocity.linvel;
}
