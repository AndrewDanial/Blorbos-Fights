use bevy::prelude::*;

use crate::blorbo::blorbo_plugin::Blorbo;
use crate::evil_blorbo::evil_blorbo_plugin::EvilBlorbo;
/// Used to keep track of the lightning visual effects.
/// Needs Entity to follow its location
#[derive(Component)]
pub struct LightningEffect(pub Entity);

pub struct LightningPlugin;

impl Plugin for LightningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, follow_entity);
    }
}

fn follow_entity(
    effect: Query<(&LightningEffect, &mut Transform), (Without<Blorbo>, Without<EvilBlorbo>)>,
    enemies: Query<(&Transform, Entity), With<EvilBlorbo>>,
    blorbos: Query<(&Transform, Entity), With<Blorbo>>,
) {
    for (LightningEffect(entity), mut transform) in effect {
        for (enemy_t, enemy_e) in enemies {
            if enemy_e == *entity {
                transform.translation.x = enemy_t.translation.x;
                transform.translation.y = enemy_t.translation.y;
            }
        }

        for (blorbo_t, blorbo_e) in blorbos {
            if blorbo_e == *entity {
                transform.translation.x = blorbo_t.translation.x;
                transform.translation.y = blorbo_t.translation.y;
            }
        }
    }
}
