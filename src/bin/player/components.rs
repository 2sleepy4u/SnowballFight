use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerNumber(pub u8);

#[derive(Component)]
pub struct Player {
    pub id: u64,
    pub speed: f32
}

#[derive(Component)]
pub struct GamepadInput(pub usize);



