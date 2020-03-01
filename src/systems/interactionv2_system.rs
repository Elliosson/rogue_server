extern crate specs;
use crate::{gamelog::GameLog, ObjectBuilder, ToDelete};
use specs::prelude::*;

pub struct Interationv2System {}

impl<'a> System<'a> for Interationv2System {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, InteractionResquestListV2>,
        WriteExpect<'a, ObjectBuilder>,
        WriteStorage<'a, ToDelete>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (_log, mut interaction_request_list, mut object_builder, mut to_deletes) = data;

        for interation_request in &interaction_request_list.requests {
            match interation_request.interaction.as_str() {
                "pick_apple" => {
                    object_builder.request(
                        interation_request.x,
                        interation_request.y,
                        "Apple".to_string(),
                    );
                }
                "chop_tree" => {
                    object_builder.request(
                        interation_request.x,
                        interation_request.y,
                        "Wood".to_string(),
                    );
                    to_deletes
                        .insert(interation_request.interacted_entity, ToDelete {})
                        .expect("Unable to insert delete entity");
                }
                _ => {}
            }
        }
        interaction_request_list.requests.clear();
    }
}

pub struct InteractionResquestV2 {
    x: i32,
    y: i32,
    interaction: String,
    interacted_entity: Entity,
    requester_entity: Entity,
}

pub struct InteractionResquestListV2 {
    requests: Vec<InteractionResquestV2>,
}

impl InteractionResquestListV2 {
    #[allow(clippy::new_without_default)]
    pub fn new() -> InteractionResquestListV2 {
        InteractionResquestListV2 {
            requests: Vec::new(),
        }
    }

    pub fn request(
        &mut self,
        x: i32,
        y: i32,
        interaction: String,
        interacted_entity: Entity,
        requester_entity: Entity,
    ) {
        self.requests.push(InteractionResquestV2 {
            x,
            y,
            interaction,
            interacted_entity,
            requester_entity,
        });
    }
}
