use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{File, read_dir};
use serde_yaml::{self};
use crate::loadout::components::{DashLoadoutList, AttackLoadout, AttackLoadoutList, DashLoadout, default_attack_action};
use snowball_fight::bezier::BezierCurve;

use super::actions::spawn_snowball;

fn do_attack(){}

pub fn load_loadout(
    mut attack_loadout_list: ResMut<AttackLoadoutList>,
    mut dash_loadout_list: ResMut<DashLoadoutList>
){
    attack_loadout_list.list = load_attacks_loadout(ATTACK_DIR_PATH);
    dash_loadout_list.list = load_dashes_loadout(ATTACK_DIR_PATH);
}

const ATTACK_DIR_PATH: &str = "assets/loadout/attacks/";
const DASH_DIR_PATH: &str = "assets/loadout/dashes/";

#[derive(Debug, Serialize, Deserialize)]
struct Loadout {
    damage: f32,
    cooldown: f32,
    duration: f32,
    mode: String,
    path: Vec<Vec3>
}
pub fn load_dashes_loadout(path: &str) -> Vec<DashLoadout> {
    info!("Loading dashes loadout...");
    let paths = read_dir(path).unwrap();
    let mut loadout_list: Vec<DashLoadout> = Vec::new();

    for path in paths {
        let file_path = path.unwrap().path();
        let file = File::open(file_path).expect("Error opening");
        let loadout: Loadout = serde_yaml::from_reader(file).expect("Error reading");
        let attack_loadout: DashLoadout = 
            DashLoadout { 
                damage: loadout.damage, 
                cooldown: loadout.cooldown, 
                duration: loadout.duration, 
                path: BezierCurve::from_vec(loadout.path) 
            };
        loadout_list.push(attack_loadout);
    }
    return loadout_list;
}

pub fn load_attacks_loadout(path: &str) -> Vec<AttackLoadout> {
    info!("Loading attacks loadout...");
    let paths = read_dir(path).unwrap();
    let mut loadout_list: Vec<AttackLoadout> = Vec::new();

    for path in paths {
        let file_path = path.unwrap().path();
        let file = File::open(file_path).expect("Error opening");
        let loadout: Loadout = serde_yaml::from_reader(file).expect("Error reading");
        let mut attack_loadout: AttackLoadout = 
            AttackLoadout { 
                damage: loadout.damage, 
                cooldown: loadout.cooldown, 
                duration: loadout.duration, 
                action: default_attack_action, 
                path: BezierCurve::from_vec(loadout.path) 
            };

        if loadout.mode.eq("snowball") {
            attack_loadout.action = spawn_snowball;
        }

        loadout_list.push(attack_loadout);
    }
    return loadout_list;
}
