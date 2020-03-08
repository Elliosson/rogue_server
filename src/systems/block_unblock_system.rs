extern crate specs;
use crate::{gamelog::GameLog, Blocking, BlocksTile, Map, Position, Unblocking};
use specs::prelude::*;

pub struct BlockUnblockSystem {}

impl<'a> System<'a> for BlockUnblockSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, BlocksTile>,
        WriteStorage<'a, Blocking>,
        WriteStorage<'a, Unblocking>,
        WriteStorage<'a, Position>,
        WriteExpect<'a, Map>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, mut blockers, mut blockings, mut unblockings, positions, mut map) =
            data;

        for (entity, _blocking, pos) in (&entities, &mut blockings, &positions).join() {
            blockers
                .insert(entity, BlocksTile {})
                .expect("Unable to insert");
            map.dirty.push((pos.x(), pos.y()));
        }

        for (entity, _unblocking, pos) in (&entities, &unblockings, &positions).join() {
            blockers.remove(entity);
            map.dirty.push((pos.x(), pos.y()));
        }

        blockings.clear();
        unblockings.clear();
    }
}
