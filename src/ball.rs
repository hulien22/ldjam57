use bevy::prelude::*;
use bevy_rapier2d::prelude::{
    ActiveCollisionTypes, ActiveEvents, CoefficientCombineRule, Collider, CollisionEvent, Damping,
    Friction, GravityScale, Restitution, RigidBody, Velocity,
};

use crate::app_state::AppState;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(OnEnter(AppState::Game), spawn_blocks);
        app.add_systems(FixedUpdate, clamp_balls);
    }
}

#[derive(Component)]
struct Ball;

pub fn spawn_ball(mut commands: Commands, transform: Transform) {
    commands.spawn((
        Ball,
        Sprite::from_color(Color::srgb(0.5, 0.5 as f32, 0.2), Vec2 { x: 10.0, y: 10.0 }),
        Transform::from_xyz(transform.translation.x, transform.translation.y - 5.0, 0.0),
        Collider::ball(5.0),
        RigidBody::Dynamic,
        GravityScale(0.0),
        Friction::coefficient(0.0),
        Restitution::coefficient(1.0),
        // Friction {
        //     coefficient: 0.0,
        //     combine_rule: CoefficientCombineRule::Min,
        // },
        // Restitution {
        //     coefficient: 1.0,
        //     combine_rule: CoefficientCombineRule::Max,
        // },
        Damping {
            linear_damping: 0.0,
            angular_damping: 1.0,
        },
        ActiveCollisionTypes::all(),
        ActiveEvents::COLLISION_EVENTS,
        StateScoped(AppState::Game),
        Name::new("Ball"),
        Velocity::linear(Vec2::new(0.0, -100.0)),
    ));
}

fn clamp_balls(mut query: Query<&mut Velocity, With<Ball>>) {
    const MAX_VELOCITY: f32 = 500.0;
    for mut velocity in query.iter_mut() {
        if velocity.linvel.length_squared() > MAX_VELOCITY * MAX_VELOCITY {
            println!(
                "clamped ball velocity: {}",
                velocity.linvel.length_squared()
            );
            velocity.linvel = velocity.linvel.normalize() * MAX_VELOCITY;
        }
    }
}
