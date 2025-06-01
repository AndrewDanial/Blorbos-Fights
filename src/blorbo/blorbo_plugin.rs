use crate::BlorboMovementPlugin;
use bevy::prelude::*;
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
        app.add_plugins(BlorboMovementPlugin);
    }
}
