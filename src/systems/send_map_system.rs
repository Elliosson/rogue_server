extern crate specs;
use crate::{Connected, Map, OnlinePlayer, Position, Renderable, Viewshed};
use specs::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct SendMapSystem {}

impl<'a> System<'a> for SendMapSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, OnlinePlayer>,
        WriteStorage<'a, Connected>,
        ReadExpect<'a, Map>,
        WriteExpect<'a, Arc<Mutex<HashMap<String, Vec<(u32, i32, Position, Renderable)>>>>>,
        WriteStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, viewsheds, online_players, connecteds, map, map_to_send, renderables) = data;

        let mut dirty = Vec::new();
        let mut map_send_guard = map_to_send.lock().unwrap();
        map_send_guard.clear();

        for (my_entity, viewshed, _online_player, connected) in
            (&entities, &viewsheds, &online_players, &connecteds).join()
        {
            let my_uiid = connected.uuid.clone();
            let mut my_viewed_map = Vec::new();

            for vis in viewshed.visible_tiles.iter() {
                let idx = map.xy_idx(vis.x, vis.y);

                if let Some(tile_content) = map.tile_content.get(&idx) {
                    for entity in tile_content.iter() {
                        //filter out my character to avoid desinc with the camera
                        if *entity != my_entity {
                            if let Some(renderable) = renderables.get(*entity) {
                                my_viewed_map.push((
                                    entity.id(),
                                    entity.gen().id(),
                                    Position::new(vis.x, vis.y, &mut dirty),
                                    renderable.clone(),
                                )); //TODO to remplace by point
                            }
                        }
                    }
                }
            }

            map_send_guard.insert(my_uiid, my_viewed_map);
        }
    }
}
