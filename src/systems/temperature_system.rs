extern crate specs;
use crate::{
    gamelog::{GameLog, WorldStatLog},
    Date, Map,
};

use specs::prelude::*;

pub struct TemperatureSystem {}

impl<'a> System<'a> for TemperatureSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, WorldStatLog>,
        WriteExpect<'a, Map>,
        WriteExpect<'a, Date>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (_entities, _log, mut _world_logs, mut map, date) = data;

        //Pour l'instant
        //gradiant de temperature constant de 15C entre le nord le sud
        let latitude_variation = 15.0;
        //temperature entre plus frid et plus chaud
        //1jour de l'anné remp min, temp max au jour 182
        let saisonal_variation = 5.0;

        //"value" of the day in term of temperature (between 0 and 1 )
        let day_ref: f32 = (date.get_day() as f32 - (Date::YEAR_DURATION / 2) as f32).abs()
            / ((Date::YEAR_DURATION / 2) as f32);

        let day_temp: f32 = day_ref * saisonal_variation;

        //todo be redone with hash map
        /*
        let len = map.tile_temperature.iter().count();
        for i in 0..len {
            let (_x, y) = map.idx_xy(i);

            //latidute between 0 and 1
            let latitude_ref = y as f32 / map.height as f32;
            let latitude_temp = latitude_ref * latitude_variation;

            let tile_temp = day_temp + latitude_temp;
            map.tile_temperature[i] = tile_temp;
        }
        */
    }
}
