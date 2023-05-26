use bevy::{prelude::*};
use std::f64::consts::PI;

fn main(){
    let direction: Vec3 = Vec3::new(0.0, 0.0, 1.0);
    let point: Vec3     = Vec3::new(0.0, 0.0, 1.0);

    let new_direction = rotate_point(point, direction);

    println!("{}", new_direction);

    let test = get_direction(direction);
    println!("{}", test);

    println!("LAST {}", get_angle(direction));
}


fn rotate_point(point: Vec3, direction: Vec3) -> Vec3 {
    let base_vector = Vec3::new(1.0, 0.0, 0.0);

    let _base_magnitude = (base_vector.x.powf(2.0) + base_vector.z.powf(2.0)).sqrt();
    let _direction_magnitude = (direction.x.powf(2.0) + direction.z.powf(2.0)).sqrt();

    //let angle: f32 = (base_vector.dot(direction) / base_magnitude * direction_magnitude).acos();

    let angle: f32 = base_vector.dot(direction).acos();
    let angle_sin: f32 = base_vector.dot(direction).asin();

    println!("Angle1: {}, Angle sin: {}", angle, angle_sin);

    let x: f32 = angle.cos() as f32 * point.x - angle.sin() as f32 * point.z;
    let z: f32 = angle.sin() as f32 * point.x + angle.cos() as f32 * point.z;

    return Vec3::new(x.round(), 0.0, z.round());
}


fn get_direction(point: Vec3) -> f32 {
    return (point.z.abs() / point.x.abs()).atan();
}


fn get_angle(direction: Vec3) -> f32 {
    let base_vector = Vec3::new(1.0, 0.0, 0.0);
    let mut angle = (direction.z.abs() / direction.x.abs()).atan();
    let sin = base_vector.dot(direction).asin();
    let cos = base_vector.dot(direction).acos() * direction.z; 

    
    if sin >= 0.0 && cos >= 0.0 {
    } 
    else if sin <= 0.0 && cos >= 0.0 {
        angle += PI as f32;
    }
    else if sin <= 0.0 && cos <= 0.0 {
        angle += 2.0 * PI as f32;
    }
    else if sin >= 0.0 && cos <= 0.0 {
        angle += 3.0 * PI as f32;
    }

    println!("DEBUG {} - {}", sin, cos);
    return angle;
}
