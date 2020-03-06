extern crate specs;
use crate::{
    gamelog::{GameLog, WorldStatLog},
    network, BuildingChoice, CombatStats, Connected, Item, Map, Name, OnlinePlayer, OnlineRunState,
    PlayerInfo, PlayerInput, PlayerInputComp, Position, Renderable, SerializeMe, Viewshed,
};
use rltk::RGB;
use specs::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::specs::saveload::{SimpleMarker, SimpleMarkerAllocator};

pub struct OnlinePlayerSystem {}

impl<'a> System<'a> for OnlinePlayerSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, Name>,
        WriteExpect<'a, WorldStatLog>,
        WriteExpect<'a, UuidPlayerHash>,
        WriteExpect<'a, Arc<Mutex<Vec<(network::Message, String)>>>>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, OnlinePlayer>,
        WriteStorage<'a, SimpleMarker<SerializeMe>>,
        WriteExpect<'a, SimpleMarkerAllocator<SerializeMe>>,
        WriteStorage<'a, Connected>,
        WriteStorage<'a, PlayerInputComp>,
        WriteStorage<'a, PlayerInfo>,
        WriteStorage<'a, BuildingChoice>,
        WriteStorage<'a, Item>,
        WriteExpect<'a, Map>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut entities,
            _log,
            mut names,
            mut _world_logs,
            mut player_hash,
            message_mutex,
            mut positions,
            mut renderables,
            mut combat_stats,
            mut viewsheds,
            mut online_players,
            mut storage,
            mut alloc,
            mut connecteds,
            mut player_inputs,
            player_infos,
            mut building_choices,
            items,
            map,
        ) = data;

        let mut player_messages: Vec<(Entity, network::Message)> = Vec::new();

        let mut new_player_list = Vec::new();

        {
            let mut message_list_guard = message_mutex.lock().unwrap();

            //todo hash map to get player entity

            for (net_mes, command) in message_list_guard.iter() {
                //println!("message list: {:?}, uid {}", net_mes, command);
                let mes = net_mes.clone();

                let mut uid = "".to_string();
                let mut player_entity: Option<&Entity> = None;
                let input;
                match mes.clone() {
                    network::Message::Registered(uuid) => {
                        uid = uuid.to_string();
                        player_entity = player_hash.hash.get(&uid.clone());
                        match player_entity {
                            Some(_entity) => {
                                println!("ERROR: someone want to register with an already use uuid")
                            }
                            None => {
                                new_player_list.push(uid.clone());
                            }
                        }
                        input = PlayerInput::NONE;
                    }
                    network::Message::RIGHT(uuid) => {
                        uid = uuid.to_string();
                        player_entity = player_hash.hash.get(&uid.clone());
                        input = PlayerInput::RIGHT
                    }
                    network::Message::LEFT(uuid) => {
                        uid = uuid.to_string();
                        player_entity = player_hash.hash.get(&uid.clone());
                        input = PlayerInput::LEFT
                    }
                    network::Message::UP(uuid) => {
                        uid = uuid.to_string();
                        player_entity = player_hash.hash.get(&uid.clone());
                        input = PlayerInput::UP
                    }
                    network::Message::DOWN(uuid) => {
                        uid = uuid.to_string();
                        player_entity = player_hash.hash.get(&uid.clone());
                        input = PlayerInput::DOWN
                    }
                    network::Message::PickUp(uuid) => {
                        uid = uuid.to_string();
                        player_entity = player_hash.hash.get(&uid.clone());
                        let mut target_item = None;

                        if let Some(entity) = player_entity {
                            if let Some(pos) = positions.get(*entity) {
                                for item_entity in map.tile_content[map.xy_idx(pos.x, pos.y)].iter()
                                {
                                    if let Some(_item) = items.get(*item_entity) {
                                        target_item = Some(item_entity);
                                    }
                                }
                            }
                        }

                        if let Some(target) = target_item {
                            input = PlayerInput::PICKUP(*target)
                        } else {
                            input = PlayerInput::NONE
                        }
                    }
                    network::Message::Build(uuid, x, y, name) => {
                        uid = uuid.to_string();
                        player_entity = player_hash.hash.get(&uid.clone());
                        input = PlayerInput::BUILD(x, y, name)
                    }
                    network::Message::Interact(uuid, x, y, name, id, gen) => {
                        uid = uuid.to_string();
                        player_entity = player_hash.hash.get(&uid.clone());
                        if let Some(entity) = player_entity {
                            let player_info = player_infos.get(*entity).unwrap();
                            let interacted_entity = get_interacted_entity(id, gen, player_info);
                            if let Some(inte_entity) = interacted_entity {
                                input = PlayerInput::INTERACT(x, y, name.clone(), inte_entity)
                            } else {
                                input = PlayerInput::NONE
                            }
                        } else {
                            input = PlayerInput::NONE
                        }
                    }
                    _ => input = PlayerInput::NONE,
                }

                match player_entity {
                    Some(entity) => {
                        player_messages.push((*entity, mes));

                        player_inputs
                            .insert(*entity, PlayerInputComp { input })
                            .expect("Unable to insert");

                        // if we received message of the player he is connected
                        //TODO have a timeout for the deconnection
                        connecteds
                            .insert(*entity, Connected { uuid: uid.clone() })
                            .expect("Unable to insert");
                    }
                    None => {}
                }

                //todo read the hash map to asociate the uid with an entity
                //attention si c'est un register on va pas avoir l'uid en faite.
                //Donc traiter dans network les autre message et ne renvoyer que les message avec uid en premier
                //pour le register faire un truc, pour l'instant justen créer uine nouvelle entier q chaque uid inconue
            }

            message_list_guard.clear();
        }

        //create new player
        for uid in new_player_list {
            let new_player;
            {
                new_player = spawn_online_player(
                    &mut entities,
                    &mut positions,
                    &mut renderables,
                    &mut combat_stats,
                    &mut viewsheds,
                    &mut online_players,
                    &mut names,
                    &mut building_choices,
                    &mut storage,
                    &mut alloc,
                    5,
                    5,
                );
            }

            player_hash.hash.insert(uid.clone(), new_player);
        }
    }
}

fn get_interacted_entity(id: u32, gen: i32, player_info: &PlayerInfo) -> Option<Entity> {
    let mut interacted_entity: Option<Entity> = None;
    for interaction in player_info.close_interations.iter() {
        if id == interaction.index && gen == interaction.generation {
            interacted_entity = Some(interaction.entity.unwrap()); // interaction.entity should not be an option but because of serialization shit I have to
            break;
        }
    }
    interacted_entity
}

pub struct PlayerMessages {
    requests: Vec<(Entity, network::Message)>,
}

impl PlayerMessages {
    #[allow(clippy::new_without_default)]
    pub fn new() -> PlayerMessages {
        PlayerMessages {
            requests: Vec::new(),
        }
    }

    pub fn request(&mut self, player_entity: Entity, message: network::Message) {
        self.requests.push((player_entity, message));
    }
}

//link the uiid with the correct player entity
pub struct UuidPlayerHash {
    pub hash: HashMap<String, Entity>,
}

impl UuidPlayerHash {
    #[allow(clippy::new_without_default)]
    pub fn new() -> UuidPlayerHash {
        UuidPlayerHash {
            hash: HashMap::new(),
        }
    }
}

/// Spawns the player and returns his/her entity object.
/// TODO faire un vair system de spawn qui va chercher dans le json
pub fn spawn_online_player<'a>(
    entities: &mut Entities<'a>,
    positions: &mut WriteStorage<'a, Position>,
    renderables: &mut WriteStorage<'a, Renderable>,
    combat_stats: &mut WriteStorage<'a, CombatStats>,
    viewsheds: &mut WriteStorage<'a, Viewshed>,
    online_players: &mut WriteStorage<'a, OnlinePlayer>,
    names: &mut WriteStorage<'a, Name>,
    building_choices: &mut WriteStorage<'a, BuildingChoice>,
    storage: &mut WriteStorage<'a, SimpleMarker<SerializeMe>>,
    alloc: &mut WriteExpect<'a, SimpleMarkerAllocator<SerializeMe>>,
    player_x: i32,
    player_y: i32,
) -> Entity {
    entities
        .build_entity()
        .with(
            Position {
                x: player_x,
                y: player_y,
            },
            positions,
        )
        .with(
            Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
                render_order: 0,
            },
            renderables,
        )
        .with(
            OnlinePlayer {
                runstate: OnlineRunState::AwaitingInput,
            },
            online_players,
        )
        .with(
            Viewshed {
                visible_tiles: Vec::new(),
                range: 30,
                dirty: true,
            },
            viewsheds,
        )
        .with(
            Name {
                name: "Player".to_string(),
            },
            names,
        )
        .with(
            CombatStats {
                max_hp: 30,
                hp: 30,
                defense: 2,
                power: 5,
            },
            combat_stats,
        )
        .with(
            BuildingChoice {
                plans: vec!["block".to_string()],
            },
            building_choices,
        )
        .marked(storage, alloc)
        .build()
}
