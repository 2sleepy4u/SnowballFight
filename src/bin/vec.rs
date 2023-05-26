use bevy::{prelude::*};
use std::f32::consts::PI;

fn main(){
    let direction: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    let point: Vec3     = Vec3::new(0.0, 0.0, 1.0);

    let angle = get_angle(direction);
    let new = rotate_point(point, angle);

    println!("{}", new);

}


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

fn rotate_point(point: Vec3, angle: f32) -> Vec3 {
    let x: f32 = angle.cos() as f32 * point.x - angle.sin() as f32 * point.z;
    let z: f32 = angle.sin() as f32 * point.x + angle.cos() as f32 * point.z;

    return Vec3::new(x.round(), 0.0, z.round());
}

fn get_angle(direction: Vec3) -> f32 {
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
        println!("1");
    } else if horizontal == Horizontal::Left && vertical != Vertical::Down {
        println!("2");
        angle_tan += PI / 2.0;
    } else if (horizontal == Horizontal::Left || horizontal == Horizontal::Neutral) && vertical != Vertical::Up {
        println!("3");
        angle_tan += PI;
    } else {
        println!("4");
        angle_tan += 3.0 / 2.0 * PI;
    }

    println!("Angle: {}",  angle_tan);
    return angle_tan;
}



