extern crate specs;
use crate::{algo, Map, MyTurn, Position, WantToMove, WantsToApproach};
use specs::prelude::*;

pub struct ApproachAI {}

impl<'a> System<'a> for ApproachAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'a, MyTurn>,
        WriteStorage<'a, WantsToApproach>,
        ReadStorage<'a, Position>,
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, WantToMove>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut turns, mut want_approach, positions, mut map, entities, mut want_moves) = data;

        let mut turn_done: Vec<Entity> = Vec::new();
        for (entity, pos, approach, _myturn) in
            (&entities, &positions, &want_approach, &turns).join()
        {
            turn_done.push(entity);

            let idx = map.xy_idx(pos.x(), pos.y());
            let temp_map_blocked = map.is_blocked(approach.idx as usize);
            map.set_blocked(approach.idx as usize, false);

            //let path = rltk::a_star_search(idx, approach.idx, &mut *map);

            let path = algo::a_star_search(
                idx as i32,
                approach.idx, //TODO change that, the "-1" is a dirty fix for the imposibility to go on a blicked tile
                &mut *map,
                64, //Max step for search, TODO thonk of a way to automatically find an acceptable number
            );

            map.set_blocked(approach.idx as usize, temp_map_blocked); //TODO remove it's ugly

            //TODO this is the most inefecient way to change position that I have ever done
            //This is done only on the purpuse to keep the current interface
            //todo refactor if time are critical for this

            if path.success && path.steps.len() > 1 {
                let (dest_x, dest_y) = map.idx_xy(path.steps[1] as usize);
                let delta_x = dest_x - pos.x();
                let delta_y = dest_y - pos.y();
                want_moves
                    .insert(entity, WantToMove { delta_x, delta_y })
                    .expect("Unable to insert");
            }
        }

        want_approach.clear();

        // Remove turn marker for those that are done
        for done in turn_done.iter() {
            turns.remove(*done);
        }
    }
}
