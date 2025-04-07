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

#[derive(Event, Debug, Default)]
pub struct BoxParticlesEvent {
    pub init_position: Vec2,
    pub target_position: Vec2,
    pub z_index: f32,
    pub color: Color,
    pub target_color: Color,
    pub size: Vec2,
    pub target_scale: Vec3,
    pub duration: Duration,
}

impl BoxParticlesEvent {
    pub fn default() -> Self {
        Self {
            init_position: Vec2::ZERO,
            target_position: Vec2::ZERO,
            z_index: 0.0,
            color: Color::WHITE,
            target_color: Color::srgba(1.0, 1.0, 1.0, 0.0),
            size: Vec2::ONE,
            target_scale: Vec3::ZERO,
            duration: Duration::from_secs(2),
        }
    }
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
            EaseFunction::Linear,
            trigger.duration,
            TransformPositionLens {
                start: start,
                end: end,
            },
        ),
        Tween::new(
            EaseFunction::Linear,
            trigger.duration,
            TransformScaleLens {
                start: Vec3::ONE,
                end: trigger.target_scale,
            },
        ),
        Tween::new(
            EaseFunction::Linear,
            trigger.duration,
            TransformRotationLens {
                start: Quat::IDENTITY,
                end: Quat::from_axis_angle(Vec3::Z, std::f32::consts::PI / 2.),
            },
        )
        .with_completed_event(DELETE_ON_COMPLETION),
    ]);
    let color_tween = Tween::new(
        EaseFunction::ExponentialOut,
        trigger.duration,
        SpriteColorLens {
            start: trigger.color,
            end: trigger.target_color,
        },
    );

    commands.spawn((
        BoxParticle,
        Sprite::from_color(trigger.color, trigger.size),
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
