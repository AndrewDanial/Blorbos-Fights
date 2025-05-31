use crate::blorbo::blorbo_plugin::*;
use bevy::prelude::*;
use rand::prelude::*;
pub struct BlorboMovementPlugin;

impl Plugin for BlorboMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, random_move);
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
