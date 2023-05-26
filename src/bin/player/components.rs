use bevy::{prelude::*, time::Stopwatch};

#[derive(Resource)]
pub struct PlayerNumber(pub u8);

#[derive(Component)]
pub struct Player {
    pub id: u32
}

#[derive(Component)]
pub struct GamepadInput(pub u32);


#[derive(Component, Clone)]
pub struct Dash {
    pub time: Stopwatch,

    pub is_dashing: bool,
    pub direction: Vec3,
    pub start: Vec3
}

#[derive(Component)]
pub struct Attack {
    pub time: Stopwatch,
    pub direction: Vec3
}

#[derive(Component)]
pub struct Stats {
    pub speed: f32,
    pub health: i8
}

#[derive(Component)]
pub struct PlayerMovement {
    pub direction: Vec3
}
