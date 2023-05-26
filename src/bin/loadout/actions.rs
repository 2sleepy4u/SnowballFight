use bevy::prelude::*;
use bevy::time::Stopwatch;
use snowball_fight::bezier::BezierCurve;
use crate::snowball::components::Snowball;
use crate::collision::components::Collider;
use snowball_fight::materials::*;

pub fn spawn_snowball(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<CelMaterial>>,
    spawn_position: Vec3,
    snowball_direction: Vec3,
    player_id: u32,
    duration: f32,
    path: BezierCurve
)
{
    let mut stop_watch = Stopwatch::new();
    stop_watch.reset();

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: 0.2,
            ..default()
        })),
        material: materials.add(CelMaterial {
            color: Color::WHITE,
            light: Vec3::new(4.0, 2.0, 4.0),
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_translation(spawn_position),
        ..default()
    })
    .insert(Name::new("Snowball"))
    .insert(Snowball {
        direction: snowball_direction,
        player_id: player_id,
        start: spawn_position,
        time: stop_watch,
        duration: duration,
        path: path.rotate_points(snowball_direction).start_from_point(spawn_position)
    })
    .insert(Collider {
        x_size: 0.4,
        y_size: 0.4,
        z_size: 0.4,
        is_trigger: true
    });

}

