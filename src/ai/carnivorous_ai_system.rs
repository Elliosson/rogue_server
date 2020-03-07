extern crate specs;
use crate::{
    Carnivore, GoOnTarget, Herbivore, Map, Point, Position, RunState, SearchScope, TargetReached,
    Viewshed, WantToEat, WantsToFlee,
};
use specs::prelude::*;
extern crate rltk;
use std::collections::HashMap;
//use std::time::{Duration, Instant};

pub struct CarnivorousAI {}

impl<'a> System<'a> for CarnivorousAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, RunState>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Herbivore>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, WantToEat>,
        WriteStorage<'a, Carnivore>,
        WriteStorage<'a, TargetReached>,
        WriteStorage<'a, GoOnTarget>,
        WriteStorage<'a, WantsToFlee>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            map,
            _runstate,
            entities,
            viewsheds,
            herbivores,
            mut positions,
            mut want_to_eats,
            carnivores,
            target_reacheds,
            mut go_targets,
            mut flees,
        ) = data;

        //TODO add hunger condition to hunt

        let mut targets: HashMap<Entity, Entity> = HashMap::new();

        //check if we managed to get a target
        for (entity, _carnivore, _pos) in (&entities, &carnivores, &mut positions).join() {
            if let Some(reached) = target_reacheds.get(entity) {
                //TODO for now it eat directly I must add a fight
                want_to_eats
                    .insert(
                        entity,
                        WantToEat {
                            target: reached.target,
                        },
                    )
                    .expect("Unable to insert");

            //TODO do not search a new target if the entity is already eating
            } else {
                //println!("no target reached");
            }
        }

        //Chose target to go, for now it's just Herbivore
        //TODO supress herbivore to have all sort of target
        for (entity, viewshed, _carnivore) in (&entities, &viewsheds, &carnivores).join() {
            //search for every herbivore in the viewshed
            let mut found_herbivores: Vec<Entity> = Vec::new();
            for visible_tile in viewshed.visible_tiles.iter() {
                let idx = map.xy_idx(visible_tile.x, visible_tile.y);
                if let Some(tile_content) = map.tile_content.get(&idx) {
                    for maybe_herbivore in tile_content.iter() {
                        if let Some(_herbivore) = herbivores.get(*maybe_herbivore) {
                            found_herbivores.push(*maybe_herbivore);
                        }
                    }
                }
            }

            //Choose the closer target
            let mut choosen_herbivore: Option<Entity> = None;
            let mut min: f32 = std::f32::MAX;
            for herbivore in found_herbivores {
                let herbivore_pos = positions.get(herbivore).unwrap();
                let pos = positions.get(entity).unwrap();
                let distance = rltk::DistanceAlg::Pythagoras.distance2d(
                    Point::new(pos.x(), pos.y()),
                    Point::new(herbivore_pos.x(), herbivore_pos.y()),
                );
                if distance < min {
                    choosen_herbivore = Some(herbivore);
                    min = distance;
                }
            }
            if let Some(choosen_target) = choosen_herbivore {
                targets.insert(entity, choosen_target);
                go_targets
                    .insert(
                        entity,
                        GoOnTarget {
                            target: choosen_target,
                            scope: SearchScope::Small,
                        },
                    )
                    .expect("Unable to insert");

                //tell the target to flee //TODO do a real system with done insert and all
                let pos = positions.get(entity).unwrap();
                let idx = map.xy_idx(pos.x(), pos.y()) as i32;
                let mut flee_list = Vec::new();
                flee_list.push(idx);

                flees
                    .insert(choosen_target, WantsToFlee { indices: flee_list })
                    .expect("Unable to insert");
            }
        }
    }
}
