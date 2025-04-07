use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, CollisionGroups, Friction, Restitution, RigidBody};
use noise::{Fbm, MultiFractal, NoiseFn, Perlin, RidgedMulti};
use strum_macros::EnumIter;

use crate::{
    app_state::AppState,
    asset_loading::GameImageAssets,
    ball::Ball,
    particles::BoxParticlesEvent,
    physics::{BALL_GROUP, BLOCK_GROUP, PADDLE_GROUP, WALL_GROUP},
};

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

pub const WALL_WIDTH: f32 = 10.0;
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
        CollisionGroups::new(WALL_GROUP, BALL_GROUP | PADDLE_GROUP),
    ));
    commands.spawn((
        Transform::from_xyz(-BLOCK_GROUP_OFFSET, 0.0, 0.0),
        RigidBody::Fixed,
        Friction::coefficient(0.0),
        Restitution::coefficient(1.1),
        Collider::halfspace(Vec2 { x: 1.0, y: 0.0 }).unwrap(),
        CollisionGroups::new(WALL_GROUP, BALL_GROUP | PADDLE_GROUP),
    ));

    // UI walls
    // commands
    //     .spawn(Node {
    //         width: Val::Percent(100.0),
    //         height: Val::Percent(100.0),
    //         justify_content: JustifyContent::SpaceBetween,
    //         ..Default::default()
    //     })
    //     .with_children(|parent| {
    //         // left vertical fill (border)
    //         parent.spawn((
    //             Node {
    //                 width: Val::Px(WALL_WIDTH),
    //                 // border: UiRect::all(Val::Px(2.)),
    //                 ..default()
    //             },
    //             BackgroundColor(Color::srgb(0., 0., 0.)),
    //         ));
    //         // right vertical fill (border)
    //         parent.spawn((
    //             Node {
    //                 width: Val::Px(WALL_WIDTH),
    //                 // border: UiRect::all(Val::Px(2.)),
    //                 right: Val::Percent(0.0),
    //                 ..default()
    //             },
    //             BackgroundColor(Color::srgb(0., 0., 0.)),
    //         ));
    //     });
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
        CollisionGroups::new(BLOCK_GROUP, BALL_GROUP | PADDLE_GROUP),
    ));
}

fn check_for_new_block_depths(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    balls_query: Query<&Transform, With<Ball>>,
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

    const BUFFER: usize = 2;
    let mut current_depth =
        ((viewport_position.y - BLOCK_SIZE / 2.0) / -(BLOCK_SIZE + BLOCK_GAP_SIZE)).ceil() as usize;

    // iterate over balls to see if any are deeper than current depth
    for ball in balls_query.iter() {
        let ball_depth = (ball.translation.y / -(BLOCK_SIZE + BLOCK_GAP_SIZE)).ceil() as usize;
        if ball_depth > current_depth {
            current_depth = ball_depth;
        }
    }
    current_depth += BUFFER;

    if current_depth > *deepest_layer {
        println!("Current depth: {}", current_depth);
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

#[derive(Component, Debug, Clone, Copy)]
pub struct CrackSprite;

fn on_add_block(
    trigger: Trigger<OnAdd, Block>,
    query: Query<&Block>,
    mut commands: Commands,
    assets: Res<GameImageAssets>,
) {
    if let Ok(block) = query.get(trigger.entity()) {
        // let crack = commands
        //     .spawn((
        //         CrackSprite,
        //         Sprite {
        //             image: assets.crack.clone(),
        //             custom_size: Some(Vec2 {
        //                 x: BLOCK_SIZE,
        //                 y: BLOCK_SIZE,
        //             }),
        //             color: Color::srgba(1.0, 1.0, 1.0, 0.0),
        //             ..Default::default()
        //         },
        //     ))
        //     .id();
        if let Some(mut entity_commands) = commands.get_entity(trigger.entity()) {
            entity_commands.try_insert((
                // Sprite::from_color(
                //     block.0.temp_colour(),
                //     Vec2 {
                //         x: BLOCK_SIZE,
                //         y: BLOCK_SIZE,
                //     },
                // ),
                Sprite {
                    image: block.0.image_handle(&assets),
                    custom_size: Some(Vec2 {
                        x: BLOCK_SIZE,
                        y: BLOCK_SIZE,
                    }),
                    // color: Color::srgb(1.02, 1.02, 1.02),
                    // color: Color::srgba(1.5, 1.5, 1.5, 0.3),
                    ..Default::default()
                },
                HitPoints(block.0.max_hitpoints()),
            ));
            // .add_child(crack);
        }
    }
}

#[derive(Component)]
pub struct DespawnHack;

fn despawn_hack(query: Query<Entity, With<DespawnHack>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).try_despawn_recursive();
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

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, EnumIter)]
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
            BlockType::Blue => 20,
            BlockType::DarkBlue => 5,
            BlockType::LightBlue => 1,
            BlockType::Purple => 5,
            BlockType::LightPurple => 6,
            BlockType::Pink => 3,
            BlockType::Red => 10,
            BlockType::Orange => 20,
        }
    }

    pub fn colour(&self) -> Color {
        match self {
            BlockType::Blue => Color::srgb(0.145, 0.290, 0.725),
            BlockType::DarkBlue => Color::srgb(0.098, 0.443, 0.675),
            BlockType::LightBlue => Color::srgb(0.286, 0.663, 0.871),
            BlockType::Purple => Color::srgb(0.459, 0.224, 0.784),
            BlockType::LightPurple => Color::srgb(0.678, 0.212, 0.757),
            BlockType::Red => Color::srgb(0.894, 0.333, 0.380),
            BlockType::Pink => Color::srgb(0.753, 0.180, 0.373),
            BlockType::Orange => Color::srgb(0.945, 0.663, 0.333),
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

    if g > 0.65 {
        BlockType::Blue
    } else if b > 0.65 {
        BlockType::LightBlue
    } else if c > 0.65 {
        BlockType::LightPurple
    } else if d > 0.65 {
        if position.y > 80.0 {
            BlockType::Red
        } else {
            BlockType::Pink
        }
    } else if e > 0.65 {
        if position.y > 160.0 {
            BlockType::Orange
        } else {
            BlockType::LightPurple
        }
    } else if f > 0.7 {
        BlockType::Purple
    } else if a > 0.65 {
        BlockType::Pink
    } else {
        pick_base_block_type(position)
    }
}

fn pick_base_block_type(position: Vec2) -> BlockType {
    let blend = 10.0
        * Fbm::<Perlin>::new(123)
            .set_frequency(0.4)
            .get([position.x as f64, position.y as f64]) as f32;
    if blend + position.y < 100.0 {
        BlockType::LightBlue
    } else if blend + position.y < 200.0 {
        BlockType::DarkBlue
    } else {
        BlockType::Blue
    }
}

pub fn block_break(block_type: BlockType, transform: &Transform, commands: &mut Commands) {
    const NUM_PARTICLES: usize = 10;
    let mut rng = rand::rng();
    let bloom_color = Color::srgba(
        block_type.colour().to_srgba().red * 1.1,
        block_type.colour().to_srgba().green * 1.1,
        block_type.colour().to_srgba().blue * 1.1,
        1.0,
    );

    for i in 0..NUM_PARTICLES {
        let angle = i as f32 * (std::f32::consts::PI * 2.0 / NUM_PARTICLES as f32);
        let x = angle.cos() * BLOCK_SIZE / 2.0;
        let y = angle.sin() * BLOCK_SIZE / 2.0;

        commands.trigger(BoxParticlesEvent {
            init_position: transform.translation.truncate(),
            // + Vec2::new(rng.random_range(-5.0..5.0), rng.random_range(-5.0..5.0)),
            target_position: transform.translation.truncate() + Vec2::new(x, y),
            z_index: -5.0,
            color: bloom_color,
            target_color: bloom_color.with_alpha(0.0),
            size: Vec2::new(BLOCK_SIZE / 4.0, BLOCK_SIZE / 4.0),
            target_scale: Vec3::ONE,
            duration: Duration::from_millis(500),
        });
    }
}
