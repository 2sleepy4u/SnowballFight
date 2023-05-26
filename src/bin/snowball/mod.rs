use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct SnowballPlugin;

impl Plugin for SnowballPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_snowballs)
            .add_system(check_collision);
    }
}
