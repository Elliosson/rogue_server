extern crate specs;
use crate::{
    gamelog::GameLog, Blocking, DeathLoot, HaveRespawnPoint, ObjectBuilder, RespawnPoint, ToDelete,
    ToSpawnList, Unblocking, WantCraft,
};
use specs::prelude::*;

pub struct Interationv2System {}

impl<'a> System<'a> for Interationv2System {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, InteractionResquestListV2>,
        WriteExpect<'a, ObjectBuilder>,
        WriteStorage<'a, ToDelete>,
        WriteStorage<'a, Blocking>,
        WriteStorage<'a, Unblocking>,
        WriteStorage<'a, RespawnPoint>,
        WriteStorage<'a, HaveRespawnPoint>,
        WriteStorage<'a, WantCraft>,
        WriteStorage<'a, DeathLoot>,
        WriteExpect<'a, ToSpawnList>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            _log,
            mut interaction_request_list,
            mut object_builder,
            mut to_deletes,
            mut blockings,
            mut unblockings,
            mut respawn_points,
            mut have_respawn_points,
            mut want_crafts,
            death_loots,
            mut to_spawns,
        ) = data;

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
                    if let Some(death_loot) = death_loots.get(interation_request.interacted_entity)
                    {
                        for loot in &death_loot.loots {
                            object_builder.request(
                                interation_request.x,
                                interation_request.y,
                                loot.clone(),
                            );
                        }
                    }
                    to_deletes
                        .insert(interation_request.interacted_entity, ToDelete {})
                        .expect("Unable to insert delete entity");
                }
                "cut" => {
                    // todo will destroy everithing, I need to add a control to check if this comand is allowed for the user
                    if let Some(death_loot) = death_loots.get(interation_request.interacted_entity)
                    {
                        for loot in &death_loot.loots {
                            object_builder.request(
                                interation_request.x,
                                interation_request.y,
                                loot.clone(),
                            );
                        }
                    }
                    to_deletes
                        .insert(interation_request.interacted_entity, ToDelete {})
                        .expect("Unable to insert delete entity");
                }
                "mine_iron" => {
                    if let Some(death_loot) = death_loots.get(interation_request.interacted_entity)
                    {
                        for loot in &death_loot.loots {
                            object_builder.request(
                                interation_request.x,
                                interation_request.y,
                                loot.clone(),
                            );
                        }
                    }
                    to_deletes
                        .insert(interation_request.interacted_entity, ToDelete {})
                        .expect("Unable to insert delete entity");
                }
                "open_door" => {
                    unblockings
                        .insert(interation_request.interacted_entity, Unblocking {})
                        .expect("Unable to insert delete entity");
                }
                "close_door" => {
                    blockings
                        .insert(interation_request.interacted_entity, Blocking {})
                        .expect("Unable to insert delete entity");
                }
                "respawn_here" => {
                    respawn_points
                        .insert(
                            interation_request.interacted_entity,
                            RespawnPoint {
                                owner: interation_request.requester_entity,
                            },
                        )
                        .expect("Unable to insert delete entity");
                    have_respawn_points
                        .insert(
                            interation_request.requester_entity,
                            HaveRespawnPoint {
                                respawn_point: interation_request.interacted_entity,
                            },
                        )
                        .expect("Unable to insert delete entity");
                }
                "plant_carrot" => to_spawns.request(
                    interation_request.x,
                    interation_request.y,
                    "CarrotPlant".to_string(),
                ),
                "wooden_spear" => {
                    //todo I will need a specific commant for craft it will be to heavy to have it in interaction
                    want_crafts
                        .insert(
                            interation_request.requester_entity,
                            WantCraft {
                                name: "WoodenSpear".to_string(),
                            },
                        )
                        .expect("Unable to insert");
                }
                "wall_block" => {
                    //todo I will need a specific commant for craft it will be to heavy to have it in interaction
                    want_crafts
                        .insert(
                            interation_request.requester_entity,
                            WantCraft {
                                name: "WallBlock".to_string(),
                            },
                        )
                        .expect("Unable to insert");
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
