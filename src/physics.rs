use std::time::Duration;

use bevy::{ecs::query, prelude::*};
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    prelude::*,
    render::RapierDebugRenderPlugin,
};
use rand::Rng;

use crate::{
    app_state::AppState,
    ball::{self, CollectedResources},
    blocks::{BLOCK_GROUP_OFFSET, Block, BlockType, HitPoints, block_break},
    paddle::Paddle,
    particles::BoxParticlesEvent,
    shop::ShopStats,
    shoppanel::{ShopPanel, UpdateShopPanelsEvent},
    statsbar::UpdateStatsBarResourcesEvent,
};
use crate::{ball::Ball, blocks::DespawnHack};

pub struct PhysicsPlugin;

pub const BLOCK_GROUP: bevy_rapier2d::geometry::Group = Group::GROUP_1;
pub const BALL_GROUP: bevy_rapier2d::geometry::Group = Group::GROUP_2;
pub const PADDLE_GROUP: bevy_rapier2d::geometry::Group = Group::GROUP_3;
pub const WALL_GROUP: bevy_rapier2d::geometry::Group = Group::GROUP_4;
pub const PADDLE_SHOP_GROUP: bevy_rapier2d::geometry::Group = Group::GROUP_5;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.0))
            // .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(
                PostUpdate,
                process_collisions.run_if(in_state(AppState::Game)),
            );
    }
}

fn process_collisions(
    mut reader: EventReader<CollisionEvent>,
    mut ball_query: Query<(Entity, &Transform, &mut CollectedResources), With<Ball>>,
    mut paddle_query: Query<(Entity, &mut CollectedResources), (With<Paddle>, Without<Ball>)>,
    mut block_query: Query<(Entity, &mut HitPoints, &Transform, &Collider, &Block), Without<Ball>>,
    mut shop_panel_query: Query<(Entity, &mut ShopPanel)>,
    camera_query: Query<(Entity, &OrthographicProjection), With<Camera>>,
    shop_stats: Res<ShopStats>,
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
                        &shop_stats,
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
                        &shop_stats,
                    );
                }
                // Process the paddle collisions. Use 'else if' to avoid reprocessing any block collisions.
                else if let Ok((entity, mut collected_resources)) = paddle_query.get_mut(lhs) {
                    on_paddle_hit(
                        &mut collected_resources,
                        entity,
                        rhs,
                        &mut commands,
                        &mut ball_query,
                        &camera_query,
                    );
                } else if let Ok((entity, mut collected_resources)) = paddle_query.get_mut(rhs) {
                    on_paddle_hit(
                        &mut collected_resources,
                        entity,
                        lhs,
                        &mut commands,
                        &mut ball_query,
                        &camera_query,
                    );
                }
                // Process shop panel collisions.
                else if let Ok((_, mut shop_panel)) = shop_panel_query.get_mut(lhs) {
                    shop_panel.enabled = true;
                    commands.trigger(UpdateShopPanelsEvent);
                } else if let Ok((_, mut shop_panel)) = shop_panel_query.get_mut(rhs) {
                    shop_panel.enabled = true;
                    commands.trigger(UpdateShopPanelsEvent);
                }
            }
            CollisionEvent::Stopped(lhs, rhs, collision_event_flags) => {
                if let Ok((_, mut shop_panel)) = shop_panel_query.get_mut(lhs) {
                    shop_panel.enabled = false;
                    commands.trigger(UpdateShopPanelsEvent);
                } else if let Ok((_, mut shop_panel)) = shop_panel_query.get_mut(rhs) {
                    shop_panel.enabled = false;
                    commands.trigger(UpdateShopPanelsEvent);
                }
            }
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
    ball_query: &mut Query<(Entity, &Transform, &mut CollectedResources), With<Ball>>,
    shop_stats: &Res<ShopStats>,
) {
    // skip if we aren't hitting a ball
    if let Ok((_, _, mut collected_resources)) = ball_query.get_mut(other) {
        match hitpoints.damage(shop_stats.damage()) {
            Ok(_) => {}
            Err(_) => {
                commands.entity(entity).insert(DespawnHack);

                // Update CollectedResources for the corresponding ball
                collected_resources.add(block.0);

                block_break(block.0, transform, commands);
            }
        }
    }
}

fn on_paddle_hit(
    collected_resources: &mut CollectedResources,
    entity: Entity,
    other: Entity,
    commands: &mut Commands,
    ball_query: &mut Query<(Entity, &Transform, &mut CollectedResources), With<Ball>>,
    camera_query: &Query<(Entity, &OrthographicProjection), With<Camera>>,
) {
    let (_, orthoproj) = camera_query.get_single().expect("Need single camera.");
    let half_screen_size = orthoproj.area.height() / 2.0;

    // check if collision is with a ball
    if let Ok((_, ball_transform, mut ball_collected_resources)) = ball_query.get_mut(other) {
        let mut rng = rand::rng();
        for (block_type, count) in &ball_collected_resources.counts {
            let num_spawns: u32;
            match count {
                0 => continue,
                1..=10 => {
                    num_spawns = 3;
                }
                11..=50 => num_spawns = 6,
                _ => num_spawns = 9,
            }

            let bloom_color = Color::srgba(
                block_type.colour().to_srgba().red * 1.1,
                block_type.colour().to_srgba().green * 1.1,
                block_type.colour().to_srgba().blue * 1.1,
                1.0,
            );

            for _ in 0..num_spawns {
                commands.trigger(BoxParticlesEvent {
                    init_position: ball_transform.translation.truncate()
                        + Vec2::new(rng.random_range(-5.0..5.0), rng.random_range(-5.0..5.0)),
                    target_position: Vec2::new(
                        BLOCK_GROUP_OFFSET,
                        ball_transform.translation.y + half_screen_size + 10.0,
                    ),
                    z_index: 5.0,
                    color: bloom_color,
                    size: Vec2::new(4., 4.),
                    target_scale: Vec3::ONE * 1.2,
                    duration: Duration::from_millis(500),
                });
            }
        }

        // add collected resources to paddle
        collected_resources.combine(&*ball_collected_resources);
        // clear the ball's collected resources
        ball_collected_resources.clear();

        commands.trigger(UpdateStatsBarResourcesEvent);
    }
}
