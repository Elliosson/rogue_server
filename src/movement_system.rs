extern crate specs;
use super::{
    ApplyMove, BlocksTile, EntityMoved, Map, Position, RunState, Speed, Viewshed, MOVE_COST,
};
use specs::prelude::*;

pub struct MovementSystem {}

impl<'a> System<'a> for MovementSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
        WriteStorage<'a, ApplyMove>,
        WriteStorage<'a, EntityMoved>,
        WriteStorage<'a, Viewshed>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, RunState>,
        WriteStorage<'a, Speed>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            mut position,
            blockers,
            entities,
            mut apply_move,
            mut moved,
            mut viewsheds,
            _player_entity,
            mut _runstate,
            mut speeds,
        ) = data;

        // Apply broad movement
        for (entity, movement, mut pos, speed) in
            (&entities, &apply_move, &mut position, &mut speeds).join()
        {
            if speed.move_point >= MOVE_COST {
                let start_idx = map.xy_idx(pos.x, pos.y);
                let dest_idx = movement.dest_idx as usize;
                let is_blocking = blockers.get(entity);
                if is_blocking.is_some() {
                    map.set_blocked(start_idx, false);
                    map.set_blocked(dest_idx, true);
                }
                pos.x = movement.dest_idx % map.width;
                pos.y = movement.dest_idx / map.width;
                if let Some(vs) = viewsheds.get_mut(entity) {
                    vs.dirty = true;
                }
                moved
                    .insert(entity, EntityMoved {})
                    .expect("Unable to insert");

                speed.move_point -= MOVE_COST;
            }
        }
        apply_move.clear();
    }
}
