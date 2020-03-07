extern crate specs;
use crate::{
    Animal, Carnivore, Dead, DeathCause, Herbivore, MyChoosenFood, MyTurn, Position, RunState,
    WantToEat,
};
use specs::prelude::*;
extern crate rltk;

//use std::time::{Duration, Instant};

pub struct EatingKillingAI {}

impl<'a> System<'a> for EatingKillingAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, RunState>,
        Entities<'a>,
        WriteStorage<'a, Herbivore>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, WantToEat>,
        WriteStorage<'a, Animal>,
        WriteStorage<'a, Carnivore>,
        WriteStorage<'a, MyTurn>,
        WriteStorage<'a, MyChoosenFood>,
        WriteStorage<'a, Dead>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            _runstate,
            entities,
            herbivores,
            positions,
            mut want_to_eats,
            animals,
            carnivores,
            mut turns,
            mut my_choosen_foods,
            mut deads,
        ) = data;

        let mut turn_done: Vec<Entity> = Vec::new();

        //check if we managed to get on our choosen food
        for (entity, _animal, pos, _carnivore, _herbivore, _turn, choosen_food) in (
            &entities,
            &animals,
            &positions,
            &carnivores,
            &herbivores,
            &turns,
            &my_choosen_foods,
        )
            .join()
        {
            //Since this stay up at the destruction of entity The entity ccan be destroyed an we need to check
            if let Some(food_pos) = positions.get(choosen_food.target) {
                //TODO for now it eat directly I must add a fight
                if in_contact(pos, food_pos) {
                    //if the choosen food is an animal, kill it
                    //TODO at combat system, for now kill directly
                    if let Some(_animal) = animals.get(choosen_food.target) {
                        deads
                            .insert(
                                choosen_food.target,
                                Dead {
                                    cause: DeathCause::Killed { killer: entity },
                                },
                            )
                            .expect("Unable to insert!");
                    } else {
                        want_to_eats
                            .insert(
                                entity,
                                WantToEat {
                                    target: choosen_food.target,
                                },
                            )
                            .expect("Unable to insert");
                    }

                    //if eat, end turn
                    turn_done.push(entity);
                }
            }
        }
        my_choosen_foods.clear();

        // Remove turn marker for those that are done
        for done in turn_done.iter() {
            turns.remove(*done);
        }
    }
}

pub fn in_contact(pos1: &Position, pos2: &Position) -> bool {
    let mut ret = false;
    if pos1.x() >= pos2.x() - 1 && pos1.x() <= pos2.x() + 1 {
        if pos1.y() >= pos2.y() - 1 && pos1.y() <= pos2.y() + 1 {
            ret = true;
        }
    }
    ret
}
