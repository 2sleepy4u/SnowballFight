pub use bevy::prelude::*;
use std::f32::consts::PI;

pub mod bezier;
pub mod materials;

/*
pub fn rotate_point(point: Vec3, direction: Vec3) -> Vec3 {
    //let base_vector = Vec3::new(1.0, 0.0, 0.0);
    //let angle: f32 = base_vector.dot(direction).acos();
    let angle: f32 = get_angle(direction);
    let x: f32 = angle.cos() as f32 * point.x - angle.sin() as f32 * point.z;
    let z: f32 = angle.sin() as f32 * point.x + angle.cos() as f32 * point.z;

    return Vec3::new(x.round(), 0.0, z.round());
}

fn get_angle(direction: Vec3) -> f32 {
    let base_vector = Vec3::new(1.0, 0.0, 0.0);
    let mut angle = (direction.z.abs() / direction.x.abs()).atan();
    println!("Direction {}", direction);
    println!("Dot {}", 1.0-base_vector.dot(direction));
    
    return angle;
}
*/

#[derive(PartialEq, Copy, Clone)]
enum Horizontal {
    Right,
    Left,
    Neutral
}

#[derive(PartialEq, Copy, Clone)]
enum Vertical {
    Up,
    Down,
    Neutral
}

pub fn rotate_point(point: Vec3, direction: Vec3) -> Vec3 {
    let angle = get_angle(direction);
    return rotate_vector(point, angle);
}

pub fn rotate_vector(point: Vec3, angle: f32) -> Vec3 {
    let x: f32 = angle.cos() as f32 * point.x - angle.sin() as f32 * point.z;
    let z: f32 = angle.sin() as f32 * point.x + angle.cos() as f32 * point.z;

    return Vec3::new(x.round(), point.y, z.round());
}

pub fn get_angle(direction: Vec3) -> f32 {
    let base_vector = Vec3::new(1.0, 0.0, 0.0);

    let mut angle_tan = (direction.z.abs() / direction.x.abs()).atan();

    let horizontal = if direction.x > 0.0 { 
        Horizontal::Right
    } else if direction.x < 0.0 {
        Horizontal::Left
    } else {
        Horizontal::Neutral
    };

    let vertical = if direction.z > 0.0 { 
        Vertical::Up
    } else if direction.z < 0.0 {
        Vertical::Down
    } else {
        Vertical::Neutral
    };


    if (horizontal == Horizontal::Right || horizontal == Horizontal::Neutral) && vertical != Vertical::Down {
    } else if horizontal == Horizontal::Left && (vertical != Vertical::Down && vertical != Vertical::Neutral) {
        angle_tan += PI / 2.0;
    } else if (horizontal == Horizontal::Left || horizontal == Horizontal::Neutral) && vertical != Vertical::Up {
        angle_tan += PI;
    } else {
        angle_tan += 3.0 / 2.0 * PI;
    }

    return angle_tan;
}


