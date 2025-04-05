use bevy::prelude::*;
use bevy_rapier2d::prelude::{
    Collider, CollisionEvent, Friction, GravityScale, Restitution, RigidBody,
};

use crate::app_state::AppState;

pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_blocks);
    }
}

const BLOCK_SIZE: f32 = 10.0;
const BLOCK_COUNT_WIDTH: usize = 40;
const BLOCK_GAP_SIZE: f32 = 2.0;
const BLOCK_GROUP_OFFSET: f32 =
    (BLOCK_SIZE * BLOCK_COUNT_WIDTH as f32 + BLOCK_GAP_SIZE * (BLOCK_COUNT_WIDTH - 1) as f32) / 2.0;

fn spawn_blocks(mut commands: Commands) {
    for i in 0..BLOCK_COUNT_WIDTH {
        for j in 0..BLOCK_COUNT_WIDTH {
            commands.spawn((
                Sprite::from_color(
                    Color::srgb(0.5, 0.5 as f32, 0.2),
                    Vec2 {
                        x: BLOCK_SIZE,
                        y: BLOCK_SIZE,
                    },
                ),
                Transform::from_xyz(
                    -BLOCK_GROUP_OFFSET
                        + j as f32 * (BLOCK_SIZE + BLOCK_GAP_SIZE)
                        + BLOCK_SIZE / 2.0,
                    i as f32 * -(BLOCK_SIZE + BLOCK_GAP_SIZE) + BLOCK_SIZE / 2.0,
                    0.0,
                ),
                Collider::cuboid(BLOCK_SIZE / 2.0, BLOCK_SIZE / 2.0),
                RigidBody::Fixed,
                GravityScale(0.0),
                Friction::coefficient(0.0),
                Restitution::coefficient(1.1),
                Block,
                HitPoints(3),
                StateScoped(AppState::Game),
                Name::new(format!("Block {} {}", i, j)),
            ));
        }
    }
    commands.spawn((
        Transform::from_xyz(BLOCK_GROUP_OFFSET, 0.0, 0.0),
        RigidBody::Fixed,
        Collider::halfspace(Vec2 { x: -1.0, y: 0.0 }).unwrap(),
    ));

    commands.spawn((
        Transform::from_xyz(-BLOCK_GROUP_OFFSET, 0.0, 0.0),
        RigidBody::Fixed,
        Collider::halfspace(Vec2 { x: 1.0, y: 0.0 }).unwrap(),
    ));
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Block;

#[derive(Component, Debug, Clone, Copy)]
pub struct HitPoints(u16);

impl HitPoints {
    pub fn damage(&mut self, amount: u16) -> Result<u16, ()> {
        self.0 -= amount;
        if self.0 <= 0 {
            return Err(());
        }
        Ok(self.0)
    }
}
