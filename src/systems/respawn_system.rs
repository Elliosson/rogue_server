extern crate specs;

use crate::{gamelog::GameLog, CombatStats, Map, OnlinePlayer, Position, Respawn};

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
            mut map,
        ) = data;

        for (_entity, _respawn, _online_player, pos, combat_stat) in (
            &entities,
            &respawns,
            &online_players,
            &mut positions,
            &mut combat_stats,
        )
            .join()
        {
            //for now just reput him on the original positon, after it will need to be more complex
            pos.moving(5, 5, &mut map.dirty);
            combat_stat.hp = combat_stat.max_hp;
        }

        respawns.clear();
    }
}
