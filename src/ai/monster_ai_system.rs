extern crate specs;
use crate::{InPlayerView, Map, Monster, OnlinePlayer, Viewshed, WantsToApproach};
use specs::prelude::*;
extern crate rltk;

//use std::time::{Duration, Instant};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, Map>,
        WriteStorage<'a, Monster>,
        WriteStorage<'a, OnlinePlayer>,
        WriteStorage<'a, InPlayerView>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, WantsToApproach>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            map,
            monsters,
            online_players,
            in_player_views,
            viewsheds,
            mut want_approachs,
        ) = data;

        //The in_view ensure there are in the viewshed of a player
        //So monster are only active if a player is nearby, this is done for reducing the load of the server
        for (entity, _monster, _in_view, viewshed) in
            (&entities, &monsters, &in_player_views, &viewsheds).join()
        {
            //we search for a target and the go for attacking him
            //TODO this is so ineficient that it's made me cry
            for point in viewshed.visible_tiles.iter() {
                let idx = map.xy_idx(point.x, point.y);
                if let Some(tile_content) = map.tile_content.get(&idx) {
                    for tile_entity in tile_content.iter() {
                        if let Some(_player) = online_players.get(*tile_entity) {
                            want_approachs
                                .insert(entity, WantsToApproach { idx: idx as i32 })
                                .expect("Unable to insert");
                        }
                    }
                }
            }
        }
    }
}
