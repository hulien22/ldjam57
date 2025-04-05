use bevy::{math::VectorSpace, prelude::*};
use bevy_rapier2d::prelude::{
    Collider, CollisionEvent, GravityScale, KinematicCharacterController, RigidBody,
};
use leafwing_input_manager::{input_map, prelude::*};

use crate::app_state::AppState;

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
            .add_systems(FixedUpdate, (move_paddle));
    }
}

#[derive(Component)]
struct Paddle;

fn spawn_paddle(mut commands: Commands) {
    commands.spawn((
        Paddle,
        Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2 { x: 10.0, y: 3.0 }),
        Transform::from_xyz(0.0, -10.0, 0.0),
        Collider::cuboid(5.0, 1.5),
        RigidBody::KinematicPositionBased,
        KinematicCharacterController::default(),
        GravityScale(0.0),
        StateScoped(AppState::Game),
        Name::new("Paddle"),
        InputManagerBundle::with_map(PaddleAction::default_bindings()),
    ));
}

fn move_paddle(
    mut query: Query<
        (
            &ActionState<PaddleAction>,
            &mut Transform,
            &mut KinematicCharacterController,
        ),
        With<Paddle>,
    >,
) {
    let (action_state, mut transform, mut controller) =
        query.get_single_mut().expect("Failed to get paddle entity");

    if action_state.axis_pair(&PaddleAction::Move) != Vec2::ZERO {
        // println!(
        //     "Moving paddle by {:?}",
        //     action_state.axis_pair(&PaddleAction::Move)
        // );
        controller.translation = Some(action_state.axis_pair(&PaddleAction::Move) * 5.0);
    }

    if action_state.just_pressed(&PaddleAction::Fire) {
        // println!("shoot");
    }
}
