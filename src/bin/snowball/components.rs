use bevy::{prelude::*, time::Stopwatch};
use snowball_fight::bezier::BezierCurve;
use crate::player::components::Player;

#[derive(Component)]
pub struct Snowball {
    pub direction: Vec3,
    pub player_id: u32,
    pub path: BezierCurve,
    pub duration: f32,
    pub start: Vec3,
    pub time: Stopwatch
}
