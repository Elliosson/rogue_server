extern crate specs;
use crate::{gamelog::GameLog, InPlayerView, Map, Monster, OnlinePlayer, Viewshed};
use specs::prelude::*;
use std::collections::HashSet;

pub struct InPlayerViewSystem {}

impl<'a> System<'a> for InPlayerViewSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, OnlinePlayer>,
        ReadStorage<'a, Viewshed>,
        WriteStorage<'a, InPlayerView>,
        WriteStorage<'a, Monster>,
        WriteExpect<'a, Map>,
    );

    //For now index all the monster in the viewshed of a player
    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, online_players, viewsheds, mut in_views, monsters, map) = data;

        let mut viewed_entities = HashSet::new();

        in_views.clear();

        for (_entity, _online_players, viewshed) in (&entities, &online_players, &viewsheds).join()
        {
            for tile in viewshed.visible_tiles.iter() {
                if let Some(tile_entities) = map.tile_content.get(&map.xy_idx(tile.x, tile.y)) {
                    for tile_entity in tile_entities.iter() {
                        if let Some(_monster) = monsters.get(*tile_entity) {
                            viewed_entities.insert(*tile_entity);
                        }
                    }
                }
            }
        }

        for entity in viewed_entities.iter() 
            in_views
                .insert(*entity, InPlayerView {})
                .expect("Unable to insert");
        }
    }
}
