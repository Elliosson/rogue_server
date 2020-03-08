extern crate specs;
use crate::{gamelog::GameLog, BlocksTile, FacingDirection, Map, Position, ToDelete, WantDestroy};
use specs::prelude::*;

pub struct DestroySystem {}

impl<'a> System<'a> for DestroySystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantDestroy>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, BlocksTile>,
        WriteStorage<'a, FacingDirection>,
        WriteStorage<'a, ToDelete>,
        WriteExpect<'a, Map>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            _log,
            mut want_destroys,
            positions,
            block_tiles,
            facings,
            mut to_deletes,
            map,
        ) = data;

        for (_entity, _want_destroy, pos, facing) in
            (&entities, &want_destroys, &positions, &facings).join()
        {
            let idx = map.xy_idx(pos.x() + facing.front_tile.x, pos.y() + facing.front_tile.y);

            if let Some(content) = map.tile_content.get(&idx) {
                for cont_entity in content.iter() {
                    if let Some(_blocker) = block_tiles.get(*cont_entity) {
                        to_deletes
                            .insert(*cont_entity, ToDelete {})
                            .expect("Unable to insert");
                    };
                }
            };
        }
        want_destroys.clear();
    }
}
