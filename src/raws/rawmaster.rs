extern crate specs;

use super::{Raws, SexeChoice};
use crate::birth::{BirthForm, Mutations};
use crate::components::*;
use crate::random_table::RandomTable;
use specs::prelude::*;
use specs::saveload::{MarkedBuilder, Marker, MarkerAllocator};
use std::collections::{HashMap, HashSet}; //TODO se if we can suppress

pub enum SpawnType {
    AtPosition { x: i32, y: i32 },
}

pub struct RawMaster {
    pub raws: Raws,
    pub item_index: HashMap<String, usize>, //todo revert pub
    pub mob_index: HashMap<String, usize>,
    pub prop_index: HashMap<String, usize>,
}

impl RawMaster {
    pub fn empty() -> RawMaster {
        RawMaster {
            raws: Raws {
                items: Vec::new(),
                mobs: Vec::new(),
                props: Vec::new(),
                spawn_table: Vec::new(),
            },
            item_index: HashMap::new(),
            mob_index: HashMap::new(),
            prop_index: HashMap::new(),
        }
    }

    pub fn load(&mut self, raws: Raws) {
        self.raws = raws;
        self.item_index = HashMap::new();
        let mut used_names: HashSet<String> = HashSet::new();
        for (i, item) in self.raws.items.iter().enumerate() {
            if used_names.contains(&item.name) {
                println!("WARNING -  duplicate item name in raws [{}]", item.name);
            }
            if self.item_index.insert(item.name.clone(), i).is_some() {}
            used_names.insert(item.name.clone());
        }
        for (i, mob) in self.raws.mobs.iter().enumerate() {
            if used_names.contains(&mob.name) {
                println!("WARNING -  duplicate mob name in raws [{}]", mob.name);
            }
            self.mob_index.insert(mob.name.clone(), i);
            used_names.insert(mob.name.clone());
        }
        for (i, prop) in self.raws.props.iter().enumerate() {
            if used_names.contains(&prop.name) {
                println!("WARNING -  duplicate prop name in raws [{}]", prop.name);
            }
            self.prop_index.insert(prop.name.clone(), i);

            used_names.insert(prop.name.clone());
        }

        for spawn in self.raws.spawn_table.iter() {
            if !used_names.contains(&spawn.name) {
                println!(
                    "WARNING - Spawn tables references unspecified entity {}",
                    spawn.name
                );
            }
        }
    }
}

fn spawn_position<'a, T: Builder>(pos: SpawnType, new_entity: T, dirty: &mut Vec<(i32, i32)>) -> T {
    let mut eb = new_entity;

    // Spawn in the specified location
    match pos {
        SpawnType::AtPosition { x, y } => {
            let position = Position::new(x, y, dirty);
            eb = eb.with(position);
        }
    }

    eb
}

fn get_renderable_component(
    renderable: &super::item_structs::Renderable,
) -> crate::components::Renderable {
    crate::components::Renderable {
        glyph: rltk::to_cp437(renderable.glyph.chars().next().unwrap()),
        fg: rltk::RGB::from_hex(&renderable.fg).expect("Invalid RGB"),
        bg: rltk::RGB::from_hex(&renderable.bg).expect("Invalid RGB"),
        render_order: renderable.order,
    }
}

//my entity builder perso that can be constructed with an already existing entity
pub struct EntityBuilderPerso<'a> {
    /// The (already created) entity for which components will be inserted.
    pub entity: Entity,
    /// A reference to the `World` for component insertions.
    pub world: &'a World,
    built: bool,
}

impl<'a> EntityBuilderPerso<'a> {
    pub fn new(entity: Entity, world: &'a World) -> Self {
        EntityBuilderPerso {
            entity,
            world,
            built: false,
        }
    }
}

impl<'a> MarkedBuilder for EntityBuilderPerso<'a> {
    fn marked<M>(self) -> Self
    where
        M: Marker,
    {
        let mut alloc = self.world.write_resource::<M::Allocator>();
        alloc.mark(self.entity, &mut self.world.write_storage::<M>());

        self
    }
}

impl<'a> Builder for EntityBuilderPerso<'a> {
    /// Inserts a component for this entity.
    ///
    /// If a component was already associated with the entity, it will
    /// overwrite the previous component.
    #[inline]
    fn with<T: Component>(self, c: T) -> Self {
        {
            let mut storage: WriteStorage<T> = SystemData::fetch(&self.world);
            // This can't fail.  This is guaranteed by the lifetime 'a
            // in the EntityBuilder.
            storage.insert(self.entity, c).unwrap();
        }

        self
    }

    /// Finishes the building and returns the entity. As opposed to
    /// `LazyBuilder`, the components are available immediately.
    #[inline]
    fn build(mut self) -> Entity {
        self.built = true;
        self.entity
    }
}

pub fn spawn_named_item<T: Builder>(
    raws: &RawMaster,
    new_entity: T,
    key: &str,
    pos: SpawnType,
    dirty: &mut Vec<(i32, i32)>,
) -> Option<Entity> {
    if raws.item_index.contains_key(key) {
        let item_template = &raws.raws.items[raws.item_index[key]];

        let mut eb = new_entity;

        // Spawn in the specified location
        eb = spawn_position(pos, eb, dirty);

        // Renderable
        if let Some(renderable) = &item_template.renderable {
            eb = eb.with(get_renderable_component(renderable));
        }

        eb = eb.with(Name {
            name: item_template.name.clone(),
        });

        eb = eb.with(crate::components::Item {});

        if let Some(weapon) = &item_template.weapon {
            eb = eb.with(Equippable {
                slot: EquipmentSlot::Melee,
            });
            eb = eb.with(MeleePowerBonus {
                power: weapon.power_bonus,
            });
        }

        if let Some(shield) = &item_template.shield {
            eb = eb.with(Equippable {
                slot: EquipmentSlot::Shield,
            });
            eb = eb.with(DefenseBonus {
                defense: shield.defense_bonus,
            });
        }

        if let Some(consumable) = &item_template.consumable {
            eb = eb.with(consumable.clone());
        }

        if let Some(equipment_effects) = &item_template.equipment_effects {
            eb = eb.with(equipment_effects.clone());
        }

        if let Some(equippable) = &item_template.equippable {
            eb = eb.with(equippable.clone());
        }
        return Some(eb.build());
    }
    None
}

pub fn spawn_named_mob<T: Builder>(
    raws: &RawMaster,
    new_entity: T,
    key: &str,
    pos: SpawnType,
    dirty: &mut Vec<(i32, i32)>,
) -> Option<Entity> {
    if raws.mob_index.contains_key(key) {
        let mob_template = &raws.raws.mobs[raws.mob_index[key]];

        let mut eb = new_entity;

        // Spawn in the specified location
        eb = spawn_position(pos, eb, dirty);

        // Renderable
        if let Some(renderable) = &mob_template.renderable {
            eb = eb.with(get_renderable_component(renderable));
        }

        eb = eb.with(Name {
            name: mob_template.name.clone(),
        });

        eb = eb.with(Monster {});
        if mob_template.blocks_tile {
            eb = eb.with(BlocksTile {});
        }
        eb = eb.with(CombatStats {
            max_hp: mob_template.stats.max_hp,
            hp: mob_template.stats.hp,
            power: mob_template.stats.power,
            defense: mob_template.stats.defense,
            base_att: 0,
            base_def: 0,
            att: 0,
        });
        eb = eb.with(Viewshed {
            visible_tiles: Vec::new(),
            range: mob_template.vision_range,
            dirty: true,
        });

        return Some(eb.build());
    }
    None
}

//key is just a string, it's just the name of the entity
//TODO it's incomplete
pub fn spawn_named_prop<T: Builder>(
    raws: &RawMaster,
    new_entity: T,
    key: &str,
    pos: SpawnType,
    dirty: &mut Vec<(i32, i32)>,
) -> Option<Entity> {
    if raws.prop_index.contains_key(key) {
        let prop_template = &raws.raws.props[raws.prop_index[key]];

        let mut eb = new_entity;

        eb = eb.with(UniqueId::new());

        // Spawn in the specified location
        eb = spawn_position(pos, eb, dirty);

        // Renderable
        if let Some(renderable) = &prop_template.renderable {
            eb = eb.with(get_renderable_component(renderable));
        }

        eb = eb.with(Name {
            name: prop_template.name.clone(),
        });

        if let Some(blocks_tile) = prop_template.blocks_tile {
            if blocks_tile == true {
                eb = eb.with(BlocksTile {});
            }
        }

        // Interactable
        if let Some(interactable) = prop_template.interactable {
            if interactable {
                eb = eb.with(Interactable {})
            };
        }

        // InteractableObject
        if let Some(interactable_object) = &prop_template.interactable_object {
            eb = eb.with(interactable_object.clone()); //TODO comprendre pourquoi il ne fait pas comme ça( il passe par un itermediaire item_component)
        }

        if let Some(leaf) = prop_template.leaf {
            if leaf == true {
                eb = eb.with(Leaf { nutriments: 100 }); //TODO no default value
            }
        }

        if let Some(tree) = prop_template.tree {
            if tree == true {
                eb = eb.with(Tree {}); //TODO no default value
            }
        }

        // EnergyReserve
        if let Some(energy_reserve) = &prop_template.energy_reserve {
            eb = eb.with(EnergyReserve {
                reserve: energy_reserve.reserve,
                body_energy: energy_reserve.body_energy,
                max_reserve: energy_reserve.max_reserve,
                base_consumption: energy_reserve.base_consumption,
                hunger: Hunger::Full,
            });
        }

        // Viewshed
        if let Some(viewshed) = &prop_template.viewshed {
            eb = eb.with(Viewshed {
                visible_tiles: Vec::new(),
                range: viewshed.range,
                dirty: true,
            });
        }

        // Herbivore
        if let Some(herbivore) = &prop_template.herbivore {
            eb = eb.with(herbivore.clone()); //TODO no default value
        }

        // Reproduction
        if let Some(reproduction) = &prop_template.reproduction {
            eb = eb.with(reproduction.clone());
        }

        // Aging
        if let Some(aging) = &prop_template.aging {
            eb = eb.with(aging.clone());
        }

        // Temp Sensitivity
        if let Some(temp_sensi) = &prop_template.temp_sensi {
            eb = eb.with(temp_sensi.clone());
        }

        // HumiditySensitivity
        if let Some(hum_sensi) = &prop_template.hum_sensi {
            eb = eb.with(hum_sensi.clone());
        }

        // Specie
        if let Some(specie) = &prop_template.specie {
            eb = eb.with(specie.clone());
        }

        // Carnivore
        if let Some(carnivore) = &prop_template.carnivore {
            eb = eb.with(carnivore.clone());
        }

        // Speed
        if let Some(speed) = &prop_template.speed {
            eb = eb.with(speed.clone());
        }

        // Animal
        if let Some(animal) = &prop_template.animal {
            eb = eb.with(animal.clone());
        }

        // Sexe
        if let Some(sexe) = &prop_template.sexe {
            match sexe {
                SexeChoice::MaleOnly => eb = eb.with(Male {}),
                SexeChoice::FemaleOnly => eb = eb.with(Female {}),
                SexeChoice::MaleStart => eb = eb.with(Male {}),
                SexeChoice::FemaleStart => eb = eb.with(Female {}),
                SexeChoice::Random => {
                    let mut rng = rltk::RandomNumberGenerator::new();
                    let num_spawns = rng.roll_dice(1, 2);
                    if num_spawns == 1 {
                        eb = eb.with(Male {})
                    } else if num_spawns == 2 {
                        eb = eb.with(Female {})
                    } else {
                        println!("Error: imposible random number !")
                    }
                }
            }
        }

        // CombatStat
        if let Some(combat) = &prop_template.combat {
            eb = eb.with(combat.clone());
        }

        // BuildingChoice
        if let Some(building_choice) = &prop_template.building_choice {
            eb = eb.with(building_choice.clone());
        }

        // OnlinePlayer
        if let Some(online_player) = &prop_template.online_player {
            eb = eb.with(online_player.clone());
        }

        // Facing direction
        if let Some(facing_direction) = &prop_template.facing_direction {
            eb = eb.with(facing_direction.clone());
        }

        // PlayerInfo
        if let Some(player_info) = &prop_template.player_info {
            eb = eb.with(player_info.clone());
        }

        // Monster
        if let Some(monster) = &prop_template.monster {
            eb = eb.with(monster.clone());
        }

        // PlayerLog
        if let Some(player_log) = &prop_template.player_log {
            eb = eb.with(player_log.clone());
        }

        // Vegetable
        if let Some(vegetable) = &prop_template.vegetable {
            eb = eb.with(vegetable.clone());
        }

        // HordeTarget
        if let Some(horde_target) = &prop_template.horde_target {
            eb = eb.with(horde_target.clone());
        }

        // InHorde
        if let Some(in_horde) = &prop_template.in_horde {
            eb = eb.with(in_horde.clone());
        }

        // Faction
        if let Some(faction) = &prop_template.faction {
            eb = eb.with(faction.clone());
        }

        // DeathLoot
        if let Some(death_loot) = &prop_template.death_loot {
            eb = eb.with(death_loot.clone());
        }

        // LastMove
        if let Some(last_move) = &prop_template.last_move {
            eb = eb.with(last_move.clone());
        }

        return Some(eb.build());
    }
    None
}

//key is just a string, it's just the name of the entity
pub fn spawn_named_entity<T: Builder>(
    raws: &RawMaster,
    new_entity: T,
    key: &str,
    pos: SpawnType,
    dirty: &mut Vec<(i32, i32)>,
) -> Option<Entity> {
    if raws.item_index.contains_key(key) {
        return spawn_named_item(raws, new_entity, key, pos, dirty);
    } else if raws.mob_index.contains_key(key) {
        return spawn_named_mob(raws, new_entity, key, pos, dirty);
    } else if raws.prop_index.contains_key(key) {
        return spawn_named_prop(raws, new_entity, key, pos, dirty);
    } else {
        println!("ERROR: Key {} not found", key);
    }

    None
}

pub fn get_spawn_table_for_depth(raws: &RawMaster, depth: i32) -> RandomTable {
    use super::SpawnTableEntry;

    let available_options: Vec<&SpawnTableEntry> = raws
        .raws
        .spawn_table
        .iter()
        .filter(|a| depth >= a.min_depth && depth <= a.max_depth)
        .collect();

    let mut rt = RandomTable::new();
    for e in available_options.iter() {
        let mut weight = e.weight;
        if e.add_map_depth_to_weight.is_some() {
            weight += depth;
        }
        rt = rt.add(e.name.clone(), weight);
    }

    rt
}

pub fn spawn_born(
    raws: &RawMaster,
    new_entity: EntityBuilder,
    form: BirthForm,
    mutations: Mutations,
) -> Option<Entity> {
    let pos = form.position;
    let pos = SpawnType::AtPosition {
        x: pos.x(),
        y: pos.y(),
    };

    let key = &form.name.name;
    //TODO insert certificate or not ?

    let mut dirty = Vec::new();

    if raws.prop_index.contains_key(key) {
        let prop_template = &raws.raws.props[raws.prop_index[key]];

        let mut eb = new_entity;

        eb = eb.with(UniqueId::new());

        // Spawn in the specified location
        eb = spawn_position(pos, eb, &mut dirty);

        eb = eb.with(Name {
            name: prop_template.name.clone(),
        });

        /*****component with possible mutation */

        // Renderable
        if let Some(renderable) = mutations.renderable {
            eb = eb.with(renderable.clone());
        } else if let Some(renderable) = &prop_template.renderable {
            eb = eb.with(get_renderable_component(renderable));
        }

        // EnergyReserve
        if let Some(energy_reserve) = mutations.energy_reserve {
            eb = eb.with(energy_reserve.clone());
        } else if let Some(energy_reserve) = &prop_template.energy_reserve {
            eb = eb.with(EnergyReserve {
                reserve: energy_reserve.reserve,
                max_reserve: energy_reserve.max_reserve,
                body_energy: energy_reserve.body_energy,
                base_consumption: energy_reserve.base_consumption,
                hunger: Hunger::Full,
            });
        }

        // Reproduction
        if let Some(reproduction) = mutations.reproduction {
            eb = eb.with(reproduction.clone());
        } else if let Some(reproduction) = &prop_template.reproduction {
            eb = eb.with(reproduction.clone());
        }

        // Temp Sensitivity
        if let Some(temp_sensi) = mutations.temp_sensi {
            eb = eb.with(temp_sensi.clone());
        } else if let Some(temp_sensi) = &prop_template.temp_sensi {
            eb = eb.with(temp_sensi.clone());
        }

        // Humidity Sensitivity
        if let Some(hum_sensi) = mutations.hum_sensi {
            eb = eb.with(hum_sensi.clone());
        } else if let Some(hum_sensi) = &prop_template.hum_sensi {
            eb = eb.with(hum_sensi.clone());
        }

        // Specie
        if let Some(specie) = mutations.specie {
            eb = eb.with(specie.clone());
        } else if let Some(specie) = &prop_template.specie {
            eb = eb.with(specie.clone());
        }

        // Carnivore
        if let Some(carnivore) = mutations.carnivore {
            eb = eb.with(carnivore.clone());
        } else if let Some(carnivore) = &prop_template.carnivore {
            eb = eb.with(carnivore.clone());
        }

        // Speed
        if let Some(speed) = mutations.speed {
            eb = eb.with(speed.clone());
        } else if let Some(speed) = &prop_template.speed {
            eb = eb.with(speed.clone());
        }

        // Herbivore
        if let Some(herbivore) = mutations.herbivore {
            eb = eb.with(herbivore.clone());
        } else if let Some(herbivore) = &prop_template.herbivore {
            eb = eb.with(herbivore.clone()); //TODO no default value
        }

        // CombatStat
        if let Some(combat) = mutations.combat_stat {
            eb = eb.with(combat.clone());
        } else if let Some(combat) = &prop_template.combat {
            eb = eb.with(combat.clone());
        }

        /********************************** */

        if let Some(blocks_tile) = prop_template.blocks_tile {
            if blocks_tile == true {
                eb = eb.with(BlocksTile {});
            }
        }

        // Interactable
        if let Some(interactable) = prop_template.interactable {
            if interactable {
                eb = eb.with(Interactable {})
            };
        }

        // InteractableObject
        if let Some(interactable_object) = &prop_template.interactable_object {
            eb = eb.with(interactable_object.clone()); //TODO comprendre pourquoi il ne fait pas comme ça( il passe par un itermediaire item_component)
        }

        if let Some(leaf) = prop_template.leaf {
            if leaf == true {
                eb = eb.with(Leaf { nutriments: 100 }); //TODO no default value
            }
        }

        if let Some(tree) = prop_template.tree {
            if tree == true {
                eb = eb.with(Tree {}); //TODO no default value
            }
        }

        // Viewshed
        if let Some(viewshed) = &prop_template.viewshed {
            eb = eb.with(Viewshed {
                visible_tiles: Vec::new(),
                range: viewshed.range,
                dirty: true,
            });
        }

        // Aging
        if let Some(aging) = &prop_template.aging {
            eb = eb.with(aging.clone());
        }

        // Animal
        if let Some(animal) = &prop_template.animal {
            eb = eb.with(animal.clone());
        }

        // BuildingChoice
        if let Some(building_choice) = &prop_template.building_choice {
            eb = eb.with(building_choice.clone());
        }

        // OnlinePlayer
        if let Some(online_player) = &prop_template.online_player {
            eb = eb.with(online_player.clone());
        }

        // Facing direction
        if let Some(facing_direction) = &prop_template.facing_direction {
            eb = eb.with(facing_direction.clone());
        }

        // PlayerInfo
        if let Some(player_info) = &prop_template.player_info {
            eb = eb.with(player_info.clone());
        }

        // Monster
        if let Some(monster) = &prop_template.monster {
            eb = eb.with(monster.clone());
        }

        // LastMove
        if let Some(last_move) = &prop_template.last_move {
            eb = eb.with(last_move.clone());
        }

        // Sexe
        if let Some(sexe) = &prop_template.sexe {
            match sexe {
                SexeChoice::MaleOnly => eb = eb.with(Male {}),
                SexeChoice::FemaleOnly => eb = eb.with(Female {}),
                SexeChoice::Random | SexeChoice::FemaleStart | SexeChoice::MaleStart => {
                    let mut rng = rltk::RandomNumberGenerator::new();
                    let num_spawns = rng.roll_dice(1, 2);
                    if num_spawns == 1 {
                        eb = eb.with(Male {})
                    } else if num_spawns == 2 {
                        eb = eb.with(Female {})
                    } else {
                        println!("Error: imposible random number !")
                    }
                }
            }
        }

        return Some(eb.build());
    }
    None
}
