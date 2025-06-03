use bevy::color::palettes::css::BLUE;
use bevy::prelude::*;

use crate::blorbo::blorbo_plugin::Blorbo;
use crate::evil_blorbo::evil_blorbo_plugin::EvilBlorbo;
use crate::shared::Health;
/// Used to keep track of the lightning visual effects.
/// Needs Entity to follow its location
#[derive(Component)]
pub struct LightningEffect(pub Entity);

pub struct LightningPlugin;

#[derive(Component)]
pub struct LightningDespawnTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct StartAndEnd(pub Entity, pub Entity);

impl Plugin for LightningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (follow_entity, line_follow, tick_despawn_timer));
    }
}

fn follow_entity(
    mut cmd: Commands,
    effect: Query<
        (&LightningEffect, &mut Transform, Entity),
        (Without<Blorbo>, Without<EvilBlorbo>),
    >,
    enemies: Query<(&Transform, &Health, Entity), With<EvilBlorbo>>,
    blorbos: Query<(&Transform, Entity), With<Blorbo>>,
) {
    let mut found = false;
    for (LightningEffect(entity), mut transform, circle) in effect {
        for (enemy_t, hp, enemy_e) in enemies {
            if enemy_e == *entity {
                transform.translation.x = enemy_t.translation.x;
                transform.translation.y = enemy_t.translation.y;
                found = true;
                if hp.0 <= 0 {
                    cmd.entity(circle).despawn();
                }
                break;
            }
        }

        for (blorbo_t, blorbo_e) in blorbos {
            if blorbo_e == *entity {
                transform.translation.x = blorbo_t.translation.x;
                transform.translation.y = blorbo_t.translation.y;
                found = true;
                break;
            }
        }

        if !found {
            cmd.entity(circle).despawn();
        }
    }
}

fn line_follow(
    mut cmd: Commands,
    lines: Query<(&mut Gizmo, &StartAndEnd, Entity)>,
    mut gizmos: ResMut<Assets<GizmoAsset>>,
    circles: Query<(&Transform, Entity)>,
) {
    for (mut g, StartAndEnd(c1, c2), entity) in lines {
        let mut found = false;
        let mut c1_t: Transform = Transform::default();
        let mut c2_t: Transform = Transform::default();
        for (c_t, entity) in circles {
            if entity == *c1 {
                c1_t = c_t.clone();
                found = true;
            }
            if entity == *c2 {
                c2_t = c_t.clone();
                found = true;
            }
        }
        if !found {
            if let Err(_) = cmd.get_entity(*c1) {
                cmd.entity(entity).despawn();
            }
            if let Err(_) = cmd.get_entity(*c2) {
                cmd.entity(entity).despawn();
            }

            continue;
        }
        let mut gizmo = GizmoAsset::default();
        gizmo.line_2d(c1_t.translation.xy(), c2_t.translation.xy(), BLUE);
        g.handle = gizmos.add(gizmo);
    }
}

fn tick_despawn_timer(
    mut cmd: Commands,
    despawn_timers: Query<(&mut LightningDespawnTimer, Entity)>,
    time: Res<Time>,
) {
    for (mut timer, e) in despawn_timers {
        timer.timer.tick(time.delta());
        if timer.timer.finished() {
            cmd.entity(e).despawn();
        }
    }
}
