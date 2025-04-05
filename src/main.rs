use app_state::AppState;
use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use blocks::BlocksPlugin;

mod app_state;
mod blocks;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: Level::INFO,
            ..Default::default()
        }))
        .add_plugins(BlocksPlugin)
        .add_systems(Startup, setup_camera)
        .init_state::<AppState>()
        .enable_state_scoped_entities::<AppState>()
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Name::new("Camera")));
}
