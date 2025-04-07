use bevy::prelude::*;

use crate::{BackgroundHoriWall, app_state::AppState, asset_loading::GameImageAssets};

pub struct StatsBarPlugin;

#[derive(Component)]
pub struct StatsBar;

#[derive(Component)]
pub struct StatsBarText;

#[derive(Component)]
pub struct StatsBarBackground;

impl Plugin for StatsBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_stats_bar)
            .add_systems(Update, update_stats_bar.run_if(in_state(AppState::Game)));
    }
}

pub const STATS_BAR_HEIGHT: f32 = 30.0;

fn spawn_stats_bar(
    mut commands: Commands,
    assets: Res<GameImageAssets>,
    camera_query: Query<(Entity, &Camera, &GlobalTransform, &OrthographicProjection)>,
) {
    let (cam_entity, camera, transform, orthoproj) =
        camera_query.get_single().expect("Need single camera.");

    let background = commands
        .spawn((
            BackgroundHoriWall,
            Sprite::from_color(
                Color::srgb(33.0 / 256.0, 33.0 / 256.0, 33.0 / 256.0),
                Vec2::new(orthoproj.area.width(), STATS_BAR_HEIGHT),
            ),
            Transform::from_translation(Vec3 {
                x: 0.0,
                y: orthoproj.area.height() / 2.0 - STATS_BAR_HEIGHT / 2.0,
                z: 100.0,
            }),
            Name::new("StatsBackground"),
        ))
        .id();

    if let Some(mut entity_commands) = commands.get_entity(cam_entity) {
        entity_commands.add_child(background);
    }
}

fn update_stats_bar(
    mut commands: Commands,
    mut stats_bar_query: Query<(&StatsBar, &mut Transform)>,
) {
    // let camera = camera_query
    //     .get_single()
    //     .expect("Need single camera to spawn background.");
    // for (stats_bar, mut transform) in stats_bar_query.iter_mut() {
    //     // Update the position of the stats bar based on the camera position
    //     transform.translation.x = camera.2.translation.x;
    //     transform.translation.y = camera.2.translation.y - 100.0; // Adjust the y offset as needed
    // }
}
