extern crate specs;
use crate::{
    gamelog::GameLog, Dead, DeathLoot, OnlinePlayer, Position, Respawn, ToDelete, ToSpawnList,
};
use specs::prelude::*;

pub struct DeathSystem {}

impl<'a> System<'a> for DeathSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, ToDelete>,
        WriteStorage<'a, Dead>,
        WriteStorage<'a, OnlinePlayer>,
        WriteStorage<'a, Respawn>,
        WriteStorage<'a, DeathLoot>,
        WriteStorage<'a, Position>,
        WriteExpect<'a, ToSpawnList>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            _log,
            mut to_deletes,
            mut deads,
            online_players,
            mut respawns,
            death_loots,
            positions,
            mut to_spawns,
        ) = data;

        for (entity, _dead, pos) in (&entities, &mut deads, &positions).join() {
            if let Some(death_loots) = death_loots.get(entity) {
                for loot_name in &death_loots.loots {
                    to_spawns.request(pos.x(), pos.y(), loot_name.clone());
                }
            }
            if let Some(_online_player) = online_players.get(entity) {
                //respawn the player

                respawns
                    .insert(entity, Respawn {})
                    .expect("Unable to insert");
            } else {
                to_deletes
                    .insert(entity, ToDelete {})
                    .expect("Unable to insert");
            }
        }

        deads.clear();
    }
}
