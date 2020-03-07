extern crate specs;
use crate::raws::*;
use crate::specs::saveload::{SimpleMarker, SimpleMarkerAllocator};
use crate::{components::*, EnergyReserve, Name, Reproduction};
use specs::prelude::*;

pub struct PropSpawnerSystem {}

impl<'a> System<'a> for PropSpawnerSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, Name>,
        WriteStorage<'a, Item>,
        WriteStorage<'a, Interactable>,
        WriteStorage<'a, InteractableObject>,
        WriteStorage<'a, Leaf>,
        WriteStorage<'a, Tree>,
        WriteStorage<'a, EnergyReserve>,
        WriteStorage<'a, BlocksTile>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Herbivore>,
        WriteStorage<'a, Reproduction>,
        WriteStorage<'a, WantsToDuplicate>,
        WriteStorage<'a, SimpleMarker<SerializeMe>>,
        WriteExpect<'a, SimpleMarkerAllocator<SerializeMe>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut entities,
            mut positions,
            mut renderables,
            mut names,
            mut items,
            mut interactables,
            mut interactable_objects,
            mut leafs,
            mut trees,
            mut en_res,
            mut block_tiles,
            mut viewsheds,
            mut herbivores,
            mut reprods,
            mut want_to_duplicates,
            mut simple_markers,
            mut simple_marker_allocators,
        ) = data;

        let mut to_spawn: Vec<(String, i32, i32)> = Vec::new();

        {
            for (_entity, _want_to_duplicate, name, pos) in
                (&entities, &want_to_duplicates, &mut names, &positions).join()
            {
                to_spawn.push((name.name.clone(), pos.x(), pos.y()));
            }
        }

        for spawn in to_spawn.iter() {
            spawn_named_prop_ingame(
                //TODO rework this shit
                (
                    &mut entities,
                    &mut positions,
                    &mut renderables,
                    &mut names,
                    &mut items,
                    &mut interactables,
                    &mut interactable_objects,
                    &mut leafs,
                    &mut trees,
                    &mut en_res,
                    &mut block_tiles,
                    &mut viewsheds,
                    &mut herbivores,
                    &mut reprods,
                    &mut want_to_duplicates,
                    &mut simple_markers,
                    &mut simple_marker_allocators,
                ),
                &spawn.0,
                SpawnType::AtPosition {
                    x: spawn.1,
                    y: spawn.2,
                },
            );
        }

        want_to_duplicates.clear();
    }
}
