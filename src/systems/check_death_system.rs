extern crate specs;
use crate::{
    gamelog::GameLog, Blocking, BlocksTile, CombatStats, Dead, DeathCause, Map, Position,
    Unblocking,
};
use specs::prelude::*;

pub struct CheckDeathSystem {}

impl<'a> System<'a> for CheckDeathSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, Dead>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, combat_stats, mut deads) = data;

        for (entity, combat_stat) in (&entities, &combat_stats).join() {
            if combat_stat.hp <= 0 {
                deads.insert(
                    entity,
                    Dead {
                        cause: DeathCause::Unknown,
                    },
                );
            }
        }
    }
}
