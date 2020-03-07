extern crate specs;
use crate::{
    EnergyReserve, Female, GoOnTarget, Hunger, InHeat, Male, MyTurn, Point, Position, Reproduction,
    SearchScope, Specie,
};
use specs::prelude::*;
use std::collections::HashMap;

pub struct SearchParterAI {}

impl<'a> System<'a> for SearchParterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, MyTurn>,
        WriteStorage<'a, InHeat>,
        WriteStorage<'a, Male>,
        WriteStorage<'a, Female>,
        WriteStorage<'a, EnergyReserve>,
        WriteStorage<'a, Reproduction>, //bad name to change
        WriteStorage<'a, Specie>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, GoOnTarget>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut turns,
            mut in_heats,
            males,
            females,
            energy_reserves,
            reprods,
            species,
            positions,
            mut go_targets,
        ) = data;

        //reset in_heats
        in_heats.clear();

        //creat hashmap of all in heat female
        let mut in_heat_hash: HashMap<String, Vec<Entity>> = HashMap::new();

        let mut turn_done = Vec::new();

        //female heat
        //1 a female have a hight enought level of energy
        //2 put her in heat

        for (entity, _female, _turn, energy, reprod, specie) in (
            &entities,
            &females,
            &turns,
            &energy_reserves,
            &reprods,
            &species,
        )
            .join()
        {
            if energy.reserve > reprod.threshold() as f32 {
                //println!("female in heat");
                in_heats
                    .insert(entity, InHeat {})
                    .expect("Unable to insert");
                let entry = in_heat_hash
                    .entry(specie.name.clone())
                    .or_insert(Vec::new());
                entry.push(entity);
            }
        }

        //1 a male have a hight level of energy and can reproduce

        //It's still his turn (no other action this turn)

        //He search for all the female of his specie that are in heat

        //He go in the closer, female in heat, he try to go the closer possible

        //male searching woman
        for (entity, _male, _turn, energy, specie, pos) in (
            &entities,
            &males,
            &turns,
            &energy_reserves,
            &species,
            &positions,
        )
            .join()
        {
            if energy.hunger == Hunger::Full {
                let mut choosen_mate = None;
                let mut min = std::f32::MAX;
                //println!("male in ok");

                if let Some(mates) = in_heat_hash.get(&specie.name) {
                    for mate in mates.iter() {
                        let mate_pos = positions.get(*mate).unwrap();

                        let distance = rltk::DistanceAlg::Pythagoras.distance2d(
                            Point::new(pos.x(), pos.y()),
                            Point::new(mate_pos.x(), mate_pos.y()),
                        );
                        if distance < min {
                            choosen_mate = Some(mate);
                            min = distance;
                        }
                    }
                }
                if let Some(mate) = choosen_mate {
                    //Go on the mate positions
                    //println!("male with female");
                    go_targets
                        .insert(
                            entity,
                            GoOnTarget {
                                target: *mate,
                                scope: SearchScope::Big,
                            },
                        )
                        .expect("Unable to insert");
                    turn_done.push(entity);
                }
            }
        }

        // Remove turn marker for those that are done
        for done in turn_done.iter() {
            turns.remove(*done);
        }
    }
}
