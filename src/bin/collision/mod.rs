use bevy::prelude::*;

pub mod events;
pub mod components;
mod systems;

use systems::*;
use components::*;

use self::events::CollisionEvent;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_system(check_collision);
            //.add_system(debug_collisions);
    }
}
