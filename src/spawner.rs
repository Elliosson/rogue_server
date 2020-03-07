extern crate rltk;
use rltk::{RandomNumberGenerator, RGB};
extern crate specs;
use super::{
    map::MAPWIDTH, raws::*, CombatStats, Name, Player, Position, Rect, Renderable, SerializeMe,
    Viewshed,
};
use crate::components::*;
use crate::specs::saveload::{MarkedBuilder, SimpleMarker};
use specs::prelude::*;
use std::collections::HashMap;

/// Spawns the player and returns his/her entity object.
pub fn player(ecs: &mut World, player_x: i32, player_y: i32) -> Entity {
    ecs.create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
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
            hp: 30,
            defense: 2,
            power: 5,
        })
        .with(OnlinePlayer {
            runstate: OnlineRunState::AwaitingInput,
        })
        .with(BuildingChoice {
            plans: vec!["block".to_string()],
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

/// Fills a room with stuff!
#[allow(clippy::map_entry)]
pub fn spawn_trees(ecs: &mut World, room: &Rect) {
    let mut spawn_points: HashMap<usize, String> = HashMap::new();

    // Scope to keep the borrow checker happy
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        //let num_spawns = rng.roll_dice(600, 50);
        let num_spawns = 100000;

        for _i in 0..num_spawns {
            let mut added = false;
            let mut tries = 0;
            while !added && tries < 20 {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                if !spawn_points.contains_key(&idx) {
                    spawn_points.insert(idx, "Tree".to_string());
                    added = true;
                } else {
                    tries += 1;
                }
            }
        }
    }

    // Actually spawn the monsters
    for spawn in spawn_points.iter() {
        let x = (*spawn.0 % MAPWIDTH) as i32;
        let y = (*spawn.0 / MAPWIDTH) as i32;

        match spawn.1.as_ref() {
            "Tree" => {
                let raws: &RawMaster = &RAWS.lock().unwrap();
                if raws.prop_index.contains_key(spawn.1) {
                    let spawn_result = spawn_named_entity(
                        raws,
                        ecs.create_entity().marked::<SimpleMarker<SerializeMe>>(),
                        spawn.1,
                        SpawnType::AtPosition { x, y },
                    );
                    if spawn_result.is_some() {
                    } else {
                        println!("WARNING: We don't know how to spawn [{}]!", spawn.1);
                    }
                } else {
                    println!("WARNING: No keys !");
                }
            }
            _ => {}
        }
    }
}

pub struct ToSpawnList {
    requests: Vec<(i32, i32, String)>,
}

impl ToSpawnList {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ToSpawnList {
        ToSpawnList {
            requests: Vec::new(),
        }
    }

    pub fn request(&mut self, x: i32, y: i32, name: String) {
        self.requests.push((x, y, name));
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

pub fn spawn_named(ecs: &mut World, key: &str, x: i32, y: i32) {
    let raws: &RawMaster = &RAWS.lock().unwrap();
    if raws.prop_index.contains_key(key) {
        let spawn_result = spawn_named_entity(
            raws,
            ecs.create_entity().marked::<SimpleMarker<SerializeMe>>(),
            key,
            SpawnType::AtPosition { x, y },
        );
        if spawn_result.is_some() {
        } else {
            println!("WARNING: We don't know how to spawn [{}]!", key);
        }
    } else {
        println!("WARNING: No keys {} !", key);
    }
}
