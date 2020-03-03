extern crate specs;
use crate::{
    gamelog::GameLog, CommandToConvert, EntityToConvert, Name, PlayerInfo, PlayerInput,
    PlayerInputComp,
};
use specs::prelude::*;

//Due to serialization probleme I can't send vec of entity thought the network.building_system.building_system
//So I send id and gen and here I match the id/gen with the enity, if the said entity is still in the corresponding collection

//maybe doing a system for this is to much, but for now it's still a love story with ECS, so let's go !
pub struct IdEntityInterfaceMatching {}

impl<'a> System<'a> for IdEntityInterfaceMatching {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, PlayerInfo>,
        WriteStorage<'a, EntityToConvert>,
        WriteStorage<'a, PlayerInputComp>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, names, player_infos, mut to_converts, mut player_inputs_comps) = data;

        for (entity, player_info, to_convert) in (&entities, &player_infos, &to_converts).join() {
            match to_convert.command.clone() {
                CommandToConvert::INTERACT(x, y, name, id, gen) => {
                    let mut interacted_entity = None;
                    for interaction in player_info.close_interations.iter() {
                        if id == interaction.index && gen == interaction.generation {
                            interacted_entity = Some(interaction.entity);
                            break;
                        }
                    }
                    if let Some(inte_entity) = interacted_entity {
                        player_inputs_comps
                            .insert(
                                entity,
                                PlayerInputComp {
                                    input: PlayerInput::INTERACT(x, y, name, inte_entity.unwrap()),
                                },
                            )
                            .expect("Unable to insert");
                    } else {
                        println!("unable to find the wanted interacted entity");
                    }
                }
                _ => {}
            }
        }

        to_converts.clear();
    }
}
