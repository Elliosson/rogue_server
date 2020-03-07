extern crate specs;
use crate::components::*;
use crate::specs::saveload::{SimpleMarker, SimpleMarkerAllocator};
use specs::prelude::*;

use super::{rawmaster::*, RAWS};

pub type SpwanPropData<'a, 'b> = (
    &'b Entities<'a>,
    &'b mut WriteStorage<'a, Position>,
    &'b mut WriteStorage<'a, Renderable>,
    &'b mut WriteStorage<'a, Name>,
    &'b mut WriteStorage<'a, Item>,
    &'b mut WriteStorage<'a, Interactable>,
    &'b mut WriteStorage<'a, InteractableObject>,
    &'b mut WriteStorage<'a, Leaf>,
    &'b mut WriteStorage<'a, Tree>,
    &'b mut WriteStorage<'a, EnergyReserve>,
    &'b mut WriteStorage<'a, BlocksTile>,
    &'b mut WriteStorage<'a, Viewshed>,
    &'b mut WriteStorage<'a, Herbivore>,
    &'b mut WriteStorage<'a, Reproduction>,
    &'b mut WriteStorage<'a, WantsToDuplicate>,
    &'b mut WriteStorage<'a, SimpleMarker<SerializeMe>>,
    &'b mut WriteExpect<'a, SimpleMarkerAllocator<SerializeMe>>,
);

//key is just a string, it's just the name of the entity
//TODO WARNING, No unique id add here
//This spawner is proplably to abandon
pub fn spawn_named_prop_ingame(data: SpwanPropData, key: &str, pos: SpawnType) {
    let (
        entities,
        positions,
        renderables,
        names,
        _items,
        interactables,
        interactable_objects,
        leafs,
        trees,
        en_res,
        block_tiles,
        viewsheds,
        herbivores,
        reprods,
        _want_to_duplicate,
        simple_markers,
        simple_marker_allocators,
    ) = data;

    let raws: &RawMaster = &RAWS.lock().unwrap();
    if raws.prop_index.contains_key(key) {
        let prop_template = &raws.raws.props[raws.prop_index[key]];

        let mut eb = entities
            .build_entity()
            .marked(simple_markers, simple_marker_allocators);

        // Spawn in the specified location
        let mut dirty = Vec::new(); //TODO ADD THIS TO MAP
        match pos {
            SpawnType::AtPosition { x, y } => {
                eb = eb.with(Position::new(x, y, &mut dirty), positions);
            }
        }

        // Renderable
        if let Some(renderable) = &prop_template.renderable {
            let renderable_obj = Renderable {
                glyph: rltk::to_cp437(renderable.glyph.chars().next().unwrap()),
                fg: rltk::RGB::from_hex(&renderable.fg).expect("Invalid RGB"),
                bg: rltk::RGB::from_hex(&renderable.bg).expect("Invalid RGB"),
                render_order: renderable.order,
            };
            eb = eb.with(renderable_obj, renderables);
        }

        eb = eb.with(
            Name {
                name: prop_template.name.clone(),
            },
            names,
        );

        // Interactable
        if let Some(interactable) = prop_template.interactable {
            if interactable {
                eb = eb.with(Interactable {}, interactables)
            };
        }

        // InteractableObject
        if let Some(interactable_object) = &prop_template.interactable_object {
            eb = eb.with(interactable_object.clone(), interactable_objects); //TODO comprendre pourquoi il ne fait pas comme ça( il passe par un itermediaire item_component)
        }

        if let Some(leaf) = prop_template.leaf {
            if leaf == true {
                eb = eb.with(Leaf { nutriments: 100 }, leafs); //TODO no default value
            }
        }

        if let Some(tree) = prop_template.tree {
            if tree == true {
                eb = eb.with(Tree {}, trees); //TODO no default value
            }
        }

        // EnergyReserve
        if let Some(energy_reserve) = &prop_template.energy_reserve {
            eb = eb.with(
                EnergyReserve {
                    reserve: energy_reserve.reserve,
                    max_reserve: energy_reserve.max_reserve,
                    body_energy: energy_reserve.body_energy,
                    base_consumption: energy_reserve.base_consumption,
                    hunger: Hunger::Full,
                },
                en_res,
            );
        }

        if let Some(block_tile) = prop_template.blocks_tile {
            if block_tile == true {
                eb = eb.with(BlocksTile {}, block_tiles); //TODO no default value
            }
        }

        // Viewshed
        if let Some(viewshed) = &prop_template.viewshed {
            eb = eb.with(
                Viewshed {
                    visible_tiles: Vec::new(),
                    range: viewshed.range,
                    dirty: true,
                },
                viewsheds,
            );
        }

        // Herbivore
        if let Some(herbivore) = &prop_template.herbivore {
            eb = eb.with(herbivore.clone(), herbivores); //TODO no default value
        }

        // Reproduction
        if let Some(reproduction) = &prop_template.reproduction {
            eb = eb.with(reproduction.clone(), reprods);
        }

        eb.build();
    }
}

type ObjectSpawmerDataRef<'a, 'b> = (
    &'b Entities<'a>,
    &'b mut WriteStorage<'a, Position>,
    &'b mut WriteStorage<'a, Renderable>,
    &'b mut WriteStorage<'a, Name>,
    &'b mut WriteStorage<'a, Item>,
);

//almost duplicaton of rawmaster function , only diference is the insert because we don't have ecs
//TODO deplacer dans raw
//TODO Atention marker not added, opject will not be save
pub fn spawn_named_item_ingame(
    data: ObjectSpawmerDataRef,
    raws: &RawMaster,
    key: &str,
    pos: SpawnType,
) {
    let (entities, positions, renderables, names, items) = data;

    if raws.item_index.contains_key(key) {
        let item_template = &raws.raws.items[raws.item_index[key]];

        let new_entity = entities.create();
        let mut dirty = Vec::new();

        // Spawn in the specified location
        match pos {
            SpawnType::AtPosition { x, y } => {
                positions
                    .insert(new_entity, Position::new(x, y, &mut dirty))
                    .expect("Unable to inser position");
            }
        }

        // Renderable
        if let Some(renderable) = &item_template.renderable {
            renderables
                .insert(
                    new_entity,
                    Renderable {
                        glyph: rltk::to_cp437(renderable.glyph.chars().next().unwrap()),
                        fg: rltk::RGB::from_hex(&renderable.fg).expect("Invalid RGB"),
                        bg: rltk::RGB::from_hex(&renderable.bg).expect("Invalid RGB"),
                        render_order: renderable.order,
                    },
                )
                .expect("Unable to insert renderable");
        }

        names
            .insert(
                new_entity,
                Name {
                    name: item_template.name.clone(),
                },
            )
            .expect("Unable to insert name");

        items
            .insert(new_entity, Item {})
            .expect("Unable to insert item");
    } else {
        println!("Error: key: {} , is not know", key);
    }
}

/*

type SpwanMobData<'a, 'b> = (
    &'b Entities<'a>,
    &'b mut WriteStorage<'a, Position>,
    &'b mut WriteStorage<'a, Renderable>,
    &'b mut WriteStorage<'a, Name>,
    &'b mut WriteStorage<'a, Item>,
    &'b mut WriteStorage<'a, Interactable>,
    &'b mut WriteStorage<'a, InteractableObject>,
    &'b mut WriteStorage<'a, Leaf>,
    &'b mut WriteStorage<'a, Tree>,
);



pub fn _spawn_named_mob_ingame(
    data: SpwanMobData,
    raws: &RawMaster,
    key: &str,
    pos: SpawnType,
) {
    let (entities, positions, renderables, names, items, interactables, interactable_objects, leafs, trees) =
        data;
    if raws.mob_index.contains_key(key) {
        let mob_template = &raws.raws.mobs[raws.mob_index[key]];

        let mut eb = entities.build_entity();

        // Spawn in the specified location
        match pos {
            SpawnType::AtPosition { x, y } => {
                eb = eb.with(Position { x, y }, positions);
            }
        }

        // Renderable
        if let Some(renderable) = &prop_template.renderable {
            let renderable_obj = Renderable {
                glyph: rltk::to_cp437(renderable.glyph.chars().next().unwrap()),
                fg: rltk::RGB::from_hex(&renderable.fg).expect("Invalid RGB"),
                bg: rltk::RGB::from_hex(&renderable.bg).expect("Invalid RGB"),
                render_order: renderable.order,
            };
            eb = eb.with(renderable_obj, renderables);
        }

        eb = eb.with(
            Name {
                name: prop_template.name.clone(),
            },
            names,
        );

        eb = eb.with(Monster{});
        if mob_template.blocks_tile {
            eb = eb.with(BlocksTile{}, monsters);
        }
        eb = eb.with(CombatStats{
            max_hp : mob_template.stats.max_hp,
            hp : mob_template.stats.hp,
            power : mob_template.stats.power,
            defense : mob_template.stats.defense
        }, );
        eb = eb.with(Viewshed{ visible_tiles : Vec::new(), range: mob_template.vision_range, dirty: true });

        return Some(eb.build());
    }
    None
}
*/
