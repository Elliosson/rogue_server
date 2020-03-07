extern crate specs;
use crate::{gamelog::GameLog, EnergyReserve, HumiditySensitive, Map, Position};
use specs::prelude::*;

pub struct HumiditySensitivitySystem {}

impl<'a> System<'a> for HumiditySensitivitySystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, HumiditySensitive>,
        WriteStorage<'a, EnergyReserve>,
        WriteStorage<'a, Position>,
        ReadExpect<'a, Map>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, hum_sensitivities, mut energy_reserves, positions, map) = data;

        // For now the humidity impact in comsumption of energy
        for (_entity, hum_sens, eng_res, pos) in (
            &entities,
            &hum_sensitivities,
            &mut energy_reserves,
            &positions,
        )
            .join()
        {
            //TODO be redone with hash map
            /*
            //get humidity
            let idx = map.xy_idx(pos.x, pos.y);
            let humidity = map.tile_humidity[idx];

            //For now he just have a consuption proportionnal to square the distance with the optimum
            //the k factor will moderate le curb of the square
            let energy_consuption = (humidity as f32 - hum_sens.optimum)
                * (humidity as f32 - hum_sens.optimum)
                * hum_sens.k;

            eng_res.reserve -= energy_consuption;
            */
        }
    }
}
