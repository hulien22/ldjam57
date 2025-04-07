use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    BackgroundHoriWall,
    app_state::AppState,
    asset_loading::GameImageAssets,
    blocks::{BlockType, WALL_WIDTH},
};

pub struct StatsBarPlugin;

#[derive(Component)]
pub struct StatsBar;

#[derive(Component)]
pub struct StatsBarText(pub String);
#[derive(Component)]
pub struct StatsBarResource(pub BlockType);

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
    // add ui
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(STATS_BAR_HEIGHT),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Stretch,
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            Name::new("Stats Bar"),
            // BackgroundColor(Color::srgba(33.0 / 256.0, 33.0 / 256.0, 33.0 / 256.0, 0.5)),
        ))
        .with_children(|parent| {
            let text_bg = (Node {
                margin: UiRect {
                    left: Val::Px(10.),
                    right: Val::Px(10.0),
                    top: Val::Auto,
                    bottom: Val::Auto,
                },
                // padding / margin
                ..default()
            },);
            // left stuff has a text field for depth and ball count
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::FlexStart,
                        ..default()
                    },
                    BackgroundColor(Color::srgba(33.0 / 256.0, 33.0 / 256.0, 33.0 / 256.0, 1.0)),
                ))
                .with_children(|parent| {
                    parent.spawn((text_bg.clone(),)).with_children(|parent| {
                        parent.spawn((
                            Text::new("Depth: 0"),
                            TextFont { ..default() },
                            TextColor(Color::WHITE),
                            StatsBarText("Depth".to_string()),
                        ));
                    });
                    parent.spawn((text_bg.clone(),)).with_children(|parent| {
                        parent.spawn((
                            Text::new("Balls: 0"),
                            TextFont { ..default() },
                            TextColor(Color::WHITE),
                            StatsBarText("Balls".to_string()),
                        ));
                    });
                });

            // right stuff is for CollectedResources
            let mut right = parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    ..default()
                },
                BackgroundColor(Color::srgba(33.0 / 256.0, 33.0 / 256.0, 33.0 / 256.0, 1.0)),
            ));
            for block_type in BlockType::iter() {
                right.with_children(|parent| {
                    parent.spawn((
                        Text::new(format!(" 0 ")),
                        TextFont { ..default() },
                        TextColor(block_type.colour()),
                        StatsBarResource(block_type),
                    ));
                });
            }
        });
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
