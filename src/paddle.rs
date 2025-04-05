use bevy::{math::VectorSpace, prelude::*};
use bevy_rapier2d::prelude::{
    ActiveCollisionTypes, ActiveEvents, Ccd, Collider, ColliderMassProperties, CollisionEvent,
    ExternalImpulse, Friction, GravityScale, KinematicCharacterController, LockedAxes, Restitution,
    RigidBody, Velocity,
};
use leafwing_input_manager::{input_map, prelude::*};

use crate::{app_state::AppState, ball::spawn_ball};

pub struct PaddlePlugin;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum PaddleAction {
    #[actionlike(DualAxis)]
    Move,
    Fire,
}

impl PaddleAction {
    pub fn default_bindings() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // kbm controls only for now
        input_map.insert_dual_axis(Self::Move, VirtualDPad::arrow_keys());
        input_map.insert(Self::Fire, KeyCode::Space);

        input_map
    }
}

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PaddleAction>::default())
            .add_systems(OnEnter(AppState::Game), spawn_paddle)
            .add_systems(FixedUpdate, move_paddle.run_if(in_state(AppState::Game)))
            .add_systems(PostUpdate, follow_cam.run_if(in_state(AppState::Game)));
    }
}

#[derive(Component)]
struct Paddle;

fn spawn_paddle(mut commands: Commands) {
    commands.spawn((
        Paddle,
        Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2 { x: 30.0, y: 3.0 }),
        Transform::from_xyz(0.0, 30.0, 0.0),
        Collider::cuboid(15.0, 1.5),
        RigidBody::Dynamic,
        // KinematicCharacterController::default(),
        GravityScale(0.0),
        ColliderMassProperties::Density(10.0),
        Friction::coefficient(1.0),
        Restitution::coefficient(2.0),
        (
            ActiveCollisionTypes::all(),
            ActiveEvents::COLLISION_EVENTS,
            Ccd::enabled(),
        ),
        StateScoped(AppState::Game),
        Name::new("Paddle"),
        InputManagerBundle::with_map(PaddleAction::default_bindings()),
        // LockedAxes::ROTATION_LOCKED_Z,
        Velocity::default(),
    ));
}

fn move_paddle(
    mut query: Query<(&ActionState<PaddleAction>, &mut Transform, &mut Velocity), With<Paddle>>,
    time: Res<Time>,
    commands: Commands,
) {
    let (action_state, mut transform, mut vel) =
        query.get_single_mut().expect("Failed to get paddle entity");

    // lerp to target velocity
    let mut target_lin_vel: Vec2 = Vec2::ZERO;
    let mut lin_damping = time.delta_secs() * 10.0;
    if action_state.axis_pair(&PaddleAction::Move) != Vec2::ZERO {
        // controller.translation = Some(action_state.axis_pair(&PaddleAction::Move) * 5.0);
        target_lin_vel = action_state.axis_pair(&PaddleAction::Move) * 100.0;
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

    if action_state.just_pressed(&PaddleAction::Fire) {
        spawn_ball(commands, transform.clone());
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
        paddle_transform.translation.y,
        FOLLOW_SPEED * time.delta_secs(),
    );
}
