use super::blorbo_plugin::{Blorbo, LightningAttackDmg};
use crate::effects::lightning::LightningEffect;
use crate::evil_blorbo::evil_blorbo_plugin::EvilBlorbo;
use crate::shared::Health;
use bevy::prelude::*;
pub struct AttackPlugin;

#[derive(Resource)]
pub struct AttackTimer {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct AttackRadius(pub f32);

#[derive(Resource)]
pub struct BounceCount(pub i32);

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (tick_attack, attack))
            .insert_resource(AttackTimer {
                timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            })
            .insert_resource(AttackRadius(100.0));
    }
}

fn attack(
    mut cmd: Commands,
    mut enemies: Query<(&Transform, &mut Health, &EvilBlorbo, Entity)>,
    blorbos: Query<(&Transform, &Blorbo, Entity)>,
    attack_timer: Res<AttackTimer>,
    attack_radius: Res<AttackRadius>,
    attack_dmg: Res<LightningAttackDmg>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !attack_timer.timer.finished() {
        return;
    }

    let circle = meshes.add(Circle::new(10.0));
    let color = materials.add(Color::linear_rgb(0.165, 0.847, 0.98)); // light blue

    for (t, _, blorbo_entity) in blorbos {
        for (enemy_t, mut health, _, enemy_entity) in enemies.iter_mut() {
            let distance = (t.translation - enemy_t.translation).length();
            info!("{}", distance);
            if distance < attack_radius.0 {
                info!("attacking");
                health.0 -= attack_dmg.0;

                // circle at the blorbo attacking
                cmd.spawn((
                    Mesh2d(circle.clone()),
                    MeshMaterial2d(color.clone()),
                    LightningEffect(blorbo_entity),
                    Transform::from_xyz(t.translation.x, t.translation.y, 1.0),
                ));

                // circle at the enemy getting attacked
                cmd.spawn((
                    Mesh2d(circle.clone()),
                    MeshMaterial2d(color.clone()),
                    LightningEffect(enemy_entity),
                    Transform::from_xyz(enemy_t.translation.x, enemy_t.translation.y, 1.0),
                ));
            }
        }
    }
}

fn tick_attack(mut attack_timer: ResMut<AttackTimer>, time: Res<Time>) {
    attack_timer.timer.tick(time.delta());
}
