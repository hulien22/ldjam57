use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: Level::INFO,
            ..Default::default()
        }))
        .run();
}
