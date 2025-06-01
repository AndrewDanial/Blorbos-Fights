// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::{asset::AssetMetaCheck, prelude::*};
use rand::prelude::*;
pub mod blorbo;
pub mod evil_blorbo;
use blorbo::{blorbo_plugin::*, movement::*};
use evil_blorbo::evil_blorbo_plugin::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins((BlorboPlugin, EvilBlorboPlugin))
        .add_systems(Startup, (spawn_camera, spawn_sprite))
        .run()
}

fn spawn_camera(mut cmd: Commands) {
    cmd.spawn(Camera2d);
}

fn spawn_sprite(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    let directions = [-1, 0, 1];
    let x_dir = rng.gen_range(0..=2);
    let y_dir = rng.gen_range(0..=2);
    cmd.spawn((
        Sprite {
            image: asset_server.load("images/blorbo.png"),
            ..default()
        },
        Blorbo {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        },
        Velocity {
            speed: 100.0,
            x: directions[x_dir] as f32,
            y: directions[y_dir] as f32,
        },
        ScreenWrap,
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(0.5)),
    ));
}
