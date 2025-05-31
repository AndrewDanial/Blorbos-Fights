// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::{asset::AssetMetaCheck, prelude::*};
use rand::prelude::*;
pub mod blorbo;
use blorbo::blorbo_plugin::*;
use blorbo::movement::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins(BlorboPlugin)
        .add_systems(Startup, (spawn_camera, spawn_sprite))
        .run()
}

fn spawn_camera(mut cmd: Commands) {
    cmd.spawn(Camera2d);
}

fn spawn_sprite(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    cmd.spawn((
        Sprite {
            image: asset_server.load("images/blorbo.png"),
            ..default()
        },
        Blorbo {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        },
        Velocity {
            x: rng.gen_range(-5.0..5.0),
            y: rng.gen_range(-5.0..5.0),
        },
    ));
}
