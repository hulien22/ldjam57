use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, Friction, Restitution, RigidBody};
use noise::{Fbm, MultiFractal, NoiseFn, Perlin, RidgedMulti};

use crate::{app_state::AppState, asset_loading::GameImageAssets};

pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_blocks)
            .add_systems(
                FixedUpdate,
                check_for_new_block_depths.run_if(in_state(AppState::Game)),
            )
            .add_observer(on_add_block)
            .add_systems(PreUpdate, despawn_hack.run_if(in_state(AppState::Game)));
    }
}

pub const WALL_WIDTH: f32 = 50.0;
pub const BLOCK_SIZE: f32 = 30.0;
pub const BLOCK_COUNT_WIDTH: usize = 40;
pub const BLOCK_GAP_SIZE: f32 = 0.0;
pub const BLOCK_GROUP_OFFSET: f32 =
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

    // UI walls
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            ..Default::default()
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent.spawn((
                Node {
                    width: Val::Px(WALL_WIDTH),
                    // border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0., 0., 0.)),
            ));
            // right vertical fill (border)
            parent.spawn((
                Node {
                    width: Val::Px(WALL_WIDTH),
                    // border: UiRect::all(Val::Px(2.)),
                    right: Val::Percent(0.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0., 0., 0.)),
            ));
        });
}

fn spawn_block_at(j: usize, i: usize, commands: &mut Commands) {
    let block_type = pick_block_type(Vec2 {
        x: j as f32,
        y: i as f32,
    });
    commands.spawn((
        Transform::from_xyz(
            -BLOCK_GROUP_OFFSET + j as f32 * (BLOCK_SIZE + BLOCK_GAP_SIZE) + BLOCK_SIZE / 2.0,
            i as f32 * -(BLOCK_SIZE + BLOCK_GAP_SIZE) + BLOCK_SIZE / 2.0,
            0.0,
        ),
        Collider::cuboid(BLOCK_SIZE / 2.0, BLOCK_SIZE / 2.0),
        RigidBody::Fixed,
        Friction::coefficient(0.0),
        Restitution::coefficient(1.1),
        Block(block_type),
        StateScoped(AppState::Game),
        Name::new(format!("Block {} {}", i, j)),
    ));
}

fn check_for_new_block_depths(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut deepest_layer: Local<usize>,
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
pub struct Block(pub BlockType);

fn on_add_block(
    trigger: Trigger<OnAdd, Block>,
    query: Query<&Block>,
    mut commands: Commands,
    assets: Res<GameImageAssets>,
) {
    if let Ok(block) = query.get(trigger.entity()) {
        // let crack = commands
        //     .spawn(
        //         (Sprite {
        //             image: assets.crack.clone(),
        //             custom_size: Some(Vec2 {
        //                 x: BLOCK_SIZE,
        //                 y: BLOCK_SIZE,
        //             }),
        //             ..Default::default()
        //         }),
        //     )
        //     .id();
        if let Some(mut entity_commands) = commands.get_entity(trigger.entity()) {
            entity_commands.try_insert((
                Sprite {
                    image: block.0.image_handle(&assets),
                    custom_size: Some(Vec2 {
                        x: BLOCK_SIZE,
                        y: BLOCK_SIZE,
                    }),
                    ..Default::default()
                },
                HitPoints(block.0.max_hitpoints()),
            ));
            //.add_child(crack);
        }
    }
}

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

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum BlockType {
    Blue,
    LightBlue,
    DarkBlue,
    Purple,
    LightPurple,
    Pink,
    Red,
    Orange,
}

impl BlockType {
    fn max_hitpoints(&self) -> u16 {
        match self {
            BlockType::Blue => 1,
            BlockType::DarkBlue => 2,
            BlockType::LightBlue => 100,
            BlockType::Purple => 5,
            BlockType::LightPurple => 6,
            BlockType::Pink => 3,
            BlockType::Red => 6,
            BlockType::Orange => 5,
        }
    }

    fn temp_colour(&self) -> Color {
        match self {
            BlockType::Blue => Color::srgb(0.322, 0.212, 0.071),
            BlockType::DarkBlue => Color::srgb(0.4, 0.4, 0.4),
            BlockType::LightBlue => Color::srgb(0.216, 0.106, 0.42),
            BlockType::Purple => Color::srgb(0.216, 0.106, 0.929),
            BlockType::LightPurple => Color::srgb(0.098, 0.541, 0.055),
            BlockType::Red => Color::srgb(0.831, 0.149, 0.055),
            BlockType::Pink => Color::srgb(0.984, 1.0, 0.169),
            BlockType::Orange => Color::srgb(1.0, 0.651, 0.216),
        }
    }

    fn image_handle(&self, assets: &Res<GameImageAssets>) -> Handle<Image> {
        match self {
            BlockType::Blue => assets.blue.clone(),
            BlockType::DarkBlue => assets.dark_blue.clone(),
            BlockType::LightBlue => assets.light_blue.clone(),
            BlockType::Purple => assets.purple.clone(),
            BlockType::LightPurple => assets.light_purple.clone(),
            BlockType::Red => assets.red.clone(),
            BlockType::Pink => assets.pink.clone(),
            BlockType::Orange => assets.orange.clone(),
        }
    }
}

fn pick_block_type(position: Vec2) -> BlockType {
    let a = Fbm::<Perlin>::new(123)
        .set_frequency(0.4)
        .get([position.x as f64, position.y as f64]);
    let b = Fbm::<Perlin>::new(12412)
        .set_frequency(0.04)
        .get([position.x as f64, position.y as f64]);
    let c = Fbm::<Perlin>::new(123546)
        .set_frequency(0.08)
        .set_lacunarity(2.5)
        .set_octaves(3)
        .get([position.x as f64, position.y as f64]);
    let d = Fbm::<Perlin>::new(1212)
        .set_frequency(0.04)
        .get([position.x as f64, position.y as f64]);
    let e = Fbm::<Perlin>::new(124363)
        .set_frequency(0.04)
        .get([position.x as f64, position.y as f64]);
    let f = RidgedMulti::<Perlin>::new(361232412)
        .set_frequency(0.04)
        //.set_octaves(3)
        .get([position.x as f64, position.y as f64]);
    let g = Fbm::<Perlin>::new(6266123)
        .set_frequency(0.04)
        .get([position.x as f64, position.y as f64]);
    let h = Fbm::<Perlin>::new(31362412)
        .set_frequency(0.04)
        .get([position.x as f64, position.y as f64]);

    info!("{} {}", a, b);
    if g > 0.65 {
        BlockType::Blue
    } else if b > 0.65 {
        BlockType::LightBlue
    } else if c > 0.65 {
        BlockType::LightPurple
    } else if d > 0.65 {
        BlockType::Red
    } else if e > 0.65 {
        BlockType::Orange
    } else if f > 0.7 {
        BlockType::Purple
    } else if a > 0.65 {
        BlockType::Pink
    } else {
        BlockType::DarkBlue
    }
}
