extern crate specs;

use crate::{
    gamelog::GameLog, CombatStats, HaveRespawnPoint, Map, OnlinePlayer, Position, Respawn,
    STARTING_POS_X, STARTING_POS_Y,
};

use specs::prelude::*;

pub struct RespawnSystem {}

impl<'a> System<'a> for RespawnSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, OnlinePlayer>,
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, Respawn>,
        WriteStorage<'a, HaveRespawnPoint>,
        WriteExpect<'a, Map>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            _log,
            mut positions,
            online_players,
            mut combat_stats,
            mut respawns,
            have_respawn_points,
            mut map,
        ) = data;

        let mut to_respawn = Vec::new();

        for (entity, _respawn, _online_player) in (&entities, &respawns, &online_players).join() {
            if let Some(respawn_point) = have_respawn_points.get(entity) {
                if let Some(pos) = positions.get(respawn_point.respawn_point) {
                    to_respawn.push((entity, pos.x(), pos.y()));
                } else {
                    to_respawn.push((entity, STARTING_POS_X, STARTING_POS_Y));
                }
            } else {
                to_respawn.push((entity, STARTING_POS_X, STARTING_POS_Y));
            }
        }

        for (entity, x, y) in to_respawn.iter() {
            if let Some(pos) = positions.get_mut(*entity) {
                pos.moving(*x, *y, &mut map.dirty);
            } else {
                println!("Error: entity to respawn have no positon")
            }

            if let Some(combat_stat) = combat_stats.get_mut(*entity) {
                combat_stat.hp = combat_stat.max_hp;
            } else {
                println!("Error: entity to respawn have no combat stat")
            }
        }

        respawns.clear();
    }
}
