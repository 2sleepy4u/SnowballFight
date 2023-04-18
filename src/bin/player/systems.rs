use bevy::{
    prelude::*,
    input::gamepad::*,
};

use super::components::*;
use crate::collision::components::Collider;
use crate::collision::events::CollisionEvent;
use snowball_fight::CelMaterial;

pub fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &Player, &mut Transform, Without<GamepadInput>)>,
    time: Res<Time>,
    mut ev_coll: EventReader<CollisionEvent>,
) 
{
    let mut direction: Vec3 = Vec3::default(); 

    for (entity, player, mut transform, keyboard) in &mut query {
        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 0.0, -1.0)
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, 0.0, 1.0)
        }
        if keyboard_input.pressed(KeyCode::Space) {
        }
        if keyboard_input.pressed(KeyCode::LShift) {
        }

        for ev in ev_coll.iter() {
            if ev.source.index() == entity.index() {
                direction += ev.direction.round();
            }
        }

        transform.translation += direction * player.speed * time.delta_seconds();
    }
}

pub fn player_input_gamepad(
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    mut query: Query<(Entity, &Player, &mut Transform, &GamepadInput)>,
    time: Res<Time>,
    mut ev_coll: EventReader<CollisionEvent>,
) {
    for (entity, player, mut transform, gamepad_id) in &mut query {
        let id = gamepad_id;
        let gamepad = Gamepad {id: id.0};

        let axis_lx = GamepadAxis { gamepad, axis_type: GamepadAxisType::LeftStickX };
        let axis_ly = GamepadAxis { gamepad, axis_type: GamepadAxisType::LeftStickY };

        if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
            let mut direction = Vec3::new(x, 0.0, -y).round();
            for ev in ev_coll.iter() {
                if ev.source.index() == entity.index() {
                    direction += ev.direction.round();
                }
            }
            transform.translation += direction * player.speed * time.delta_seconds(); 
        }

        // In a real game, the buttons would be configurable, but here we hardcode them
        let jump_button = GamepadButton { gamepad, button_type: GamepadButtonType::South };
        let dash_button = GamepadButton { gamepad, button_type: GamepadButtonType::East };
        if buttons.just_pressed(jump_button) {
            // button just pressed: make the player jump
        }
        if buttons.pressed(dash_button) {
            // button being held down: heal the player
        }
    }
}


pub fn spawn_player(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CelMaterial>>,
    mut player_count: ResMut<PlayerNumber>,
){
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule {
            radius: 0.2,
            depth: 0.5,
            ..default()
        })),
        //material: materials.add(Color::rgb(0.8, 0.5, 0.7).into()),
        material: materials.add(CelMaterial {
            color: Color::RED,
            light: Vec3::new(4.0, 2.0, 4.0),
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
    .insert(Name::new("G1"))
    .insert(Player {
        id: 1,
        speed: 3.0
    })
    .insert(Collider  {
        x_size: 0.2,
        y_size: 0.5,
        z_size: 0.2
    });

    player_count.0 += 1;

    info!("Aggiunto un giocatore. Numero attuale: {}", player_count.0);
}



pub fn gamepad_connections(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CelMaterial>>,

    mut query: Query<(Entity, &Player, &GamepadInput)>,
    mut player_count: ResMut<PlayerNumber>,

    mut connection_events: EventReader<GamepadConnectionEvent>,
) {
    for connection_event in connection_events.iter() {
        let gamepad = connection_event.gamepad;

        if let GamepadConnection::Connected(info) = &connection_event.connection {
            info!("{:?} Connected", gamepad);
            commands.spawn(MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule {
                    radius: 0.2,
                    depth: 0.5,
                    ..default()
                })),
                material: materials.add(CelMaterial {
                    color: Color::BLUE,
                    light: Vec3::new(4.0, 2.0, 4.0),
                    alpha_mode: AlphaMode::Blend,
                }),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default()
            })
            .insert(Name::new("Cube"))
            .insert(Player {
                id: 1,
                speed: 3.0
            })
            .insert(GamepadInput(gamepad.id))
            .insert(Collider  {
                x_size: 0.2,
                y_size: 0.5,
                z_size: 0.2
            });
            player_count.0 += 1;

            info!("Aggiunto un giocatore. Numero attuale: {}", player_count.0);
        } else {
            info!("{:?} Disconnected", gamepad);

            for (entity, player, PlayerGamepad) in &mut query {
                if PlayerGamepad.0 == gamepad.id {
                   if let Some(mut entity_commands) = commands.get_entity(entity) {
                    entity_commands.despawn()
                   }
                }
            }

        }
    }
}


