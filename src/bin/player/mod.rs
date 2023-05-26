use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;
use components::PlayerNumber;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerNumber(0))
            .insert_resource(Gamepads::default())
            .add_startup_system(spawn_keyboard_player)
            .add_system(gamepad_connections)
            .add_system(player_input_keyboard)
            .add_system(player_input_gamepad)
            .add_system(check_player_collision)
            .add_system(check_player_stats)
            .add_system(player_movement);
            //.add_system(player_input)
            //.add_system(player_input_gamepad);
    }
}



