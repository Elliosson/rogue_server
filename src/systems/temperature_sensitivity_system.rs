extern crate specs;
use crate::{gamelog::GameLog, EnergyReserve, Map, Position, TemperatureSensitive};
use specs::prelude::*;

pub struct TemperatureSensitivitySystem {}

impl<'a> System<'a> for TemperatureSensitivitySystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, TemperatureSensitive>,
        WriteStorage<'a, EnergyReserve>,
        WriteStorage<'a, Position>,
        ReadExpect<'a, Map>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, temp_sensitivities, mut energy_reserves, positions, map) = data;

        // For now the temperature impact in comsumption of energy
        for (_entity, temp_sens, eng_res, pos) in (
            &entities,
            &temp_sensitivities,
            &mut energy_reserves,
            &positions,
        )
            .join()
        {
            //get temperature
            let idx = map.xy_idx(pos.x(), pos.y());

            if let Some(temp) = map.tile_temperature.get(&idx) {
                //For now he just have a consuption proportionnal to square the distance with the optimum
                //the k factor will moderate le curb of the square
                let energy_consuption = (*temp as f32 - temp_sens.optimum)
                    * (*temp as f32 - temp_sens.optimum)
                    * temp_sens.k;

                eng_res.reserve -= energy_consuption;
            }
        }
    }
}
