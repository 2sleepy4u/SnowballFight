use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_basic_scene)
            .add_startup_system(spawn_walls)
            .add_system(update_light);
    }
}
