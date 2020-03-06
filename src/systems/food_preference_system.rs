extern crate specs;
use crate::{gamelog::GameLog, Carnivore, EnergyReserve, FoodPreference, FoodType, Herbivore};
use specs::prelude::*;
use std::collections::BTreeMap;

pub struct FoodPreferenceSystem {}

impl<'a> System<'a> for FoodPreferenceSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, FoodPreference>,
        WriteStorage<'a, Carnivore>,
        WriteStorage<'a, Herbivore>,
        WriteStorage<'a, EnergyReserve>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, mut food_prefs, carnivores, herbivores, energies) = data;

        for (entity, carnivore, herbivore, energy) in
            (&entities, &carnivores, &herbivores, &energies).join()
        {
            //todo not ok, must alway hunt his favorite food when hungry
            //println!("food pref");
            let mut choices = BTreeMap::new();
            choices.insert(
                (carnivore.digestion * 0.55 * 1.1 * energy.max_reserve as f32) as i32,
                FoodType::Meat,
            );
            //*0.9 is for prioritify eating meat over killing
            choices.insert(
                (carnivore.digestion * 0.55 * energy.max_reserve as f32) as i32,
                FoodType::Animal,
            );
            choices.insert(
                (herbivore.digestion * 0.55 * energy.max_reserve as f32) as i32,
                FoodType::Vegetable,
            );
            food_prefs
                .insert(entity, FoodPreference { choices })
                .expect("Unable to insert");
        }
    }
}
