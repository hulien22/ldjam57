use bevy::prelude::*;

use crate::app_state::AppState;

pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_blocks);
    }
}

fn spawn_blocks(mut commands: Commands) {
    for i in 0..20 {
        for j in 0..20 {
            commands.spawn((
                Sprite::from_color(Color::srgb(0.5, 0.5 as f32, 0.2), Vec2 { x: 10.0, y: 10.0 }),
                Transform::from_xyz(i as f32 * 11.0, j as f32 * 11.0, 0.0),
                StateScoped(AppState::Game),
                Name::new(format!("Block {} {}", i, j)),
            ));
        }
    }
}
