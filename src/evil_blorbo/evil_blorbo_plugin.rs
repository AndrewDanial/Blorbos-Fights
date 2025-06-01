use crate::blorbo::blorbo_plugin::Blorbo;
use bevy::prelude::*;
use rand::prelude::*;
pub struct EvilBlorboPlugin;

#[derive(Resource)]
pub struct SpawnTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct EvilBlorbo;

impl Plugin for EvilBlorboPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_evil_blorbo, move_towards_blorbos))
            .insert_resource::<SpawnTimer>(SpawnTimer {
                timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            });
    }
}

fn spawn_evil_blorbo(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    timer.timer.tick(time.delta());

    if timer.timer.finished() {
        let rand_x = rng.gen_range(-500.0..500.0);
        let rand_y = rng.gen_range(-500.0..500.0);
        cmd.spawn((
            Sprite {
                image: asset_server.load("images/evil_blorbo.png"),
                ..default()
            },
            Transform::from_xyz(rand_x, rand_y, 0.0).with_scale(Vec3::splat(0.5)),
            EvilBlorbo,
        ));
    }
}

fn move_towards_blorbos(
    evil_blorbos: Query<(&mut Transform, &EvilBlorbo), Without<Blorbo>>,
    blorbos: Query<(&Transform, &Blorbo), Without<EvilBlorbo>>,
    time: Res<Time>,
) {
    for (mut evil_t, _) in evil_blorbos {
        let mut direction = Vec3::ZERO;
        for (t, _) in blorbos.iter() {
            direction = evil_t.translation - t.translation;
        }

        evil_t.translation += -direction.normalize() * 100.0 * time.delta_secs();
    }
}
