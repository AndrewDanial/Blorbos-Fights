use bevy::prelude::*;

#[derive(Component)]
pub struct Health(pub i32);

/// How far apart enemies should be
#[derive(Component)]
pub struct RepelRadius(pub f32);

