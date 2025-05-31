use crate::BlorboMovementPlugin;
use bevy::prelude::*;
#[derive(Component)]
pub struct Blorbo {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

pub struct BlorboPlugin;

impl Plugin for BlorboPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BlorboMovementPlugin);
    }
}
