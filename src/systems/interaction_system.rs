extern crate specs;
use crate::{Interaction, ObjectBuilder, ToDelete};

use specs::prelude::*;

pub struct InteractionSystem {}

//for now just destruct the interacted
impl<'a> System<'a> for InteractionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, ObjectBuilder>,
        WriteExpect<'a, InteractionResquest>,
        WriteStorage<'a, ToDelete>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (_entities, mut object_builder, mut interaction_request, mut to_deletes) = data;

        //parcours all interaction request
        /*for (x, y, interaction, interacted_entity) in &interaction_request.requests {
            //build object
            for to_build in &interaction.object_to_build {
                //ask for building the object
                object_builder.request(*x, *y, to_build.clone());
            }

            //eventualy destroy the entiety
            if interaction.destructif == true {
                to_deletes
                    .insert(*interacted_entity, ToDelete {})
                    .expect("Unable to insert delete entity");
            }
        }*/

        interaction_request.requests.clear();
    }
}

pub struct InteractionResquest {
    requests: Vec<(i32, i32, Interaction, Entity)>,
}

impl InteractionResquest {
    #[allow(clippy::new_without_default)]
    pub fn new() -> InteractionResquest {
        InteractionResquest {
            requests: Vec::new(),
        }
    }

    pub fn request(&mut self, x: i32, y: i32, interaction: Interaction, interacted_entity: Entity) {
        self.requests.push((x, y, interaction, interacted_entity));
    }
}
