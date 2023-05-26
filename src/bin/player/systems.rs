use bevy::time::Stopwatch;
use bevy::{
    prelude::*,
    input::gamepad::*,
};
use snowball_fight::bezier::BezierCurve;

use super::components::*;
use crate::collision::components::Collider;
use crate::collision::events::CollisionEvent;
use crate::snowball::components::Snowball;

use snowball_fight::materials::*;

use crate::loadout::components::{AttackLoadout, DashLoadout, DashLoadoutList, AttackLoadoutList, default_attack_action};


pub fn check_player_stats (
    mut commands: Commands,
    mut query: Query<(Entity, &Player, &mut Stats)>,
) {
    for (entity, player, mut stats) in &mut query {
        if stats.health == 0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn check_player_collision (
    mut commands: Commands,
    mut query: Query<(Entity, &Player, &mut Stats)>,
    mut snowball_query: Query<(Entity, &Snowball)>,
    mut ev_coll: EventReader<CollisionEvent>,
) {
    for ev in ev_coll.iter() {
        for (entity, player, mut stats) in &mut query {
            if let Ok((snowball_entity, snowball )) = snowball_query.get_mut(ev.target) {
                if entity.index() == ev.source.index() &&
                   snowball.player_id != player.id
                {
                    stats.health -= 1;
                }
            }
        }
    } 
}

pub fn player_movement(
    time: Res<Time>,
    mut query: Query<(Entity, 
                    &Player, &mut Transform,
                    &mut Dash, &mut Attack, 
                    //&mut AttackLoadout, &mut DashLoadout, &Stats,
                    Option<&AttackLoadout>, Option<&DashLoadout>, &Stats, &mut PlayerMovement)>,
    mut ev_coll: EventReader<CollisionEvent>,
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CelMaterial>>,
    mut loadout_list: ResMut<AttackLoadoutList>,
    mut dash_list: ResMut<DashLoadoutList>,
) {
    for (entity, player, mut transform, 
         mut dash, mut attack, 
         mut attack_loadout, mut dash_loadout, stats, mut movement) in &mut query {
        dash.time.tick(time.delta());
        attack.time.tick(time.delta());

        let default_attack = &AttackLoadout { damage: 0.0, cooldown: 0.0, duration: 0.0, action: default_attack_action, path: BezierCurve::default() };

        let default_dash = &DashLoadout { damage: 0.0, cooldown: 0.0, duration: 0.0, path: BezierCurve::default() };

        let actual_attack_loadout = match attack_loadout {
            Some(attack_loadout) => attack_loadout,
            None => default_attack 
        };

        let actual_dash_loadout = match dash_loadout {
            Some(dash_loadout) => dash_loadout,
            None => default_dash };

        if !attack.direction.eq(&Vec3::ZERO) && ( attack.time.elapsed_secs() > actual_attack_loadout.cooldown || attack.time.paused()) {
            attack.time.unpause();
            attack.time.reset();
            (actual_attack_loadout.action)(
                &mut commands, &mut meshes, &mut materials, 
                transform.translation, attack.direction, player.id, 
                actual_attack_loadout.duration, actual_attack_loadout.path.clone());
        }


        if !dash.direction.eq(&Vec3::ZERO) && !dash.is_dashing && ( dash.time.elapsed_secs() > actual_dash_loadout.cooldown || dash.time.paused()) {
            dash.time.unpause();
            dash.time.reset();
            dash.is_dashing = true;
        }

        for ev in ev_coll.iter() {
            if ev.source.index() == entity.index() && !ev.is_trigger {
                movement.direction += ev.direction.round();
                dash.is_dashing = false;
                if ev.direction.eq(&Vec3::ZERO) {
                    transform.translation += Vec3::new(0.0, 0.0, 1.0) * time.delta_seconds() ;
                } 
            }
        }

        if !dash.is_dashing {
            transform.translation += movement.direction * stats.speed * time.delta_seconds();
            transform.translation.y = 0.5;
        } else {
            transform.translation = actual_dash_loadout.path.clone()
                .rotate_points(dash.direction)
                .start_from_point(dash.start)
                .compute(dash.time.elapsed_secs() / actual_dash_loadout.duration);

        }


        if dash.time.elapsed_secs() / actual_dash_loadout.duration >= 1.0 {
            dash.is_dashing = false;
        }
    }

}

pub fn player_input_keyboard(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands, 
    mut query: Query<(Entity, &Player, &mut Transform,
                    &mut Dash, &mut Attack, &Stats, &mut PlayerMovement, 
                    Without<GamepadInput>)>,
    mut loadout_list: ResMut<AttackLoadoutList>,
    mut dash_list: ResMut<DashLoadoutList>,

) {
    for (entity, player, mut transform, 
         mut dash, mut attack, stats, mut movement, keyboard) in &mut query {

        let mut movement_direction: Vec3 = Vec3::default(); 
        let mut attack_direction: Vec3 = Vec3::default();
        let mut dash_direction: Vec3 = Vec3::default();

        if keyboard_input.pressed(KeyCode::W) {
            movement_direction += Vec3::new(0.0, 0.0, -1.0)
        }
        if keyboard_input.pressed(KeyCode::A) {
            movement_direction += Vec3::new(-1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::D) {
            movement_direction += Vec3::new(1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::S) {
            movement_direction += Vec3::new(0.0, 0.0, 1.0)
        }

        if keyboard_input.pressed(KeyCode::Up) {
            attack_direction += Vec3::new(0.0, 0.0, -1.0)
        }
        if keyboard_input.pressed(KeyCode::Down) {
            attack_direction += Vec3::new(0.0, 0.0, 1.0)
        }
        if keyboard_input.pressed(KeyCode::Right) {
            attack_direction += Vec3::new(1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Left) {
            attack_direction += Vec3::new(-1.0, 0.0, 0.0)
        }

        if keyboard_input.pressed(KeyCode::LShift) {
            dash_direction = movement_direction;
        }
        if keyboard_input.pressed(KeyCode::Key1) {
            commands.entity(entity).remove::<AttackLoadout>();
            commands.entity(entity).insert(loadout_list.list[0].clone());

        }
        if keyboard_input.pressed(KeyCode::Key2) {
            commands.entity(entity).remove::<AttackLoadout>();
            commands.entity(entity).insert(loadout_list.list[1].clone());
        }
        if keyboard_input.pressed(KeyCode::Key3) {
            commands.entity(entity).remove::<DashLoadout>();
            commands.entity(entity).insert(dash_list.list[0].clone());
        }


        movement.direction = movement_direction;
        attack.direction = attack_direction;
        if !dash.is_dashing {
            dash.direction = dash_direction;
            dash.start = transform.translation;
        }

    }
}


pub fn player_input_gamepad(
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    mut query: Query<(Entity, &Player, &mut Transform, 
                      &mut Dash, &mut Attack, &Stats, &mut PlayerMovement, &GamepadInput)>,
) {
    for (entity, player, mut transform, mut dash, mut attack, stats, mut movement, gamepad_id) in &mut query {
        let id = gamepad_id;
        let gamepad = Gamepad {id: id.0 as usize};

        let axis_lx = GamepadAxis { gamepad, axis_type: GamepadAxisType::LeftStickX };
        let axis_ly = GamepadAxis { gamepad, axis_type: GamepadAxisType::LeftStickY };

        let axis_rx = GamepadAxis { gamepad, axis_type: GamepadAxisType::RightStickX };
        let axis_ry = GamepadAxis { gamepad, axis_type: GamepadAxisType::RightStickY };

        let mut movement_direction = Vec3::ZERO;
        let mut attack_direction = Vec3::ZERO;
        let mut dash_direction = Vec3::ZERO;
        if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
            movement_direction = Vec3::new(x, 0.0, -y).round();
        }

        if let (Some(x), Some(y)) = (axes.get(axis_rx), axes.get(axis_ry)) {
            attack_direction = Vec3::new(x, 0.0, -y).round();
        }

        // In a real game, the buttons would be configurable, but here we hardcode them
        let attack_button = GamepadButton { gamepad, button_type: GamepadButtonType::South };
        let dash_button = GamepadButton { gamepad, button_type: GamepadButtonType::RightTrigger };
        if buttons.just_pressed(attack_button) {
        }
        if buttons.just_pressed(dash_button) {
            dash_direction = movement_direction;
        }

        movement.direction = movement_direction;
        attack.direction = attack_direction;
        if !dash.is_dashing {
            dash.direction = dash_direction;
            dash.start = transform.translation;
        }
    }
}


pub fn spawn_keyboard_player(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CelMaterial>>,
    mut player_count: ResMut<PlayerNumber>,
    mut attack_list: ResMut<AttackLoadoutList>,
    mut dash_list: ResMut<DashLoadoutList>,

){
    spawn_player(&mut commands, &mut meshes, &mut materials, &mut player_count, 1, ControllerType::Keyboard, &mut attack_list, &mut dash_list);
}



pub fn gamepad_connections(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CelMaterial>>,

    mut query: Query<(Entity, &Player, &GamepadInput)>,
    mut player_count: ResMut<PlayerNumber>,

    mut connection_events: EventReader<GamepadConnectionEvent>,
    mut attack_list: ResMut<AttackLoadoutList>,
    mut dash_list: ResMut<DashLoadoutList>,
) {
    for connection_event in connection_events.iter() {
        let gamepad = connection_event.gamepad;

        if let GamepadConnection::Connected(info) = &connection_event.connection {
            info!("{:?} Connected", gamepad);
            spawn_player(&mut commands, &mut meshes, &mut materials, &mut player_count, gamepad.id as u32, ControllerType::Gamepad, &mut attack_list, &mut dash_list);
        } else {
            info!("{:?} Disconnected", gamepad);

            for (entity, player, PlayerGamepad) in &mut query {
                if PlayerGamepad.0 == gamepad.id as u32 {
                   if let Some(mut entity_commands) = commands.get_entity(entity) {
                    entity_commands.despawn()
                   }
                }
            }

        }
    }
}

#[derive(PartialEq)]
enum ControllerType {
    Gamepad,
    Keyboard
}

fn spawn_player(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<CelMaterial>>,
    mut player_count: &mut ResMut<PlayerNumber>,
    player_id: u32,
    controller_type: ControllerType,
    mut loadout_list: &mut ResMut<AttackLoadoutList>,
    mut dash_list: &mut ResMut<DashLoadoutList>,
) {
    let mut dash_stop_watch = Stopwatch::new();
    dash_stop_watch.reset();
    dash_stop_watch.pause();

    let mut attack_stop_watch = Stopwatch::new();
    attack_stop_watch.reset();
    attack_stop_watch.pause();

    player_count.0 += 1;

    let mut player = commands.spawn(MaterialMeshBundle {
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
    });
    player.insert(Name::new(format!("Player {}", player_count.0)))
    .insert(Player { id: player_id })
    .insert(Stats { 
        speed: 3.0,
        health: 3
    })
    .insert(Collider  {
        x_size: 0.2,
        y_size: 0.5,
        z_size: 0.2,
        is_trigger: false
    })
    .insert(Dash {
        time: dash_stop_watch,
        is_dashing: false,
        direction: Vec3::ZERO,
        start: Vec3::ZERO,
    })
    .insert(Attack {
        time: attack_stop_watch,
        direction: Vec3::ZERO,
    })
    .insert(loadout_list.list[0].clone())
    .insert(PlayerMovement {
        direction: Vec3::ZERO,
    });

    if controller_type == ControllerType::Gamepad {
        player.insert(GamepadInput(player_id));
    }

    info!("Aggiunto un giocatore. Numero attuale: {}", player_count.0);
}
