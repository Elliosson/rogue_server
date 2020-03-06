extern crate specs;
use crate::{
    gamelog::{GameLog, GeneralLog},
    Aging, Dead, DeathCause, Speed,
};
use specs::prelude::*;

pub struct AgingSystem {}

impl<'a> System<'a> for AgingSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, GeneralLog>,
        WriteStorage<'a, Aging>,
        WriteStorage<'a, Speed>,
        WriteStorage<'a, Dead>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut log, mut general_log, mut agings, mut speeds, mut deads) = data;

        //For now just kill when the creature reach the life expetancy
        for (entity, aging) in (&entities, &mut agings).join() {
            aging.age += 1;
            if aging.age > aging.life_expectancy {
                //kill him

                deads
                    .insert(
                        entity,
                        Dead {
                            cause: DeathCause::Natural,
                        },
                    )
                    .expect("Unable to inset");

                log.entries
                    .insert(0, format!("A entity is dead of  old age."));
                general_log
                    .entries
                    .push(format!("Entity {} is dead of  old age.", entity.id()));
            }
        }

        //while aging the speed diminish (less move point per turn)
        for (_entity, aging, speed) in (&entities, &agings, &mut speeds).join() {
            /*if aging.age < aging.life_expectancy / 10 {
                let aging_factor: f32 = (aging.age) as f32 / (aging.life_expectancy as f32 / 10.0);
                speed.point_per_turn = (speed.base_point_per_turn as f32 * aging_factor) as i32;
            } else */
            if aging.age > aging.life_expectancy / 2 {
                let aging_factor: f32 = (aging.life_expectancy - aging.age) as f32
                    / (aging.life_expectancy as f32 / 2.0);
                speed.point_per_turn = (speed.base_point_per_turn as f32 * aging_factor) as i32;
            } else {
                speed.point_per_turn = speed.base_point_per_turn;
            }
        }
    }
}
