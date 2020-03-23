extern crate specs;
use crate::{GoByStep, Position, WantToMove};
use specs::prelude::*;

pub struct GoStepSystem {}

impl<'a> System<'a> for GoStepSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, GoByStep>,
        WriteStorage<'a, WantToMove>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut go_steps, mut want_moves, positions) = data;

        let mut to_move: Vec<(Entity, i32, i32, Entity)> = Vec::new();

        for (entity, go_step, pos) in (&entities, &go_steps, &positions).join() {
            to_move.push((entity, pos.x(), pos.y(), go_step.target));
        }

        //for now it just a stupid advance
        for (entity, pos_x, pos_y, target) in to_move {
            if let Some(target_pos) = positions.get(target) {
                let dist_x = target_pos.x() - pos_x;
                let dist_y = target_pos.y() - pos_y;
                let mut delta_x = 0;
                let mut delta_y = 0;

                if dist_x.abs() > dist_y.abs() {
                    if dist_x < 0 {
                        delta_x = -1;
                    } else {
                        delta_x = 1;
                    }
                } else {
                    if dist_y < 0 {
                        delta_y = -1;
                    } else {
                        delta_y = 1;
                    }
                }

                want_moves
                    .insert(entity, WantToMove { delta_x, delta_y })
                    .expect("Unable to insert");
            } else {
                //the target have no positon so it is invalide
                go_steps.remove(entity);
            }
        }
        // no clear but I should stop went the entity is destroyer
        // todo handle when there is no move target

        // todo add the turn check to limite the possible action once we have a real monster ai
    }
}
