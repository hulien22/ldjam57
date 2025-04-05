use app_state::AppState;
use ball::BallPlugin;
use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    text::FontSmoothing,
};
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use blocks::BlocksPlugin;
use paddle::PaddlePlugin;
use physics::PhysicsPlugin;

mod app_state;
mod ball;
mod blocks;
mod paddle;
mod physics;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: Level::INFO,
            ..Default::default()
        }))
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 12.0,
                    font: default(),
                    font_smoothing: FontSmoothing::default(),
                },
                text_color: Color::srgb(0.0, 1.0, 0.0),
                enabled: true,
            },
        })
        .add_plugins(BlocksPlugin)
        .add_plugins(PaddlePlugin)
        .add_plugins(BallPlugin)
        .add_plugins(PhysicsPlugin)
        .add_systems(Startup, setup_camera)
        .init_state::<AppState>()
        .enable_state_scoped_entities::<AppState>()
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Name::new("Camera")));
}
