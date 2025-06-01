// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowResolution};
pub mod blorbo;
pub mod evil_blorbo;
use blorbo::{blorbo_plugin::*, movement::*};
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
        .add_plugins((BlorboPlugin, EvilBlorboPlugin))
        .add_systems(Startup, spawn_camera)
        .run()
}

fn spawn_camera(mut cmd: Commands) {
    cmd.spawn(Camera2d);
}
