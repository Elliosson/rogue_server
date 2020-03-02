extern crate specs;
use crate::{
    gamelog::GameLog, CloseInteration, Connected, InBackpack, InteractableObject, InventaireItem,
    Item, Map, Name, OnlinePlayer, PlayerInfo, Position,
};
use specs::prelude::*;

pub struct PlayerInfoSystem {}

impl<'a> System<'a> for PlayerInfoSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, PlayerInfo>,
        ReadStorage<'a, InBackpack>,
        ReadStorage<'a, InteractableObject>,
        ReadStorage<'a, Connected>,
        ReadStorage<'a, OnlinePlayer>,
        ReadStorage<'a, Item>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Position>,
        ReadExpect<'a, Map>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            _log,
            mut player_infos,
            backpacks,
            interactable_objects,
            connecteds,
            online_players,
            items,
            names,
            positions,
            map,
        ) = data;

        //insert  a new player info in every conected player
        //effectively clean the thing
        player_infos.clear();
        for (entity, _connected, _online_player) in (&entities, &connecteds, &online_players).join()
        {
            player_infos
                .insert(
                    entity,
                    PlayerInfo {
                        inventaire: Vec::new(),
                        close_interations: Vec::new(),
                    },
                )
                .expect("Unable to insert");
        }

        //TODO these function are hightly ineficiant, to refactor if needed
        //fill inventory
        for (entity, backpack, _item, name) in (&entities, &backpacks, &items, &names).join() {
            if let Some(player_info) = player_infos.get_mut(backpack.owner) {
                player_info.inventaire.push(InventaireItem {
                    name: name.name.clone(),
                    index: entity.id(),
                    generation: entity.gen().id(),

                    entity: Some(entity),
                })
            }
        }

        //fill player interactions
        for (_entity, _connected, _online_player, pos, player_info) in (
            &entities,
            &connecteds,
            &online_players,
            &positions,
            &mut player_infos,
        )
            .join()
        {
            let entities_on_pos = &map.tile_content[map.xy_idx(pos.x, pos.y)];

            for on_pos_entity in entities_on_pos.iter() {
                if let Some(interactable) = interactable_objects.get(*on_pos_entity) {
                    for intereraction in interactable.interactions.iter() {
                        player_info.close_interations.push(CloseInteration {
                            interaction_name: intereraction.name.clone(),
                            index: on_pos_entity.id(),
                            generation: on_pos_entity.gen().id(),
                            entity: Some(*on_pos_entity),
                        })
                    }
                }
            }
        }
    }
}
