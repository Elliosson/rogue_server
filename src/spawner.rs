extern crate rltk;
use rltk::{RandomNumberGenerator, RGB};
extern crate specs;
use super::{
    map::MAPWIDTH, raws::*, CombatStats, Map, Name, Player, Position, Rect, Renderable,
    SerializeMe, Viewshed,
};
use crate::components::*;
use crate::specs::saveload::{MarkedBuilder, SimpleMarker};
use specs::prelude::*;

use std::collections::HashMap;

/// Spawns the player and returns his/her entity object.
pub fn player(ecs: &mut World, player_x: i32, player_y: i32) -> Entity {
    let mut dirty = Vec::new();
    let entity = ecs
        .create_entity()
        .with(Position::new(player_x, player_y, &mut dirty))
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            render_order: 0,
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name {
            name: "Player".to_string(),
        })
        .with(CombatStats {
            max_hp: 30,
            hp: 20,
            defense: 2,
            power: 5,
            base_att: 2,
            base_def: 2,
            att: 0,
        })
        .with(OnlinePlayer {
            runstate: OnlineRunState::AwaitingInput,
        })
        .with(BuildingChoice {
            plans: vec![
                "block".to_string(),
                "Door".to_string(),
                "Garden".to_string(),
                "Craft Shop".to_string(),
            ],
        })
        .with(FacingDirection {
            orientation: Orientation::East,
            front_tile: rltk::Point { x: 1, y: 0 },
        })
        .with(PlayerInfo {
            inventaire: Vec::new(),
            close_interations: Vec::new(),
            possible_builds: Vec::new(),
            my_info: MyInfo {
                pos: rltk::Point { x: 0, y: 0 },
                hp: 0,
                max_hp: 0,
                player_log: Vec::new(),
            },
            equipement: Vec::new(),
            combat_stats: Default::default(),
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();

    let mut map = ecs.write_resource::<Map>();
    map.dirty.append(&mut dirty);

    entity
}

/// Fills a room with stuff!
#[allow(clippy::map_entry)]
pub fn spawn_named_everywhere(ecs: &mut World, room: &Rect, name: String, num_spawns: i32) {
    let mut spawn_points: HashMap<usize, String> = HashMap::new();

    // Scope to keep the borrow checker happy
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();

        for _i in 0..num_spawns {
            let mut added = false;
            let mut tries = 0;
            while !added && tries < 20 {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                if !spawn_points.contains_key(&idx) {
                    spawn_points.insert(idx, name.clone());
                    added = true;
                } else {
                    tries += 1;
                }
            }
        }
    }
    let mut dirty = Vec::new();

    // Actually spawn the named
    for spawn in spawn_points.iter() {
        let x = (*spawn.0 % MAPWIDTH) as i32;
        let y = (*spawn.0 / MAPWIDTH) as i32;

        let raws: &RawMaster = &RAWS.lock().unwrap();
        if raws.prop_index.contains_key(spawn.1) || raws.item_index.contains_key(spawn.1) {
            let spawn_result = spawn_named_entity(
                raws,
                ecs.create_entity().marked::<SimpleMarker<SerializeMe>>(),
                spawn.1,
                SpawnType::AtPosition { x, y },
                &mut dirty,
            );
            if let Some(entity) = spawn_result {
                let mut map: specs::shred::FetchMut<Map> = ecs.write_resource::<Map>();
                let idx = map.xy_idx(x, y);
                let tile_content = map.tile_content.entry(idx).or_insert(Vec::new());
                tile_content.push(entity);
                map.dirty.append(&mut dirty);
            } else {
                println!("WARNING: We don't know how to spawn [{}]!", spawn.1);
            }
        } else {
            println!("WARNING: No keys !");
        }
    }

    let mut map = ecs.write_resource::<Map>();
    map.dirty.append(&mut dirty);
}

pub struct ToSpawnList {
    requests: Vec<(i32, i32, String)>,
}

impl ToSpawnList {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        ToSpawnList {
            requests: Vec::new(),
        }
    }

    pub fn request(&mut self, x: i32, y: i32, name: String) {
        self.requests.push((x, y, name));
    }
}

pub struct ToConstructList {
    requests: Vec<(i32, i32, String, Entity)>,
}

impl ToConstructList {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        ToConstructList {
            requests: Vec::new(),
        }
    }

    pub fn request(&mut self, x: i32, y: i32, name: String, entity: Entity) {
        self.requests.push((x, y, name, entity));
    }
}

pub fn spawner_named(ecs: &mut World) {
    let mut spawns_temps: Vec<(i32, i32, String)> = Vec::new();
    {
        let to_spawns = ecs.write_resource::<ToSpawnList>();

        for (x, y, name) in to_spawns.requests.iter() {
            spawns_temps.push((*x, *y, name.clone()))
        }
    }
    for (x, y, name) in spawns_temps.iter() {
        spawn_named(ecs, &name.clone(), *x, *y)
    }
    let mut to_spawns = ecs.write_resource::<ToSpawnList>();

    to_spawns.requests.clear();
}

pub fn constructer_named(ecs: &mut World) {
    let mut spawns_temps: Vec<(i32, i32, String, Entity)> = Vec::new();
    {
        let to_constructs = ecs.write_resource::<ToConstructList>();

        for (x, y, name, entity) in to_constructs.requests.iter() {
            spawns_temps.push((*x, *y, name.clone(), *entity))
        }
    }
    for (x, y, name, entity) in spawns_temps.iter() {
        construct_named(ecs, &name.clone(), *x, *y, *entity)
    }
    let mut to_constructs = ecs.write_resource::<ToConstructList>();

    to_constructs.requests.clear();
}

// like create named but with an already existing entity
//todo destroy entity id not builded, this could pose probleme honestly, I have to think about this
//TODO factorize avec spawn
pub fn construct_named(ecs: &mut World, key: &str, x: i32, y: i32, entity: Entity) {
    println!("pass constructe {}", key);
    let raws: &RawMaster = &RAWS.lock().unwrap();
    //let mut entity_builder = ecs.create_entity();
    //entity_builder.entity = entity; //TODO that seem ungly, I don't now if it's ok
    let mut dirty = Vec::new();
    if raws.prop_index.contains_key(key) || raws.item_index.contains_key(key) {
        let entity_builder = EntityBuilderPerso::new(entity, ecs);
        let spawn_result = spawn_named_entity(
            raws,
            entity_builder.marked::<SimpleMarker<SerializeMe>>(),
            key,
            SpawnType::AtPosition { x, y },
            &mut dirty,
        );
        if let Some(entity) = spawn_result {
            //todo honesstly the only good wa ywould be to be sure that the enity is insert on ly once in the vec of the hash map ut I don't now how to do that
            let mut map: specs::shred::FetchMut<Map> = ecs.write_resource::<Map>();
            let idx = map.xy_idx(x, y);
            let tile_content = map.tile_content.entry(idx).or_insert(Vec::new());
            tile_content.push(entity);
            map.dirty.append(&mut dirty);
        } else {
            println!(
                "ERROR: An enitity is left with no componant .We don't know how to spawn [{}]!",
                key
            );
        }
    } else {
        println!(
            "ERROR: An enitity is left with no componant. No keys {} !",
            key
        );
    }
}

pub fn spawn_named(ecs: &mut World, key: &str, x: i32, y: i32) {
    let raws: &RawMaster = &RAWS.lock().unwrap();
    let mut dirty = Vec::new();
    if raws.prop_index.contains_key(key) || raws.item_index.contains_key(key) {
        let spawn_result = spawn_named_entity(
            raws,
            ecs.create_entity().marked::<SimpleMarker<SerializeMe>>(),
            key,
            SpawnType::AtPosition { x, y },
            &mut dirty,
        );
        if let Some(entity) = spawn_result {
            //todo honesstly the only good wa ywould be to be sure that the enity is insert on ly once in the vec of the hash map ut I don't now how to do that
            let mut map: specs::shred::FetchMut<Map> = ecs.write_resource::<Map>();
            let idx = map.xy_idx(x, y);
            let tile_content = map.tile_content.entry(idx).or_insert(Vec::new());
            tile_content.push(entity);
            map.dirty.append(&mut dirty);
        } else {
            println!("WARNING: We don't know how to spawn [{}]!", key);
        }
    } else {
        println!("WARNING: No keys {} !", key);
    }
}
