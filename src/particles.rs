use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{
    Animator, Tracks, Tween, TweenCompleted,
    lens::{
        ColorMaterialColorLens, SpriteColorLens, TransformPositionLens, TransformRotationLens,
        TransformScaleLens,
    },
};

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(Update, despawn_finished_particle)
            .add_systems(Update, handle_tween_events)
            .add_observer(box_particle_observer);
    }
}

#[derive(Event, Debug)]
pub struct BoxParticlesEvent {
    pub init_position: Vec2,
    pub target_position: Vec2,
    pub z_index: f32,
    pub color: Color,
}

#[derive(Component)]
pub struct BoxParticle;

const DELETE_ON_COMPLETION: u64 = 1;

fn box_particle_observer(trigger: Trigger<BoxParticlesEvent>, mut commands: Commands) {
    let start = Vec3::new(
        trigger.init_position.x,
        trigger.init_position.y,
        trigger.z_index,
    );
    let end = Vec3::new(
        trigger.target_position.x,
        trigger.target_position.y,
        trigger.z_index,
    );
    let transform = Transform::from_translation(start);
    let transform_tween_track = Tracks::new([
        Tween::new(
            EaseFunction::ExponentialOut,
            Duration::from_secs(2),
            TransformPositionLens {
                start: start,
                end: end,
            },
        ),
        Tween::new(
            EaseFunction::ExponentialOut,
            Duration::from_secs(2),
            TransformScaleLens {
                start: Vec3::ONE,
                end: Vec3::ZERO,
            },
        ),
        Tween::new(
            EaseFunction::BounceInOut,
            Duration::from_secs(2),
            TransformRotationLens {
                start: Quat::IDENTITY,
                end: Quat::from_axis_angle(Vec3::Z, std::f32::consts::PI / 2.),
            },
        )
        .with_completed_event(DELETE_ON_COMPLETION),
    ]);
    let color_tween = Tween::new(
        EaseFunction::ExponentialOut,
        Duration::from_secs(2),
        SpriteColorLens {
            start: trigger.color,
            end: trigger.color.with_alpha(0.0),
        },
    );

    commands.spawn((
        BoxParticle,
        Sprite::from_color(Color::WHITE, Vec2 { x: 10.0, y: 10.0 }),
        transform,
        Animator::new(transform_tween_track),
        Animator::new(color_tween),
    ));
}

fn handle_tween_events(
    mut commands: Commands,
    mut tween_completed_reader: EventReader<TweenCompleted>,
) {
    for t in tween_completed_reader.read() {
        if t.user_data == DELETE_ON_COMPLETION {
            commands.entity(t.entity).try_despawn_recursive();
        }
    }
}
