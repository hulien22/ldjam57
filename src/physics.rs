use bevy::{ecs::query, prelude::*};
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    prelude::*,
    render::RapierDebugRenderPlugin,
};

use crate::{
    app_state::AppState,
    ball::{self, CollectedResources},
    blocks::{Block, BlockType, HitPoints},
};
use crate::{ball::Ball, blocks::DespawnHack};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.0))
            //.add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(
                PostUpdate,
                process_collisions.run_if(in_state(AppState::Game)),
            );
    }
}

fn process_collisions(
    mut reader: EventReader<CollisionEvent>,
    mut ball_query: Query<(Entity, &mut CollectedResources), With<Ball>>,
    mut block_query: Query<(Entity, &mut HitPoints, &Transform, &Collider, &Block), Without<Ball>>,
    mut commands: Commands,
) {
    for &collision in reader.read() {
        match collision {
            CollisionEvent::Started(lhs, rhs, collision_event_flags) => {
                if let Ok((entity, mut hitpoints, transform, collider, block)) =
                    block_query.get_mut(lhs)
                {
                    on_block_hit(
                        block,
                        hitpoints.as_mut(),
                        transform,
                        collider,
                        entity,
                        rhs,
                        &mut commands,
                        &mut ball_query,
                    );
                } else if let Ok((entity, mut hitpoints, transform, collider, block)) =
                    block_query.get_mut(rhs)
                {
                    on_block_hit(
                        block,
                        hitpoints.as_mut(),
                        transform,
                        collider,
                        entity,
                        lhs,
                        &mut commands,
                        &mut ball_query,
                    );
                }
            }
            _ => {}
        }
    }
}

fn on_block_hit(
    block: &Block,
    hitpoints: &mut HitPoints,
    transform: &Transform,
    collider: &Collider,
    entity: Entity,
    other: Entity,
    commands: &mut Commands,
    ball_query: &mut Query<(Entity, &mut CollectedResources), With<Ball>>,
) {
    match hitpoints.damage(1) {
        Ok(_) => {}
        Err(_) => {
            commands.entity(entity).insert(DespawnHack);

            // Update CollectedResources for the corresponding ball
            if let Ok((_, mut collected_resources)) = ball_query.get_mut(other) {
                collected_resources.add(block.0);
            }
        }
    }
}
