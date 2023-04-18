use bevy::{prelude::*, render::camera::ScalingMode};
use snowball_fight::CelMaterial;
use crate::collision::components::Collider;

pub fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
    });
}

pub fn spawn_walls (
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CelMaterial>>,
) {
    let light_position: Vec3 = Vec3::new(4.0, 2.0, 4.0);
// cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(3.0, 1.0, 1.0))),
        material: materials.add(CelMaterial {
            color: Color::YELLOW,
            light: light_position,
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(3.0, 0.5, 0.0),
        ..default()
    })
    .insert(Name::new("Wall"))
    .insert(Collider {
        x_size: 1.5,
        y_size: 0.5,
        z_size: 0.5
    });


    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(3.0, 1.0, 1.0))),
        material: materials.add(CelMaterial {
            color: Color::RED,
            light: light_position,
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(-3.0, 0.5, 0.0),
        ..default()
    })
    .insert(Collider {
        x_size: 1.5,
        y_size: 0.5,
        z_size: 0.5
    });

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(3.0, 1.0, 1.0))),
        material: materials.add(CelMaterial {
            color: Color::GREEN,
            light: light_position,
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(0.0, 0.5, -3.0),
        ..default()
    })
    .insert(Collider {
        x_size: 1.5,
        y_size: 0.5,
        z_size: 0.5
    });
}
pub fn spawn_basic_scene(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials_std: ResMut<Assets<StandardMaterial>>,
) {
    let light_position: Vec3 = Vec3::new(2.3, 6.7, -0.6);
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(10.0).into()),
        material: materials_std.add(StandardMaterial {
            base_color: Color::WHITE,
            metallic: 0.1,
            ..default()
        }),
        ..default()
    });
    
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.3, 6.7, -0.6),
        ..default()
    })
    ;
    // camera
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(0.0, 4.0, 5.0)
            .with_rotation(Quat::from_xyzw(-0.3, 0.0, 0.0, 0.9)), //.looking_at(Vec3::X, Vec3::Y),
        ..default()
    });
}

pub fn update_light(
    mut query_light: Query<(&PointLight, &Transform)>,
    mut query_materials: Query<&Handle<CelMaterial>>,
    mut materials: ResMut<Assets<CelMaterial>>,
) 
{
    let mut light_position: Vec3 = Vec3::ZERO;
    for (light, pos) in &mut query_light {
        light_position = pos.translation;

    }

    for handler in &mut query_materials {
        if let Some(material) = materials.get_mut(handler) {
            material.light = light_position;
        }
    }
}
