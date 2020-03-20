extern crate specs;
use crate::{
    gamelog::GameLog, BuildingChoice, BuildingPlan, CloseInteration, CombatStats, Connected,
    InBackpack, InteractableObject, InventaireItem, Item, Map, Name, OnlinePlayer, PlayerInfo,
    PlayerLog, Position,
};
use specs::prelude::*;

//Collect player information that will be send thought network
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
        ReadStorage<'a, BuildingChoice>,
        ReadStorage<'a, CombatStats>,
        ReadStorage<'a, PlayerLog>,
        ReadExpect<'a, Map>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            _log,
            mut player_infos,
            backpacks,
            interactable_objects,
            _connecteds,
            online_players,
            items,
            names,
            positions,
            building_choices,
            combat_stats,
            player_logs,
            map,
        ) = data;

        //Todo check in player is connected and find a way to handle local player
        for (_entity, _online_player, pos, player_info, combat_stat) in (
            &entities,
            &online_players,
            &positions,
            &mut player_infos,
            &combat_stats,
        )
            .join()
        {
            player_info.inventaire.clear();
            player_info.close_interations.clear();
            player_info.possible_builds.clear();
            player_info.my_info.pos.x = pos.x();
            player_info.my_info.pos.y = pos.y();
            player_info.my_info.hp = combat_stat.hp;
            player_info.my_info.max_hp = combat_stat.max_hp;
        }

        for (_entity, _online_player, player_log, player_info) in
            (&entities, &online_players, &player_logs, &mut player_infos).join()
        {
            player_info.my_info.player_log = player_log.logs().clone();
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
        //Todo check in player is connected and find a way to handle local player
        for (_entity, _online_player, pos, player_info, name) in (
            &entities,
            &online_players,
            &positions,
            &mut player_infos,
            &names,
        )
            .join()
        {
            let mut close_entity: Vec<Entity> = Vec::new();
            //Get entity on position
            if let Some(entities_on_pos) = map.tile_content.get(&map.xy_idx(pos.x(), pos.y())) {
                close_entity.extend(entities_on_pos);
            }
            //We get all the entity in the adjacent tiles
            if let Some(entities_on_pos) = map.tile_content.get(&map.xy_idx(pos.x() + 1, pos.y())) {
                close_entity.extend(entities_on_pos);
            }
            if let Some(entities_on_pos) = map.tile_content.get(&map.xy_idx(pos.x() - 1, pos.y())) {
                close_entity.extend(entities_on_pos);
            }
            if let Some(entities_on_pos) = map.tile_content.get(&map.xy_idx(pos.x(), pos.y() + 1)) {
                close_entity.extend(entities_on_pos);
            }
            if let Some(entities_on_pos) = map.tile_content.get(&map.xy_idx(pos.x(), pos.y() - 1)) {
                close_entity.extend(entities_on_pos);
            }

            //interaction on position
            for on_pos_entity in close_entity.iter() {
                if let Some(interactable) = interactable_objects.get(*on_pos_entity) {
                    for intereraction in interactable.interactions.iter() {
                        player_info.close_interations.push(CloseInteration {
                            interaction_name: intereraction.name.clone(),
                            object_name: name.name.clone(),
                            index: on_pos_entity.id(),
                            generation: on_pos_entity.gen().id(),
                            entity: Some(*on_pos_entity),
                        })
                    }
                }
            }
        }
        //fill building plan
        //TODO harmonize name
        for (entity, building_choice) in (&entities, &building_choices).join() {
            if let Some(player_info) = player_infos.get_mut(entity) {
                for plan in building_choice.plans.iter() {
                    player_info
                        .possible_builds
                        .push(BuildingPlan { name: plan.clone() });
                }
            }
        }
    }
}
