extern crate specs;
use crate::{gamelog::GameLog, Dead, OnlinePlayer, Respawn, ToDelete};
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
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, mut to_deletes, mut deads, online_players, mut respawns) = data;

        for (entity, _dead) in (&entities, &mut deads).join() {
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
