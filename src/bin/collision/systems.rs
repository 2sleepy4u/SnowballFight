use bevy::prelude::*;

use super::components::*;
use super::events::*;

pub fn check_collision(
    source_query: Query<(Entity, &Collider, &Transform)>,
    target_query: Query<(Entity, &Collider, &Transform)>,
    mut collision_event: EventWriter<CollisionEvent>
){
    for (s_entity, s_collider, s_transform) in &source_query {
        for (t_entity, t_collider, t_transform)in &target_query {
            if s_entity.index() !=  t_entity.index() {
                let isColliding: bool = check(s_transform, s_collider, t_transform, t_collider);     
                if isColliding {
                    let direction: Vec3 = get_direction(s_transform, s_collider, t_transform, t_collider);                    
                    collision_event.send(CollisionEvent {
                        source: s_entity,
                        target: t_entity,
                        direction: direction,
                        is_trigger: t_collider.is_trigger
                    });
                }
            }
        }
    }
}


pub fn debug_collisions(
    mut ev_coll: EventReader<CollisionEvent>,
) {
    for ev in ev_coll.iter() {
        info!("Collision!!");
    }
}

fn get_direction(
    first_transform: &Transform, first_collider: &Collider,
    second_transform: &Transform, second_collider: &Collider
) -> Vec3 {
    let first_pos = first_transform.translation;
    let second_pos = second_transform.translation;

    let mut direction = first_pos - second_pos;
    let extra_size = 0.2;

    //Se fossi stato piu piccolo su uno dei 3 assi, avrei comunque scatenato la collisione??
    let x_dir = if !(
            first_pos.x - first_collider.x_size + extra_size <= second_pos.x + second_collider.x_size &&
            first_pos.x + first_collider.x_size - extra_size >= second_pos.x - second_collider.x_size 
        ) { 1.0 } else { 0.0 }; 

    let y_dir = if !(
            first_pos.y - first_collider.y_size + extra_size <= second_pos.y + second_collider.y_size &&
            first_pos.y + first_collider.y_size - extra_size >= second_pos.y - second_collider.y_size 
        ) { 1.0 } else { 0.0 }; 
    let z_dir = if !(
            first_pos.z - first_collider.z_size + extra_size <= second_pos.z + second_collider.z_size &&
            first_pos.z + first_collider.z_size - extra_size >= second_pos.z - second_collider.z_size 
        ) { 1.0 } else { 0.0 }; 

    direction *= Vec3::new(x_dir, y_dir, z_dir);

    return direction;
}


fn check(
    first_transform: &Transform, first_collider: &Collider,
    second_transform: &Transform, second_collider: &Collider
) -> bool {
    let first_pos = first_transform.translation;
    let second_pos = second_transform.translation;

    return (
        (
            first_pos.x - first_collider.x_size <= second_pos.x + second_collider.x_size &&
            first_pos.x + first_collider.x_size >= second_pos.x - second_collider.x_size 
        ) && (
            first_pos.y - first_collider.y_size <= second_pos.y + second_collider.y_size &&
            first_pos.y + first_collider.y_size >= second_pos.y - second_collider.y_size 
        ) && (
            first_pos.z - first_collider.z_size <= second_pos.z + second_collider.z_size &&
            first_pos.z + first_collider.z_size >= second_pos.z - second_collider.z_size 
        )
    )
}


