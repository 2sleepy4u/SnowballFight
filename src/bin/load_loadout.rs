use serde::{Deserialize, Serialize};
use std::fs::{File, read_dir};
use serde_yaml::{self};
use bevy::prelude::*;

mod loadout;
use crate::loadout::components::*;
use snowball_fight::bezier::BezierCurve;

#[derive(Debug, Serialize, Deserialize)]
struct Loadout {
    damage: f32,
    cooldown: f32,
    duration: f32,
    path: Vec<Vec3>
}

const ATTACK_DIR_PATH: &str = "assets/loadout/attacks/";

pub fn load_attacks_loadout(path: &str) -> Vec<AttackLoadout> {
    info!("Loading attacks loadout..");
    let paths = read_dir(path).unwrap();
    let mut loadout_list: Vec<AttackLoadout> = Vec::new();

    for path in paths {
        let file_path = path.unwrap().path();
        let file = File::open(file_path).expect("Error opening");
        let loadout: Loadout = serde_yaml::from_reader(file).expect("Error reading");
        let attack_loadout: AttackLoadout = 
            AttackLoadout { 
                damage: loadout.damage, 
                cooldown: loadout.cooldown, 
                duration: loadout.duration, 
                action: default_attack_action, 
                path: BezierCurve::default() 
            };
        loadout_list.push(attack_loadout);
    }

    return loadout_list;
}
