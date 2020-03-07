extern crate specs;
use crate::{
    gamelog::{GameLog, WorldStatLog},
    Date, Map,
};

use specs::prelude::*;

pub struct HumiditySystem {}

impl<'a> System<'a> for HumiditySystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, WorldStatLog>,
        WriteExpect<'a, Map>,
        WriteExpect<'a, Date>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (_entities, _log, mut _world_logs, mut map, _date) = data;

        //Pour l'instant
        //gradiant d'humidity constant entre 20%  et  100% entre l'est et l'ouest'
        //TODO add real humidity system with oceanic courant and all.
        let mim_humidity: f32 = 20.0;
        let humidity_variation: f32 = 80.0;

        //todo be redone with hashmap
        /*
        let len = map.tile_humidity.iter().count();
        for i in 0..len {
            let (x, _y) = map.idx_xy(i);

            //longitude between 0 and 1
            let longitude_ref = x as f32 / map.width as f32;
            let longitude_hum = mim_humidity + longitude_ref * humidity_variation;

            let tile_humidity = longitude_hum;
            map.tile_humidity[i] = tile_humidity;
        }
        */
    }
}
