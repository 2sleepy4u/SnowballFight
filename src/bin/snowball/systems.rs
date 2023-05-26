use bevy::prelude::*;

use super::components::*;
use snowball_fight::bezier::*;
use crate::{collision::events::CollisionEvent, player::components::Player};

pub fn move_snowballs(
    mut query: Query<(Entity, &mut Transform, &mut Snowball)>,
    time: Res<Time>,
    mut commands: Commands, 
){
    for (mut entity, mut transform, mut snowball) in &mut query {
        snowball.time.tick(time.delta());
        let t = snowball.time.elapsed_secs() / snowball.duration;

        if t >= 1.0 {
            commands.entity(entity).despawn();
        } else {
            transform.translation = snowball.path.compute(t);
        }

    }
}

pub fn check_collision(
    mut commands: Commands, 
    mut player_query: Query<(Entity, &Player)>,
    mut query: Query<(Entity, &Snowball)>,
    mut ev_coll: EventReader<CollisionEvent>,
){
    for ev in ev_coll.iter() {
        for (entity, snowball) in &mut query {
            if let Ok((player_entity, player )) = player_query.get_mut(ev.target) {
                if ev.source.index() == entity.index() {
                    if player.id != snowball.player_id {
                        commands.entity(entity).despawn();
                    }
                } 
            }
        }
    }
}
