use app_state::AppState;
use asset_loading::{AssetLoadingPlugin, GameImageAssets};
use ball::BallPlugin;
use bevy::{
    asset::AssetMetaCheck,
    core_pipeline::{
        bloom::{Bloom, BloomPrefilter},
        tonemapping::Tonemapping,
    },
    log::{Level, LogPlugin},
    prelude::*,
    render::camera::ScalingMode,
    text::FontSmoothing,
    window::{WindowResized, WindowResolution},
};
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::TweeningPlugin;
use blocks::{BLOCK_GROUP_OFFSET, BlocksPlugin, WALL_WIDTH};
use paddle::PaddlePlugin;
use particles::ParticlesPlugin;
use physics::PhysicsPlugin;
use shop::ShopPlugin;
use ui::UiPlugin;

mod app_state;
mod asset_loading;
mod ball;
mod blocks;
mod paddle;
mod particles;
mod physics;
mod shop;
mod ui;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
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
        .add_plugins(UiPlugin)
        .add_plugins(ShopPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(AppState::Game), spawn_background)
        .add_systems(Update, on_resize_system)
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
        Bloom {
            intensity: 0.5,
            low_frequency_boost: 0.7,
            low_frequency_boost_curvature: 0.95,
            high_pass_frequency: 1.0,
            prefilter: BloomPrefilter {
                threshold: 0.8,
                threshold_softness: 0.2,
            },
            composite_mode: bevy::core_pipeline::bloom::BloomCompositeMode::Additive,
            ..default()
        },
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

#[derive(Component)]
struct Background;
#[derive(Component)]
struct BackgroundWall;

fn spawn_background(
    mut commands: Commands,
    assets: Res<GameImageAssets>,
    camera_query: Query<(Entity, &Camera, &GlobalTransform)>,
) {
    let camera = camera_query
        .get_single()
        .expect("Need single camera to spawn background.");
    let background = commands
        .spawn((
            Background,
            Sprite {
                image: assets.background.clone(),
                custom_size: camera.1.logical_viewport_size(),
                ..Default::default()
            },
            Transform::from_translation(Vec3 {
                x: 0.0,
                y: 0.0,
                z: -100.0,
            }),
            Name::new("Background"),
        ))
        .id();
    let wall1 = commands
        .spawn((
            BackgroundWall,
            Sprite::from_color(
                Color::srgb(33.0 / 256.0, 33.0 / 256.0, 33.0 / 256.0),
                Vec2::new(WALL_WIDTH, 720.0),
            ),
            Transform::from_translation(Vec3 {
                x: -(BLOCK_GROUP_OFFSET + (WALL_WIDTH / 2.0)),
                y: 0.0,
                z: -99.0,
            }),
            Name::new("BackgroundWall"),
        ))
        .id();
    let wall2 = commands
        .spawn((
            BackgroundWall,
            Sprite::from_color(
                Color::srgb(33.0 / 256.0, 33.0 / 256.0, 33.0 / 256.0),
                Vec2::new(WALL_WIDTH, 720.0),
            ),
            Transform::from_translation(Vec3 {
                x: (BLOCK_GROUP_OFFSET + (WALL_WIDTH / 2.0)),
                y: 0.0,
                z: -99.0,
            }),
            Name::new("BackgroundWall"),
        ))
        .id();

    // Add as children of the camera so they move with it
    if let Some(mut entity_commands) = commands.get_entity(camera.0) {
        entity_commands.add_child(background);
        entity_commands.add_child(wall1);
        entity_commands.add_child(wall2);
    }
}

fn on_resize_system(
    mut bg_query: Query<&mut Sprite, With<Background>>,
    mut bgwall_query: Query<&mut Sprite, (With<BackgroundWall>, Without<Background>)>,
    camera_query: Query<
        (Entity, &OrthographicProjection),
        (With<Camera>, Changed<OrthographicProjection>),
    >,
) {
    for (_, orthoproj) in camera_query.iter() {
        for mut sprite in bg_query.iter_mut() {
            sprite.custom_size = Some(Vec2::new(orthoproj.area.width(), orthoproj.area.height()));
        }
        for mut sprite in bgwall_query.iter_mut() {
            sprite.custom_size = Some(Vec2::new(WALL_WIDTH, orthoproj.area.height()));
        }
    }
}
