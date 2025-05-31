use crate::blorbo::blorbo_plugin::*;
use bevy::prelude::*;
use rand::prelude::*;
pub struct BlorboMovementPlugin;

impl Plugin for BlorboMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (random_move, apply_screen_wrap));
    }
}

fn random_move(q: Query<(&mut Transform, &mut Velocity, &mut Blorbo)>, time: Res<Time>) {
    let mut rng = rand::thread_rng();
    for (mut transform, mut velocity, mut blorbo) in q {
        transform.translation += Vec3::new(velocity.x, velocity.y, 0.0);
        blorbo.timer.tick(time.delta());
        if blorbo.timer.finished() {
            info!("timer finished");
            *velocity = Velocity {
                x: rng.gen_range(-2.0..2.0),
                y: rng.gen_range(-2.0..2.0),
            };
        }
    }
}

fn apply_screen_wrap(
    window: Single<&Window>,
    mut wrap_query: Query<&mut Transform, With<ScreenWrap>>,
) {
    let size = window.size() + 16.0;
    let half_size = size / 2.0;
    for mut transform in &mut wrap_query {
        let position = transform.translation.xy();
        let wrapped = (position + half_size).rem_euclid(size) - half_size;
        transform.translation = wrapped.extend(transform.translation.z);
    }
}
