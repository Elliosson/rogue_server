extern crate specs;
use crate::{gamelog::GameLog, Position, ToDelete, ToSpawnList, Vegetable};

use specs::prelude::*;

pub struct VegetableGrowSystemV2 {}

impl<'a> System<'a> for VegetableGrowSystemV2 {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, Vegetable>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, ToDelete>,
        WriteExpect<'a, ToSpawnList>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, mut vegetables, positions, mut to_deletes, mut to_spawns) = data;

        for (entity, vegetable, pos) in (&entities, &mut vegetables, &positions).join() {
            if vegetable.completion > 1.0 {
                //end of grow
                for product in vegetable.product.iter() {
                    to_spawns.request(pos.x(), pos.y(), product.clone());
                }
                // for now it's always destroy the vegetable, after I can add an option to let him alive
                to_deletes
                    .insert(entity, ToDelete {})
                    .expect("Unable to insert");
            } else {
                vegetable.completion += 1.0 / vegetable.grow_time as f32;
            }
        }
    }
}
