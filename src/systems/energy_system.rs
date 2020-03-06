extern crate specs;
use crate::{
    gamelog::{GameLog, GeneralLog},
    Dead, DeathCause, EnergyReserve, Hunger,
};
use specs::prelude::*;

pub struct EnergySystem {}

impl<'a> System<'a> for EnergySystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, GeneralLog>,
        WriteStorage<'a, EnergyReserve>,
        WriteStorage<'a, Dead>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut log, mut general_log, mut energy_reserves, mut deads) = data;

        for (entity, mut en_res) in (&entities, &mut energy_reserves).join() {
            //consumption of energy
            en_res.reserve -= en_res.base_consumption;

            if en_res.reserve <= 0.0 {
                //kill entity

                deads
                    .insert(
                        entity,
                        Dead {
                            cause: DeathCause::Natural,
                        },
                    )
                    .expect("Unable to inset");

                log.entries
                    .insert(0, format!("A entity is dead of starvation."));
                general_log
                    .entries
                    .push(format!("Entity {} is dead of starvation.", entity.id()));
            } else if en_res.reserve < (en_res.max_reserve / 2.0) {
                en_res.hunger = Hunger::Hungry;
            } else {
                en_res.hunger = Hunger::Full;

                //reproduce
            }
        }
    }
}
