use crate::BlorboMovementPlugin;
use crate::shared::Health;
use bevy::prelude::*;
use rand::prelude::*;
#[derive(Component)]
pub struct Blorbo {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Velocity {
    pub speed: f32,
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct ScreenWrap;

pub struct BlorboPlugin;

impl Plugin for BlorboPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BlorboMovementPlugin)
            .add_systems(Startup, spawn_blorbo);
    }
}

fn spawn_blorbo(mut cmd: Commands, asset_server: Res<AssetServer>) {
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
        Health(100),
    ));
}
