extern crate specs;
use crate::{
    Animal, Carnivore, CombatStats, EnergyReserve, FoodPreference, FoodType, GoOnTarget, Herbivore,
    Leaf, Map, Meat, MyChoosenFood, MyTurn, Point, Position, RunState, SearchScope, Specie, Speed,
    TargetedForEat, Viewshed, WantsToFlee,
};
use specs::prelude::*;
extern crate rltk;

//use std::time::{Duration, Instant};

pub struct TargetingAI {}

impl<'a> System<'a> for TargetingAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, RunState>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Herbivore>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Leaf>,
        WriteStorage<'a, TargetedForEat>,
        WriteStorage<'a, GoOnTarget>,
        WriteStorage<'a, Specie>,
        WriteStorage<'a, Animal>,
        WriteStorage<'a, Carnivore>,
        WriteStorage<'a, WantsToFlee>,
        WriteStorage<'a, EnergyReserve>,
        WriteStorage<'a, MyTurn>,
        WriteStorage<'a, MyChoosenFood>,
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, Speed>,
        WriteStorage<'a, Meat>,
        WriteStorage<'a, FoodPreference>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            map,
            _runstate,
            entities,
            viewsheds,
            herbivores,
            mut positions,
            leafs,
            mut targeted_eats,
            mut go_targets,
            species,
            animals,
            carnivores,
            mut flees,
            energy_reserves,
            mut turns,
            mut my_choosen_foods,
            mut combat_stats,
            speeds,
            meats,
            food_prefs,
        ) = data;

        targeted_eats.clear(); //TODO dirty, create a system specificaly to clear this.

        let mut turn_done: Vec<Entity> = Vec::new();

        //Chose the food to go
        //first try to have his favorite food
        for (entity, viewshed, _animal, _carnivore, _herbivore, energy_reserve, _turn, food_pref) in
            (
                &entities,
                &viewsheds,
                &animals,
                &carnivores,
                &herbivores,
                &energy_reserves,
                &turns,
                &food_prefs,
            )
                .join()
        {
            //search for every possible food in the viewshed, and store them
            let mut viewed_food: Vec<Entity> = Vec::new();

            for visible_tile in viewshed.visible_tiles.iter() {
                let idx = map.xy_idx(visible_tile.x, visible_tile.y);
                if let Some(tile_content) = map.tile_content.get(&idx) {
                    for maybe_food in tile_content.iter() {
                        viewed_food.push(*maybe_food);
                    }
                }
            }

            //search all type of food from the favorite to the less favorite
            for (seuil, food_type) in food_pref.choices.iter().rev() {
                if energy_reserve.reserve < *seuil as f32 {
                    let found_foods = match food_type {
                        FoodType::Meat => {
                            let fil = filter_component_distance(
                                &viewed_food,
                                entity,
                                &mut positions,
                                &meats,
                            );
                            fil
                            //&meats
                        }
                        FoodType::Animal => {
                            let filtered = filter_component_distance(
                                &viewed_food,
                                entity,
                                &mut positions,
                                &animals,
                            );
                            let filtered = not_my_specie(filtered, entity, &species);
                            filtered

                            //not_my_specie(filtered, entity, &species)
                            //&animals
                        }
                        FoodType::Vegetable => {
                            let fil = filter_component_distance(
                                &viewed_food,
                                entity,
                                &mut positions,
                                &leafs,
                            );
                            fil
                        }
                    };
                    //search to go on this type of food
                    //and actually say to go
                    if choose_food(
                        found_foods,
                        entity,
                        &mut positions,
                        &mut targeted_eats,
                        &mut go_targets,
                        &mut my_choosen_foods,
                        &mut combat_stats,
                        &speeds,
                        &energy_reserves,
                        &animals,
                    ) {
                        //if find food, end turn
                        turn_done.push(entity);
                        break;
                    }
                }
            }
        }

        //check someone want to eat us
        //TODO make a better flee with real move using speed and go to.
        for (entity, _animal, _pos, _carnivore, _herbivore) in (
            &entities,
            &animals,
            &mut positions,
            &carnivores,
            &herbivores,
        )
            .join()
        {
            if let Some(targeted) = targeted_eats.get(entity) {
                //For now just flee if someone want to eat us
                let mut flee_list = Vec::new();
                flee_list.push(map.xy_idx(targeted.predator_pos.x, targeted.predator_pos.y) as i32);

                flees
                    .insert(entity, WantsToFlee { indices: flee_list })
                    .expect("Unable to insert");

                //if flee, end turn
                turn_done.push(entity);
            }
        }

        // Remove turn marker for those that are done
        for done in turn_done.iter() {
            turns.remove(*done);
        }
        turn_done.clear();

        // Remove turn marker for those that are done
        for done in turn_done.iter() {
            turns.remove(*done);
        }
        turn_done.clear();
    }
}
//il the list return all the entity with the good component sorted by distance
pub fn filter_component_distance<'a, W: Component>(
    entity_list: &Vec<Entity>,
    entity: Entity,
    positions: &mut WriteStorage<'a, Position>,
    searched_component: &WriteStorage<'a, W>,
) -> Vec<(f32, Entity)> {
    let pos = positions.get(entity).unwrap();

    let mut filtered_entities = Vec::new();

    for comp_entity in entity_list.iter() {
        if let Some(_component) = searched_component.get(*comp_entity) {
            if let Some(com_pos) = positions.get(*comp_entity) {
                let distance = rltk::DistanceAlg::Pythagoras
                    .distance2d(Point::new(pos.x, pos.y), Point::new(com_pos.x, com_pos.y));

                filtered_entities.push((distance, *comp_entity));
            }
        }
    }
    //sort
    filtered_entities.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    filtered_entities
}
fn not_my_specie<'a>(
    entity_list: Vec<(f32, Entity)>,
    entity: Entity,
    species: &WriteStorage<'a, Specie>,
) -> Vec<(f32, Entity)> {
    let mut ret = Vec::new();
    let my_specie = species.get(entity).unwrap();
    for (_dist, member) in entity_list {
        if let Some(specie) = species.get(member) {
            if specie.name == my_specie.name {
                //exclude of the list
            } else {
                ret.push((_dist, member));
            }
        } else {
            ret.push((_dist, member));
        }
    }
    ret
}

//In a list of possible food, choose the closest that is not taken by someone closer to the food
//return true if a food have been choosen
fn choose_food<'a>(
    //sorted list of food and distance
    found_foods: Vec<(f32, Entity)>,
    entity: Entity,
    positions: &mut WriteStorage<'a, Position>,
    targeted_eats: &mut WriteStorage<'a, TargetedForEat>,
    go_targets: &mut WriteStorage<'a, GoOnTarget>,
    my_choosen_foods: &mut WriteStorage<'a, MyChoosenFood>,
    combat_stats: &mut WriteStorage<'a, CombatStats>,
    speeds: &WriteStorage<'a, Speed>,
    energies: &WriteStorage<'a, EnergyReserve>,
    animals: &WriteStorage<'a, Animal>,
) -> bool {
    let mut ret = false;
    let mut choosen_food: Option<Entity> = None;
    let mut min: f32 = std::f32::MAX;
    let pos = positions.get(entity).unwrap();

    for (distance, food) in found_foods.iter() {
        let maybe_targeted_eat = targeted_eats.get(*food);

        //if their is a other creature that want the target, then I only go if I am closer
        let mut competitor_distance = std::f32::MAX;
        let mut choose_made = false;
        if let Some(targeted) = maybe_targeted_eat {
            competitor_distance = targeted.distance;
        }

        if *distance < competitor_distance {
            //If food can fight, only go if I am stronger
            if let Some(_animal) = animals.get(*food) {
                if am_i_stronger(&combat_stats, entity, *food)
                    && hunt_gain(&speeds, &energies, &positions, entity, *food)
                {
                    choose_made = true;
                }
            } else {
                choose_made = true;
            }
        }

        if choose_made {
            choosen_food = Some(*food);
            min = *distance;
            //we found the food
            break;
        }
    }
    if let Some(food) = choosen_food {
        targeted_eats
            .insert(
                food,
                TargetedForEat {
                    //TODO add a flee if we are target to eat
                    predator: entity,
                    distance: min,
                    predator_pos: Point::new(pos.x, pos.y),
                },
            )
            .expect("Unable ot insert");
        go_targets
            .insert(
                entity,
                GoOnTarget {
                    target: food,
                    scope: SearchScope::Small,
                },
            )
            .expect("Unable to insert");
        my_choosen_foods
            .insert(entity, MyChoosenFood { target: food })
            .expect("Unable to insert");
        ret = true;
    }
    return ret;
}

//check combat stat to se if I am stronger
//Also return true If the enemy doesn't have combat stat
pub fn am_i_stronger<'a>(
    combat_stats: &WriteStorage<'a, CombatStats>,
    me: Entity,
    enemy: Entity,
) -> bool {
    let my_stat = combat_stats.get(me).unwrap();
    let ret;

    if let Some(enemy_stat) = combat_stats.get(enemy) {
        if my_stat.power > enemy_stat.power {
            ret = true;
        } else {
            ret = false;
        }
    } else {
        ret = true
    }
    ret
}

//check combat stat to se if I am stronger
//Also return true If the enemy doesn't have combat stat
pub fn _am_i_faster<'a>(speeds: &WriteStorage<'a, Speed>, me: Entity, enemy: Entity) -> bool {
    let my_speed = speeds.get(me).unwrap();
    let ret;

    if let Some(enemy_speed) = speeds.get(enemy) {
        let relative_speed = my_speed.point_per_turn as f32 / enemy_speed.point_per_turn as f32;
        if relative_speed > 1.0 {
            ret = true;
        } else {
            ret = false;
        }
    } else {
        ret = true
    }
    ret
}

//TODO not very accurate because not take in count digestion
// TODO also not take in coutn that the body_energy is not consume in the hunt
pub fn hunt_gain<'a>(
    speeds: &WriteStorage<'a, Speed>,
    energies: &WriteStorage<'a, EnergyReserve>,
    positions: &WriteStorage<'a, Position>,
    me: Entity,
    enemy: Entity,
) -> bool {
    let my_speed = speeds.get(me).unwrap();
    let my_energy = energies.get(me).unwrap();
    let enemy_speed = speeds.get(enemy).unwrap();
    let enemy_energy = energies.get(enemy).unwrap();
    let my_pos = positions.get(me).unwrap();
    let enemy_pos = positions.get(enemy).unwrap();
    let must_hunt;

    if my_speed.speed() > enemy_speed.speed() {
        let distance = rltk::DistanceAlg::Pythagoras.distance2d(
            Point::new(my_pos.x, my_pos.y),
            Point::new(enemy_pos.x, enemy_pos.y),
        );

        let chase_time: f32 = distance / (my_speed.speed() - enemy_speed.speed());
        let energy_cost: f32 = chase_time * my_energy.base_consumption;
        let energy_gain: f32 = f32::max(
            0.0,
            enemy_energy.reserve - chase_time * enemy_energy.base_consumption,
        ) + enemy_energy.body_energy;
        let hunt_benefice = energy_gain - energy_cost;
        if hunt_benefice > 0.0 {
            must_hunt = true;
        } else {
            must_hunt = false;
        }
    } else {
        must_hunt = false;
    }
    must_hunt
}
