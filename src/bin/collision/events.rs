use bevy::prelude::*;

pub struct CollisionEvent {
    pub source: Entity,
    pub target: Entity,
    pub direction: Vec3,
    pub is_trigger: bool
}
