use bevy::prelude::*;

//Quad shape collider 
#[derive(Component)]
pub struct Collider {
    pub x_size: f32,
    pub y_size: f32,
    pub z_size: f32
}
