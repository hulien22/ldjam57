use app_state::AppState;
use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use blocks::BlocksPlugin;
use paddle::PaddlePlugin;
use physics::PhysicsPlugin;

mod app_state;
mod blocks;
mod paddle;
mod physics;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: Level::INFO,
            ..Default::default()
        }))
        .add_plugins(BlocksPlugin)
        .add_plugins(PaddlePlugin)
        .add_plugins(PhysicsPlugin)
        .add_systems(Startup, setup_camera)
        .init_state::<AppState>()
        .enable_state_scoped_entities::<AppState>()
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Name::new("Camera")));
}
