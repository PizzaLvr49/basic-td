mod camera;
mod enemy;
mod grid;

use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use bevy::prelude::*;
use bevy_rand::prelude::*;

use camera::CameraPlugin;
use enemy::EnemyPlugin;
use grid::GridPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                present_mode: bevy::window::PresentMode::Mailbox,
                name: Some("Basic TD".to_string()),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 42.0,
                    ..default()
                },
                frame_time_graph_config: FrameTimeGraphConfig {
                    min_fps: 400.0,
                    target_fps: 1400.0,
                    ..default()
                },
                ..default()
            },
        })
        .add_plugins(EntropyPlugin::<WyRand>::with_seed([0; 8]))
        .add_plugins(GridPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(EnemyPlugin)
        .run();
}
