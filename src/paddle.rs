use std::time::Duration;

use bevy::{math::VectorSpace, prelude::*};
use bevy_rapier2d::prelude::{
    ActiveCollisionTypes, ActiveEvents, Ccd, Collider, ColliderMassProperties, CollisionEvent,
    CollisionGroups, ExternalImpulse, Friction, GravityScale, KinematicCharacterController,
    LockedAxes, Restitution, RigidBody, Sensor, Velocity,
};
use leafwing_input_manager::{input_map, prelude::*};
use rand::Rng;

use crate::{
    app_state::AppState,
    asset_loading::GameImageAssets,
    ball::{CollectedResources, spawn_ball},
    blocks::{BLOCK_GROUP_OFFSET, BLOCK_SIZE},
    particles::{BoxParticle, BoxParticlesEvent},
    physics::{BALL_GROUP, BLOCK_GROUP, PADDLE_GROUP, PADDLE_SHOP_GROUP, WALL_GROUP},
    shop::{ShopItem, ShopStats, try_buy},
    shoppanel::{ShopPanel, UpdateShopPanelsEvent},
    statsbar::{UpdateStatsBarBallsEvent, UpdateStatsBarDepthEvent, UpdateStatsBarResourcesEvent},
};

pub struct PaddlePlugin;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PaddleAction {
    #[actionlike(DualAxis)]
    Move,
    Fire,
    Interact,
}

impl PaddleAction {
    pub fn default_bindings() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // kbm controls only for now
        input_map.insert_dual_axis(Self::Move, VirtualDPad::arrow_keys());
        input_map.insert_dual_axis(Self::Move, VirtualDPad::wasd());
        input_map.insert(Self::Fire, KeyCode::Space);
        input_map.insert(Self::Interact, KeyCode::KeyE);

        input_map
    }
}

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PaddleAction>::default())
            .add_systems(OnEnter(AppState::Game), spawn_paddle)
            .add_systems(FixedUpdate, move_paddle.run_if(in_state(AppState::Game)))
            .add_systems(PostUpdate, follow_cam.run_if(in_state(AppState::Game)))
            .add_systems(Update, spawn_particles.run_if(in_state(AppState::Game)));
    }
}

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct NumBalls(pub u32);

#[derive(Component)]
pub struct PaddleBottomSprite;

// TODO height won't change, but width will so need to move to a resource
const PADDLE_WIDTH: f32 = 32.2;
const PADDLE_HEIGHT: f32 = 5.0;
const UFO_SCALE: f32 = PADDLE_HEIGHT / 60. * 2.;

const PADDLE_MAX_HEIGHT: f32 = 1000.0;
const PADDLE_BLOOM: f32 = 1.4;

fn spawn_paddle(mut commands: Commands, assets: Res<GameImageAssets>) {
    commands
        .spawn((
            Paddle,
            // Sprite::from_color(
            //     Color::srgb(1.0, 1.0, 1.0),
            //     Vec2 {
            //         x: PADDLE_WIDTH,
            //         y: PADDLE_HEIGHT,
            //     },
            // ),
            Transform::from_xyz(0.0, 980.0, 0.0),
            Collider::cuboid(PADDLE_WIDTH / 2., PADDLE_HEIGHT),
            RigidBody::Dynamic,
            // KinematicCharacterController::default(),
            (
                GravityScale(0.0),
                ColliderMassProperties::Density(10.0),
                Friction::coefficient(1.0),
                Restitution::coefficient(2.0),
                ActiveCollisionTypes::all(),
                ActiveEvents::COLLISION_EVENTS,
                Ccd::enabled(),
                CollisionGroups::new(PADDLE_GROUP, WALL_GROUP | BALL_GROUP | BLOCK_GROUP),
                Velocity::default(),
            ),
            StateScoped(AppState::Game),
            Name::new("Paddle"),
            InputManagerBundle::with_map(PaddleAction::default_bindings()),
            // LockedAxes::ROTATION_LOCKED_Z,
            InheritedVisibility::default(),
            CollectedResources::new(),
            NumBalls(3),
        ))
        .with_children(|parent| {
            parent.spawn(Sprite {
                image: assets.ufo_top.clone(),
                custom_size: Some(Vec2 { x: 193., y: 60. } * UFO_SCALE),
                color: Color::srgb(PADDLE_BLOOM, PADDLE_BLOOM, PADDLE_BLOOM),
                ..Default::default()
            });
            parent.spawn((
                Sprite {
                    image: assets.ufo_bottom.clone(),
                    custom_size: Some(Vec2 { x: 193., y: 60. } * UFO_SCALE),
                    color: Color::srgb(PADDLE_BLOOM, PADDLE_BLOOM, PADDLE_BLOOM),
                    ..Default::default()
                },
                PaddleBottomSprite,
            ));
            parent.spawn((
                Collider::ball(1.),
                ActiveCollisionTypes::all(),
                ActiveEvents::COLLISION_EVENTS,
                Sensor,
                CollisionGroups::new(PADDLE_SHOP_GROUP, PADDLE_SHOP_GROUP),
            ));
        });

    // spawn other random bg stuff here too cuz why not
    commands.spawn((
        Sprite {
            image: assets.title.clone(),
            custom_size: Some(Vec2 { x: 1280., y: 700. }),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 1080.0, -50.0),
        Name::new("Title"),
    ));
    commands.spawn((
        Sprite {
            image: assets.wasd.clone(),
            custom_size: Some(Vec2 { x: 100., y: 100. }),
            ..Default::default()
        },
        Transform::from_xyz(150.0, 800.0, -50.0),
        Name::new("WASD"),
    ));
    commands.spawn((
        Sprite {
            image: assets.arrows.clone(),
            custom_size: Some(Vec2 { x: 100., y: 100. }),
            ..Default::default()
        },
        Transform::from_xyz(-150.0, 800.0, -50.0),
        Name::new("Arrows"),
    ));
    commands.spawn((
        Sprite {
            image: assets.movetext.clone(),
            custom_size: Some(Vec2 { x: 300., y: 100. }),
            // custom_size: Some(Vec2 { x: 193., y: 60. } * UFO_SCALE),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 800.0, -50.0),
        Name::new("MoveText"),
    ));
    commands.spawn((
        Sprite {
            image: assets.space.clone(),
            custom_size: Some(Vec2 { x: 100., y: 50. }),
            ..Default::default()
        },
        Transform::from_xyz(-150.0, 720.0, -50.0),
        Name::new("space"),
    ));
    commands.spawn((
        Sprite {
            image: assets.shootballtext.clone(),
            custom_size: Some(Vec2 { x: 300., y: 100. }),
            ..Default::default()
        },
        Transform::from_xyz(100.0, 720.0, -50.0),
        Name::new("shootballtext"),
    ));
}

fn move_paddle(
    mut query: Query<
        (
            Entity,
            &ActionState<PaddleAction>,
            &mut Transform,
            &mut Velocity,
            &mut NumBalls,
            &mut CollectedResources,
        ),
        With<Paddle>,
    >,
    time: Res<Time>,
    mut commands: Commands,
    assets: Res<GameImageAssets>,
    mut shop_panel_query: Query<(&ShopPanel)>,
    mut paddle_bottom_query: Query<(&PaddleBottomSprite, &mut Sprite)>,
    camera_query: Query<(Entity, &OrthographicProjection), With<Camera>>,
    mut stats: ResMut<ShopStats>,
) {
    let (
        paddle_entity,
        action_state,
        mut transform,
        mut vel,
        mut num_balls,
        mut collected_resources,
    ) = query.get_single_mut().expect("Failed to get paddle entity");
    let (_, orthoproj) = camera_query.get_single().expect("Need single camera.");
    let half_screen_size = orthoproj.area.height() / 2.0;

    // lerp to target velocity
    let mut target_lin_vel: Vec2 = Vec2::ZERO;
    let mut lin_damping = time.delta_secs() * 10.0;
    if action_state.axis_pair(&PaddleAction::Move) != Vec2::ZERO {
        // controller.translation = Some(action_state.axis_pair(&PaddleAction::Move) * 5.0);
        target_lin_vel = action_state.clamped_axis_pair(&PaddleAction::Move) * stats.speed();
        lin_damping *= 2.0;
    }
    vel.linvel = vel.linvel.lerp(target_lin_vel, lin_damping);

    // handle rotation
    const MAX_ROTATION: f32 = f32::to_radians(5.0);
    if transform.rotation.z.abs() > MAX_ROTATION {
        // past the limit, stop rotating
        vel.angvel = 0.0;
        transform.rotation.z = transform.rotation.z.signum() * MAX_ROTATION;
    } else {
        let target_ang_vel: f32;
        let ang_damping: f32;
        const ANG_VEL: f32 = 1.0;
        const MIN_VEL: f32 = 0.1;
        if target_lin_vel.x.abs() > MIN_VEL {
            target_ang_vel = -ANG_VEL * target_lin_vel.x.signum();
            ang_damping = time.delta_secs() * 10.0;
        } else {
            // rotate back towards zero
            target_ang_vel = -ANG_VEL * transform.rotation.z.signum();
            ang_damping = time.delta_secs() * 20.0;
        }
        vel.angvel = FloatExt::lerp(vel.angvel, target_ang_vel, ang_damping);
    }

    if action_state.just_pressed(&PaddleAction::Fire) && num_balls.0 > 0 {
        num_balls.0 -= 1;
        commands.trigger(UpdateStatsBarBallsEvent { balls: num_balls.0 });
        spawn_ball(&mut commands, transform.clone(), assets);
    }

    if action_state.just_pressed(&PaddleAction::Interact) {
        // find enabled shop panel
        for shop_panel in shop_panel_query.iter() {
            if !shop_panel.enabled {
                continue;
            }

            if shop_panel.is_refresh {
                let init_balls = num_balls.0;
                // refresh ball count
                num_balls.0 = stats.capacity() as u32;
                commands.trigger(UpdateStatsBarBallsEvent { balls: num_balls.0 });

                // particles
                let mut rng = rand::rng();
                for _ in init_balls..num_balls.0 {
                    commands.trigger(BoxParticlesEvent {
                        init_position: transform.translation.truncate()
                            + Vec2::new(
                                rng.random_range(-20.0..20.0),
                                rng.random_range(-20.0..20.0),
                            ),
                        target_position: Vec2::new(
                            -BLOCK_GROUP_OFFSET,
                            transform.translation.y + half_screen_size + 10.0,
                        ),
                        z_index: -5.0,
                        color: Color::WHITE,
                        target_color: Color::WHITE,
                        size: Vec2::new(10., 10.),
                        target_scale: Vec3::ONE * 1.2,
                        duration: Duration::from_millis(500),
                    });
                }

                break;
            }

            // try to buy
            if let Some(cost) = match shop_panel.item {
                ShopItem::Damage => stats.damage_cost(),
                ShopItem::Speed => stats.speed_cost(),
                ShopItem::Capacity => stats.capacity_cost(),
                ShopItem::Size => stats.size_cost(),
            } {
                if try_buy(&cost, &mut collected_resources.counts) {
                    match shop_panel.item {
                        ShopItem::Damage => stats.damage_level += 1,
                        ShopItem::Speed => stats.speed_level += 1,
                        ShopItem::Capacity => stats.capacity_level += 1,
                        ShopItem::Size => {
                            stats.size_level += 1;
                            // update paddle size
                            for (_, mut sprite) in paddle_bottom_query.iter_mut() {
                                // let vec = Vec2 { x: 193., y: 60. } * UFO_SCALE
                                //     + Vec2::new(stats.size(), 0.0);
                                sprite.custom_size =
                                    Some(Vec2::new(stats.size(), 60.0 * UFO_SCALE));
                            }
                            // if let Some(mut cuboid) = collider.as_cuboid_mut() {
                            //     cuboid.set_half_extents(Vec2::new(
                            //         stats.size() / 2.,
                            //         PADDLE_HEIGHT / 2.,
                            //     ));
                            // } else {
                            //     info!("Failed to get cuboid from collider: {:?}", collider);
                            // }
                            commands.entity(paddle_entity).remove::<Collider>();
                            commands
                                .entity(paddle_entity)
                                .insert(Collider::cuboid(stats.size() / 2., PADDLE_HEIGHT));
                        }
                    }
                    commands.trigger(UpdateStatsBarResourcesEvent);
                    commands.trigger(UpdateShopPanelsEvent);
                } else {
                    info!("Failed to buy: {:?}", shop_panel.item);
                    // todo play error sound
                }
            }
        }
    }
}

const FOLLOW_SPEED: f32 = 5.0;

fn follow_cam(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Paddle>)>,
    paddle_query: Query<&Transform, (With<Paddle>, Without<Camera>)>,
    time: Res<Time>,
) {
    let (mut camera_transform) = camera_query
        .get_single_mut()
        .expect("Need single camera to follow paddle.");
    let (paddle_transform) = paddle_query
        .get_single()
        .expect("Need single paddle to follow.");
    camera_transform.translation.y = VectorSpace::lerp(
        camera_transform.translation.y,
        paddle_transform.translation.y.min(PADDLE_MAX_HEIGHT),
        FOLLOW_SPEED * time.delta_secs(),
    );
}

fn spawn_particles(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut Velocity, &CollectedResources), With<Paddle>>,
) {
    let (transform, velocity, collected_resources) =
        query.get_single_mut().expect("Failed to get paddle entity");
    if velocity.linvel.length_squared() < 10. {
        return;
    }
    commands.trigger(BoxParticlesEvent {
        init_position: Vec2::new(transform.translation.x, transform.translation.y),
        target_position: transform.translation.truncate() - velocity.linvel.normalize(),
        z_index: -10.0,
        color: Color::srgb(0.4, 0.2, 0.2),
        target_color: Color::srgba(0.4, 0.2, 0.2, 0.0),
        size: Vec2::new(10., 10.),
        target_scale: Vec3::ZERO,
        duration: Duration::from_secs(2),
    });

    // also update stats depth
    commands.trigger(UpdateStatsBarDepthEvent {
        depth: ((transform.translation.y - BLOCK_SIZE) / BLOCK_SIZE).floor() as i32,
    });
}
