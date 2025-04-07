use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    BackgroundHoriWall,
    app_state::AppState,
    asset_loading::GameImageAssets,
    ball::CollectedResources,
    blocks::{BlockType, WALL_WIDTH},
    paddle::Paddle,
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
            .add_observer(update_stats_bar_resources)
            .add_observer(update_stats_bar_depth)
            .add_observer(update_stats_bar_balls);
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
                        Text::new(format!("")),
                        TextFont { ..default() },
                        TextColor(block_type.colour()),
                        StatsBarResource(block_type),
                    ));
                });
            }
        });
}

#[derive(Event, Debug, Default)]
pub struct UpdateStatsBarResourcesEvent;

fn update_stats_bar_resources(
    trigger: Trigger<UpdateStatsBarResourcesEvent>,
    mut query: Query<(&StatsBarResource, &mut Text)>,
    mut paddle_query: Query<(Entity, &CollectedResources), With<Paddle>>,
) {
    // get the paddle entity and its resources
    let (_, collected_resources) = paddle_query
        .get_single_mut()
        .expect("Failed to get paddle entity");

    for (resource, mut text) in query.iter_mut() {
        let count = collected_resources.counts.get(&resource.0);
        if let Some(c) = count {
            text.0 = format!(" {} ", c);
        } else {
            text.0 = format!("");
        }
    }
}

#[derive(Event, Debug, Default)]
pub struct UpdateStatsBarDepthEvent {
    pub depth: i32,
}

fn update_stats_bar_depth(
    trigger: Trigger<UpdateStatsBarDepthEvent>,
    mut query: Query<(&StatsBarText, &mut Text)>,
) {
    for (stats_bar_text, mut text) in query.iter_mut() {
        if stats_bar_text.0 == "Depth" {
            text.0 = format!("Depth: {}", trigger.depth);
        }
    }
}

#[derive(Event, Debug, Default)]
pub struct UpdateStatsBarBallsEvent {
    pub balls: u32,
}

fn update_stats_bar_balls(
    trigger: Trigger<UpdateStatsBarBallsEvent>,
    mut query: Query<(&StatsBarText, &mut Text)>,
) {
    for (stats_bar_text, mut text) in query.iter_mut() {
        if stats_bar_text.0 == "Balls" {
            text.0 = format!("Balls: {}", trigger.balls);
        }
    }
}
