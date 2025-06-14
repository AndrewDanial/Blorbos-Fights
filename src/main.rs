// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::{
    color::palettes::css::GREEN,
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    text::FontSmoothing,
};

use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowResolution};
pub mod blorbo;
pub mod effects;
pub mod evil_blorbo;
use blorbo::blorbo_plugin::BlorboPlugin;
use effects::lightning::LightningPlugin;
use evil_blorbo::evil_blorbo_plugin::*;
pub mod shared;
fn main() -> AppExit {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1280.0, 720.0),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins((
            BlorboPlugin,
            EvilBlorboPlugin,
            LightningPlugin,
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        // Here we define size of our overlay
                        font_size: 42.0,
                        // If we want, we can use a custom font
                        font: default(),
                        // We could also disable font smoothing,
                        font_smoothing: FontSmoothing::default(),
                        ..default()
                    },
                    // We can also change color of the overlay
                    text_color: GREEN.into(),
                    // We can also set the refresh interval for the FPS counter
                    refresh_interval: core::time::Duration::from_millis(100),
                    enabled: true,
                },
            },
        ))
        .add_systems(Startup, spawn_camera)
        .run()
}

fn spawn_camera(mut cmd: Commands) {
    cmd.spawn((Camera2d, Transform::from_xyz(0.0, 0.0, 2.0)));
}
