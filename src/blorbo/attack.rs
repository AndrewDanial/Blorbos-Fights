use super::blorbo_plugin::{Blorbo, LightningAttackDmg};
use crate::effects::lightning::{LightningDespawnTimer, LightningEffect, StartAndEnd};
use crate::evil_blorbo::evil_blorbo_plugin::EvilBlorbo;
use crate::shared::Health;
use bevy::color::palettes::css::*;
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

#[derive(Resource)]
pub struct EnemyQueue(pub Vec<(Transform, f32, Entity)>);

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (tick_attack, (attack, chain_lightning).chain()))
            .insert_resource(AttackTimer {
                timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            })
            .insert_resource(AttackRadius(100.0))
            .insert_resource(EnemyQueue(vec![]));
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
    mut gizmos: ResMut<Assets<GizmoAsset>>,
    mut enemy_queue: ResMut<EnemyQueue>,
) {
    if !attack_timer.timer.finished() {
        return;
    }

    let circle = meshes.add(Circle::new(10.0));
    let color = materials.add(Color::linear_rgb(0.165, 0.847, 0.98)); // light blue

    for (t, _, blorbo_entity) in blorbos {
        for (enemy_t, _, _, enemy_entity) in enemies.iter_mut() {
            let distance = (t.translation - enemy_t.translation).length();
            if distance < attack_radius.0 {
                enemy_queue
                    .0
                    .push((enemy_t.clone(), distance, enemy_entity));
            }
        }
        enemy_queue.0.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        for (enemy_t, mut health, _, enemy_entity) in enemies.iter_mut() {
            if !enemy_queue.0.is_empty() && enemy_queue.0[0].2 != enemy_entity {
                return;
            }
            let distance = (t.translation - enemy_t.translation).length();
            if distance > attack_radius.0 {
                continue;
            }
            health.0 -= attack_dmg.0;
            if health.0 <= 0 {
                continue;
            }

            // circle at the blorbo attacking
            let c_1 = cmd
                .spawn((
                    Mesh2d(circle.clone()),
                    MeshMaterial2d(color.clone()),
                    LightningEffect(blorbo_entity),
                    Transform::from_xyz(t.translation.x, t.translation.y, 1.0),
                    LightningDespawnTimer {
                        timer: Timer::from_seconds(2.0, TimerMode::Once),
                    },
                ))
                .id();

            // circle at the enemy getting attacked
            let c_2 = cmd
                .spawn((
                    Mesh2d(circle.clone()),
                    MeshMaterial2d(color.clone()),
                    LightningEffect(enemy_entity),
                    Transform::from_xyz(enemy_t.translation.x, enemy_t.translation.y, 1.0),
                    LightningDespawnTimer {
                        timer: Timer::from_seconds(2.0, TimerMode::Once),
                    },
                ))
                .id();

            let mut gizmo = GizmoAsset::default();
            gizmo.line_2d(enemy_t.translation.xy(), t.translation.xy(), BLUE);

            cmd.spawn((
                Gizmo {
                    handle: gizmos.add(gizmo),
                    ..default()
                },
                StartAndEnd(c_1, c_2),
                LightningDespawnTimer {
                    timer: Timer::from_seconds(2.0, TimerMode::Once),
                },
            ));
        }
    }
}

fn chain_lightning(
    mut cmd: Commands,
    mut enemy_queue: ResMut<EnemyQueue>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut gizmos: ResMut<Assets<GizmoAsset>>,
    mut enemies: Query<(&Transform, &mut Health, Entity)>,
    attack_radius: Res<AttackRadius>,
    attack_dmg: Res<LightningAttackDmg>,
) {
    if enemy_queue.0.is_empty() {
        return;
    }
    let circle = meshes.add(Circle::new(10.0));
    let color = materials.add(Color::linear_rgb(0.165, 0.847, 0.98)); // light blue
    let first_enemy = enemy_queue.0[0];
    enemy_queue.0.clear();

    for (transform, mut health, entity) in enemies.iter_mut() {
        let distance = (first_enemy.0.translation - transform.translation).length();
        if distance > attack_radius.0 || entity == first_enemy.2 {
            continue;
        }
        health.0 -= attack_dmg.0;
        if health.0 <= 0 {
            continue;
        }

        let c_1 = cmd
            .spawn((
                Mesh2d(circle.clone()),
                MeshMaterial2d(color.clone()),
                LightningEffect(first_enemy.2),
                Transform::from_xyz(
                    first_enemy.0.translation.x,
                    first_enemy.0.translation.y,
                    1.0,
                ),
                LightningDespawnTimer {
                    timer: Timer::from_seconds(2.0, TimerMode::Once),
                },
            ))
            .id();

        let c_2 = cmd
            .spawn((
                Mesh2d(circle.clone()),
                MeshMaterial2d(color.clone()),
                LightningEffect(entity),
                Transform::from_xyz(transform.translation.x, transform.translation.y, 1.0),
                LightningDespawnTimer {
                    timer: Timer::from_seconds(2.0, TimerMode::Once),
                },
            ))
            .id();

        let mut gizmo = GizmoAsset::default();
        gizmo.line_2d(
            first_enemy.0.translation.xy(),
            transform.translation.xy(),
            BLUE,
        );

        cmd.spawn((
            Gizmo {
                handle: gizmos.add(gizmo),
                ..default()
            },
            StartAndEnd(c_1, c_2),
            LightningDespawnTimer {
                timer: Timer::from_seconds(2.0, TimerMode::Once),
            },
        ));
    }
}

fn tick_attack(mut attack_timer: ResMut<AttackTimer>, time: Res<Time>) {
    attack_timer.timer.tick(time.delta());
}
