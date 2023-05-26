use bevy::prelude::*;
use snowball_fight::{materials::*, bezier::BezierCurve};

#[derive(Component, Clone)]
pub struct AttackLoadout {
    pub damage: f32,
    pub cooldown: f32,
    pub duration: f32,

    pub action: attack_action,
    pub path: BezierCurve
}

#[derive(Component, Clone)]
pub struct DashLoadout {
    pub damage: f32,
    pub cooldown: f32,
    pub duration: f32,
    pub path: BezierCurve
}

type attack_action = fn (
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<CelMaterial>>,
    spawn_position: Vec3,
    snowball_direction: Vec3,
    player_id: u32, 
    duration: f32,
    path: BezierCurve
    );

pub fn default_attack_action( 
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<CelMaterial>>,
    spawn_position: Vec3,
    snowball_direction: Vec3,
    player_id: u32,
    duration: f32,
    path: BezierCurve
    ){ info!("Default attack action!");}

impl Default for AttackLoadout {
    fn default() -> Self { 
        AttackLoadout { 
            damage: 0.0, 
            cooldown: 0.0, 
            duration: 0.0, 
            action: default_attack_action, 
            path: BezierCurve::default() 
        }
    }
}

impl Default for DashLoadout {
    fn default() -> Self { 
        DashLoadout { 
            damage: 0.0, 
            cooldown: 0.0, 
            duration: 0.0, 
            path: BezierCurve::default() 
        }
    }
}

#[derive(Resource)]
pub struct AttackLoadoutList {
    pub list: Vec<AttackLoadout>
}

#[derive(Resource)]
pub struct DashLoadoutList {
    pub list: Vec<DashLoadout>
}


