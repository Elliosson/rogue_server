extern crate specs;
use crate::{
    GoOnTarget, Herbivore, Leaf, Map, Point, Position, RunState, SearchScope, TargetReached,
    TargetedForEat, Viewshed, WantToEat,
};
use specs::prelude::*;
extern crate rltk;
use std::collections::HashMap;
//use std::time::{Duration, Instant};

pub struct CowAI {}

impl<'a> System<'a> for CowAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, RunState>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Herbivore>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Leaf>,
        WriteStorage<'a, WantToEat>,
        WriteStorage<'a, TargetedForEat>,
        WriteStorage<'a, GoOnTarget>,
        WriteStorage<'a, TargetReached>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            map,
            _runstate,
            entities,
            viewsheds,
            cows,
            mut positions,
            leafs,
            mut want_to_eats,
            mut targeted_eats,
            mut go_targets,
            target_reacheds,
        ) = data;

        let mut targets_leaf: HashMap<Entity, Entity> = HashMap::new();

        targeted_eats.clear(); //TODO dirty, create a system specificaly to clear this.

        //check if we managed to get a target
        for (entity, _cow, _pos) in (&entities, &cows, &mut positions).join() {
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

        //Chose the leaf to go
        for (cow_entity, viewshed, _cow) in (&entities, &viewsheds, &cows).join() {
            //search for every leaf in the viewshed
            let mut found_leaf: Vec<Entity> = Vec::new();
            for visible_tile in viewshed.visible_tiles.iter() {
                let idx = map.xy_idx(visible_tile.x, visible_tile.y);
                if let Some(tile_content) = map.tile_content.get(&idx) {
                    for maybe_leaf in tile_content.iter() {
                        if let Some(_leaf) = leafs.get(*maybe_leaf) {
                            found_leaf.push(*maybe_leaf);
                        }
                    }
                }
            }

            let mut choosen_leaf: Option<Entity> = None;
            let mut min: f32 = std::f32::MAX;
            let pos = positions.get(cow_entity).unwrap();
            for leaf in found_leaf {
                let leaf_pos = positions.get(leaf).unwrap();
                let maybe_targeted_eat = targeted_eats.get(leaf);

                //if their is a other creature that want the target, then I only go if I am closer
                let mut competitor_distance = std::f32::MAX;
                if let Some(targeted) = maybe_targeted_eat {
                    competitor_distance = targeted.distance;
                }
                let distance = rltk::DistanceAlg::Pythagoras
                    .distance2d(Point::new(pos.x, pos.y), Point::new(leaf_pos.x, leaf_pos.y));
                if (distance < min) && (distance < competitor_distance) {
                    choosen_leaf = Some(leaf);
                    min = distance;
                }
            }
            if let Some(leaf) = choosen_leaf {
                targeted_eats
                    .insert(
                        leaf,
                        TargetedForEat {
                            predator: cow_entity,
                            distance: min,
                            predator_pos: Point::new(pos.x, pos.y),
                        },
                    )
                    .expect("Unable ot insert");
                go_targets
                    .insert(
                        cow_entity,
                        GoOnTarget {
                            target: leaf,
                            scope: SearchScope::Small,
                        },
                    )
                    .expect("Unable to insert");
            }
        }

        targets_leaf.clear();
    }
}
