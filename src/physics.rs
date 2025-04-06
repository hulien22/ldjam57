use bevy::prelude::*;
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    prelude::*,
    render::RapierDebugRenderPlugin,
};

use crate::blocks::DespawnHack;
use crate::{
    app_state::AppState,
    blocks::{Block, HitPoints},
};

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
    mut block_query: Query<(Entity, &mut HitPoints, &Transform, &Collider), (With<Block>)>,
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
        Err(_) => {
            commands.entity(entity).insert(DespawnHack);
        }
    }
}
