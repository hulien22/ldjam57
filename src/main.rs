use app_state::AppState;
use asset_loading::AssetLoadingPlugin;
use ball::BallPlugin;
use bevy::{
    asset::AssetMetaCheck,
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    log::{Level, LogPlugin},
    prelude::*,
    render::camera::ScalingMode,
    text::FontSmoothing,
    window::WindowResolution,
};
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::TweeningPlugin;
use blocks::{BLOCK_GROUP_OFFSET, BlocksPlugin, WALL_WIDTH};
use paddle::PaddlePlugin;
use particles::ParticlesPlugin;
use physics::PhysicsPlugin;

mod app_state;
mod asset_loading;
mod ball;
mod blocks;
mod paddle;
mod particles;
mod physics;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Fix for wasm, skip meta checks
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(LogPlugin {
                    level: Level::INFO,
                    ..Default::default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "ldjam57".into(),
                        // cursor_options: CursorOptions {
                        //     visible: false,
                        //     ..default()
                        // },
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        mode: bevy::window::WindowMode::Windowed,
                        resolution: WindowResolution::default(),
                        position: WindowPosition::default(),
                        resizable: true,
                        resize_constraints: WindowResizeConstraints::default(),
                        window_level: bevy::window::WindowLevel::Normal,
                        desired_maximum_frame_latency: None, //defaults 2
                        //transparent: true,
                        // Tells wasm to resize the window according to the available canvas
                        fit_canvas_to_parent: true,
                        // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
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
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(AssetLoadingPlugin)
        .add_plugins(TweeningPlugin)
        .add_plugins(ParticlesPlugin)
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
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true,
            ..default()
        },
        // // Using a tonemapper that desaturates to white is recommended (https://bevyengine.org/examples/2d-rendering/bloom-2d/)
        Tonemapping::TonyMcMapface,
        Bloom { ..default() },
        Name::new("Camera"),
        OrthographicProjection {
            scale: 1.0,
            scaling_mode: ScalingMode::FixedHorizontal {
                viewport_width: (BLOCK_GROUP_OFFSET + WALL_WIDTH) * 2.0,
            },
            ..OrthographicProjection::default_2d()
        },
    ));
}
