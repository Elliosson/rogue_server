extern crate specs;
use crate::{
    algo, BlocksTile, EntityMoved, GoOnTarget, Map, Position, RunState, SearchScope, Speed,
    TargetReached, Viewshed, MOVE_COST,
};
use specs::prelude::*;
//use std::time::Instant;

pub struct GoTargetSystem {}

impl<'a> System<'a> for GoTargetSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
        WriteStorage<'a, EntityMoved>,
        WriteStorage<'a, Viewshed>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, RunState>,
        WriteStorage<'a, GoOnTarget>,
        WriteStorage<'a, Speed>,
        WriteStorage<'a, TargetReached>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            mut positions,
            blockers,
            entities,
            mut moved,
            mut viewsheds,
            _player_entity,
            mut _runstate,
            mut go_targets,
            mut speeds,
            mut target_reachs,
        ) = data;

        target_reachs.clear();

        //let _now = Instant::now();

        for (entity, go_target, speed) in (&entities, &go_targets, &mut speeds).join() {
            if speed.move_point >= MOVE_COST {
                //let _now2 = Instant::now();
                let max_step;
                match go_target.scope {
                    SearchScope::Small => max_step = 200,
                    SearchScope::Big => max_step = 1000,
                }
                let mut path;
                {
                    let pos = positions.get(entity).expect("No postion");
                    let target_pos = positions.get(go_target.target).expect("No postion");

                    //Inblock the destination, if not he will never find a way to go on the target
                    //let mut path_map = map.clone();
                    let dest_idx = map.xy_idx(target_pos.x(), target_pos.y());
                    //path_map.blocked[dest_idx] = false;
                    //I can't copy the map because it's too heavy. This is a quick fix to unbloc the destination
                    //TODO I must create a astar that permit to go on a blocked destination
                    let temp_map_blocked = map.is_blocked(dest_idx);
                    map.set_blocked(dest_idx, false);

                    //println!("first part time = {}", now2.elapsed().as_micros());

                    //let _now3 = Instant::now();

                    path = algo::a_star_search(
                        map.xy_idx(pos.x(), pos.y()) as i32,
                        map.xy_idx(target_pos.x(), target_pos.y()) as i32, //TODO change that, the "-1" is a dirty fix for the imposibility to go on a blicked tile
                        &mut *map,
                        max_step, //Max step for search, TODO thonk of a way to automatically find an acceptable number
                    );
                    map.set_blocked(dest_idx, temp_map_blocked); //TODO remove it's ugly
                                                                 //println!("a* search time = {}", now3.elapsed().as_micros());
                }

                //println!("2 part time = {}", now2.elapsed().as_micros());

                //Move for Real
                //TODO I need to resolve herbivore movement before carnivor movement, but even so it's not perfect
                if path.success {
                    //I am not sure the pathfind can find a way if the target is blocking entity, aparently, no
                    let pos = positions.get_mut(entity).expect("No postion");
                    if path.steps.len() > 1 {
                        path.steps.remove(0); //we remove the initial position

                        for (vec_idx, dest_idx) in path.steps.iter().enumerate() {
                            //we are in the last iteration
                            if vec_idx >= path.steps.len() - 1 {
                                //we are in contact //TODO find a way to clean target_reachs
                                /*
                                target_reachs
                                    .insert(
                                        entity,
                                        TargetReached {
                                            target: go_target.target,
                                        },
                                    )
                                    .expect("unable to insert");
                                    */
                                break;
                            }
                            //100 move point per tilde for now
                            if speed.move_point >= MOVE_COST {
                                //aply move
                                let start_idx = map.xy_idx(pos.x(), pos.y());
                                let is_blocking = blockers.get(entity);
                                if is_blocking.is_some() {
                                    map.set_blocked(start_idx, false);
                                    map.set_blocked(*dest_idx as usize, true);
                                }
                                pos.moving(
                                    dest_idx % map.width,
                                    dest_idx / map.width,
                                    &mut map.dirty,
                                );

                                if let Some(vs) = viewsheds.get_mut(entity) {
                                    vs.dirty = true;
                                }
                                moved
                                    .insert(entity, EntityMoved {})
                                    .expect("Unable to insert"); //this should not be usefull anymore
                                speed.move_point -= MOVE_COST;
                            } else {
                                break; //not enought move point
                            }
                        }
                    } else {
                        //we are in contact
                        target_reachs
                            .insert(
                                entity,
                                TargetReached {
                                    target: go_target.target,
                                },
                            )
                            .expect("unable to insert");
                    }
                } else {
                    //println!("path failed");
                }
                //println!("a* iter time = {}", now2.elapsed().as_micros());
            }
        }
        //println!("a* time = {}", now.elapsed().as_micros());

        go_targets.clear();
    }
}
