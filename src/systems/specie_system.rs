extern crate specs;
use crate::{
    gamelog::{GameLog, WorldStatLog},
    Name, Position, Renderable, Specie, TemperatureSensitive,
};
use cogset::{Euclid, Kmeans};
use rltk::HSV;
use specs::prelude::*;
use std::collections::HashMap;

pub struct SpecieSystem {}

impl<'a> System<'a> for SpecieSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Name>,
        WriteExpect<'a, WorldStatLog>,
        WriteStorage<'a, Specie>,
        WriteStorage<'a, TemperatureSensitive>,
        WriteStorage<'a, Renderable>,
        WriteExpect<'a, rltk::RandomNumberGenerator>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            _log,
            _names,
            _world_logs,
            mut species,
            temp_sensis,
            mut renderables,
            mut rng,
            positions,
        ) = data;

        let mut species_hash: HashMap<String, Vec<Entity>> = HashMap::new();

        let max_specie_member = 150;

        //create an hash map of all the member of all specie
        for (entity, specie, _temp_sens) in (&entities, &mut species, &temp_sensis).join() {
            if species_hash.contains_key(&specie.name) {
                let member_list = species_hash.get_mut(&specie.name).unwrap();
                member_list.push(entity);
            } else {
                species_hash.insert(specie.name.clone(), vec![entity]);
            }
        }

        let mut new_species: Vec<Vec<Entity>> = Vec::new();

        //For now just divide the specie acording to the number of member
        for (_name, member_list) in &species_hash {
            if member_list.len() > max_specie_member {
                //New methode
                //Try to divide the species acording to their position with the k-mean algorithm
                let mut data = Vec::new();
                for ent in member_list.iter() {
                    let pos = positions.get(*ent).unwrap();
                    //prepare vec for Kmeans
                    let euclid = Euclid([pos.x() as f64, pos.y() as f64]);
                    data.push(euclid);
                }
                //Do kmean
                let k = 2;
                let kmeans = Kmeans::new(&data, k);
                let clusters = kmeans.clusters();

                //Assing each entity to his new species
                //retrive all the group
                for group in clusters {
                    //list of all the member of the new specie
                    let mut new_specie: Vec<Entity> = Vec::new();
                    //retrieve the index of all the element of the group
                    for idx in group.1 {
                        new_specie.push(member_list[idx]); //TODO supress the 0, the f32 should be temperature optimum and this is no longer usefull
                    }
                    //push in the vec of all the species to create
                    new_species.push(new_specie);
                }
            }
        }

        //change the id of new_species, // also change their appearance
        //also change the name once the name is added to the mutation
        let mut count = 0;
        for new_specie in new_species {
            count += 1;

            //random glyph color for the new specie
            let glyph = rng.roll_dice(1, 255) as u8;

            //TODO find a good thing to have only flashy color
            let mut hue = rng.roll_dice(1, 99) as f32;
            //try to avoid the blue, you see nothing on a dark background
            while hue > 61.0 && hue < 72.0 {
                hue = rng.roll_dice(1, 99) as f32;
            }
            let hue = hue / 100.0;
            for entity in new_specie {
                let mut specie = species.get_mut(entity).unwrap();

                //concatenate ne old name with a number
                specie.name = specie.name.clone() + &count.to_string();

                //change the color of the new specie todo get random color
                let mut renderable = renderables.get_mut(entity).unwrap();

                renderable.glyph = glyph;
                renderable.fg = HSV::from_f32(hue, 0.7, 0.99).to_rgb();
            }
        }
    }
}
