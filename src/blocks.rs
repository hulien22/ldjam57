use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, Friction, Restitution, RigidBody};
use noisy_bevy::fbm_simplex_2d_seeded;

use crate::app_state::AppState;

pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_blocks)
            .add_systems(
                FixedUpdate,
                check_for_new_block_depths.run_if(in_state(AppState::Game)),
            )
            .add_systems(PreUpdate, despawn_hack);
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
            spawn_block_at(j, i, &mut commands);
        }
    }

    // Spawn walls/planes on the sides.
    commands.spawn((
        Transform::from_xyz(BLOCK_GROUP_OFFSET, 0.0, 0.0),
        RigidBody::Fixed,
        Friction::coefficient(0.0),
        Restitution::coefficient(1.1),
        Collider::halfspace(Vec2 { x: -1.0, y: 0.0 }).unwrap(),
    ));
    commands.spawn((
        Transform::from_xyz(-BLOCK_GROUP_OFFSET, 0.0, 0.0),
        RigidBody::Fixed,
        Friction::coefficient(0.0),
        Restitution::coefficient(1.1),
        Collider::halfspace(Vec2 { x: 1.0, y: 0.0 }).unwrap(),
    ));
}

fn spawn_block_at(j: usize, i: usize, commands: &mut Commands) {
    let block_type = pick_block_type(Vec2 {
        x: j as f32,
        y: i as f32,
    });
    commands.spawn((
        Sprite::from_color(
            block_type.temp_colour(),
            Vec2 {
                x: BLOCK_SIZE,
                y: BLOCK_SIZE,
            },
        ),
        Transform::from_xyz(
            -BLOCK_GROUP_OFFSET + j as f32 * (BLOCK_SIZE + BLOCK_GAP_SIZE) + BLOCK_SIZE / 2.0,
            i as f32 * -(BLOCK_SIZE + BLOCK_GAP_SIZE) + BLOCK_SIZE / 2.0,
            0.0,
        ),
        Collider::cuboid(BLOCK_SIZE / 2.0, BLOCK_SIZE / 2.0),
        RigidBody::Fixed,
        Friction::coefficient(0.0),
        Restitution::coefficient(1.1),
        Block,
        HitPoints(block_type.max_hitpoints()),
        StateScoped(AppState::Game),
        Name::new(format!("Block {} {}", i, j)),
    ));
}

fn check_for_new_block_depths(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut deepest_layer: Local<usize>,
    mut draw: Gizmos,
    mut commands: Commands,
) {
    if *deepest_layer == 0 {
        *deepest_layer = BLOCK_COUNT_WIDTH;
    }
    let (camera, camera_transform) = camera_query
        .get_single()
        .expect("Need single camera to check depth.");
    let viewport_position = camera
        .viewport_to_world_2d(camera_transform, camera.logical_viewport_size().unwrap())
        .expect("Need viewport position to check depth.");
    draw.circle_2d(viewport_position, 5f32, Color::srgb(1.0, 0.0, 1.0));
    let current_depth =
        ((viewport_position.y - BLOCK_SIZE / 2.0) / -(BLOCK_SIZE + BLOCK_GAP_SIZE)).ceil() as usize;
    if current_depth > *deepest_layer {
        for l in *deepest_layer..current_depth {
            for j in 0..BLOCK_COUNT_WIDTH {
                spawn_block_at(j, l, &mut commands);
            }
        }
        *deepest_layer = current_depth;
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Block;

#[derive(Component)]
pub struct DespawnHack;

fn despawn_hack(query: Query<Entity, With<DespawnHack>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).try_despawn();
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct HitPoints(u16);

impl HitPoints {
    pub fn damage(&mut self, amount: u16) -> Result<u16, ()> {
        if self.0 <= amount {
            return Err(());
        }
        self.0 -= amount;
        Ok(self.0)
    }
}

pub enum BlockType {
    Basic,
    Iron,
    Obsidian,
}

impl BlockType {
    fn max_hitpoints(&self) -> u16 {
        match self {
            BlockType::Basic => 1,
            BlockType::Iron => 2,
            BlockType::Obsidian => 100,
        }
    }

    fn temp_colour(&self) -> Color {
        match self {
            BlockType::Basic => Color::srgb(0.322, 0.212, 0.071),
            BlockType::Iron => Color::srgb(0.4, 0.4, 0.4),
            BlockType::Obsidian => Color::srgb(0.216, 0.106, 0.42),
        }
    }
}

const MY_FAV_SEED: f32 = 12121.0;
const FREQUENCY_SCALE: f32 = 0.05;
const AMPLITUDE_SCALE: f32 = 4.0;
const RADIUS: f32 = 30.;
const OCTAVES: usize = 3;
const LACUNARITY: f32 = 2.;
const GAIN: f32 = 0.5;

fn pick_block_type(position: Vec2) -> BlockType {
    let val = (fbm_simplex_2d_seeded(
        position * FREQUENCY_SCALE,
        OCTAVES,
        LACUNARITY,
        GAIN,
        MY_FAV_SEED,
    ) + 1.0)
        / 2.0;
    info!("{}", val);
    if val < 0.5 {
        BlockType::Basic
    } else if val < 0.8 {
        BlockType::Iron
    } else {
        BlockType::Obsidian
    }
}
