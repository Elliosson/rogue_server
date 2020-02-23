extern crate specs;
use crate::{Map, Position, Viewshed, WantToMove, WINDOWHEIGHT, WINDOWWIDTH};
use specs::prelude::*;
use std::cmp::{max, min};

pub struct WantToMoveSystem {}

impl<'a> System<'a> for WantToMoveSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteExpect<'a, Map>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, WantToMove>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut positions, map, mut viewshed, mut want_to_moves) = data;

        for (_entity, pos, viewshed, want_to_move) in
            (&entities, &mut positions, &mut viewshed, &mut want_to_moves).join()
        {
            if pos.x + want_to_move.delta_x < 1
                || pos.x + want_to_move.delta_x > map.width - 1
                || pos.y + want_to_move.delta_y < 1
                || pos.y + want_to_move.delta_y > map.height - 1
            {
                break;
            }
            let destination_idx =
                map.xy_idx(pos.x + want_to_move.delta_x, pos.y + want_to_move.delta_y);
            if !map.blocked[destination_idx] {
                pos.x = min(WINDOWWIDTH as i32 - 1, max(0, pos.x + want_to_move.delta_x));
                pos.y = min(
                    WINDOWHEIGHT as i32 - 1,
                    max(0, pos.y + want_to_move.delta_y),
                );
                viewshed.dirty = true;
            }
        }

        want_to_moves.clear();
    }
}
