

pub fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(Entity, 
                    &Player, &mut Transform,
                    &mut Dash, &mut Attack, 
                    //&mut AttackLoadout, &mut DashLoadout, &Stats,
                    Option<&AttackLoadout>, Option<&DashLoadout>,
                    &Stats,
                    Without<GamepadInput>)>,
    mut ev_coll: EventReader<CollisionEvent>,
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CelMaterial>>,
    mut loadout_list: ResMut<AttackLoadoutList>,
    mut dash_list: ResMut<DashLoadoutList>,
) 
{
    let mut direction: Vec3 = Vec3::default(); 
    let mut attack_direction: Vec3 = Vec3::default();

    for (entity, player, mut transform, 
         mut dash, mut attack, 
         mut attack_loadout, mut dash_loadout, stats, keyboard) in &mut query {

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
            None => default_dash
        };


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
        if keyboard_input.pressed(KeyCode::Key4) {
        }

        if !attack_direction.eq(&Vec3::ZERO) && ( attack.time.elapsed_secs() > actual_attack_loadout.cooldown || attack.time.paused()) {
            attack.time.unpause();
            attack.time.reset();

            (actual_attack_loadout.action)(
                &mut commands, &mut meshes, &mut materials, 
                transform.translation, attack_direction, player.id, 
                actual_attack_loadout.duration, actual_attack_loadout.path.clone());
        }

            
         if keyboard_input.pressed(KeyCode::LShift) && ( dash.time.elapsed_secs() > actual_dash_loadout.cooldown || dash.time.paused()) {
            dash.time.unpause();
            dash.time.reset();
            dash.is_dashing = true;
            dash.direction = direction;
            dash.start = transform.translation;
        }

        for ev in ev_coll.iter() {
            if ev.source.index() == entity.index() && !ev.is_trigger {
                direction += ev.direction.round();
                dash.is_dashing = false;
                if ev.direction.eq(&Vec3::ZERO) {
                    transform.translation += Vec3::new(0.0, 0.0, 1.0) * time.delta_seconds() ;
                } 
            }
        }

        if !dash.is_dashing {
            transform.translation += direction * stats.speed * time.delta_seconds();
            transform.translation.y = 0.5;
        } else {

            //FIX
            if dash.direction.eq(&Vec3::ZERO) {
                dash.direction = Vec3::new(1.0, 0.0, 0.0);

            }
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
