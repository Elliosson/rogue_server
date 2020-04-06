extern crate specs;
use crate::{
    gamelog::{GameLog, WorldStatLog},
    network, Connected, Item, Map, PlayerInfo, PlayerInput, PlayerInputComp, Position,
    ToConstructList,
};

use specs::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub const STARTING_POS_X: i32 = 505;
pub const STARTING_POS_Y: i32 = 505;

pub struct OnlinePlayerSystem {}

impl<'a> System<'a> for OnlinePlayerSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, WorldStatLog>,
        WriteExpect<'a, UuidPlayerHash>,
        WriteExpect<'a, NamePlayerHash>,
        WriteExpect<'a, Arc<Mutex<Vec<(network::Message, String)>>>>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Connected>,
        WriteStorage<'a, PlayerInputComp>,
        WriteStorage<'a, PlayerInfo>,
        WriteStorage<'a, Item>,
        WriteExpect<'a, Map>,
        WriteExpect<'a, ToConstructList>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            _log,
            mut _world_logs,
            mut player_hash,
            mut pseudo_player_hash,
            message_mutex,
            positions,
            mut connecteds,
            mut player_inputs,
            player_infos,
            items,
            map,
            mut to_construct,
        ) = data;

        let mut player_messages: Vec<(Entity, network::Message)> = Vec::new();

        let mut new_player_list = Vec::new();

        {
            let mut message_list_guard = message_mutex.lock().unwrap();

            //todo hash map to get player entity

            for (net_mes, _command) in message_list_guard.iter() {
                //println!("message list: {:?}, uid {}", net_mes, command);
                let mes = net_mes.clone();

                let mut uid = "".to_string();
                let mut player_entity: Option<&Entity> = None;
                let input;
                match mes.clone() {
                    network::Message::Registered(uuid, name) => {
                        uid = uuid.to_string();
                        player_entity = player_hash.hash.get(&uid.clone());
                        match player_entity {
                            Some(_entity) => {
                                println!("ERROR: someone want to register with an already use uuid")
                            }
                            None => {
                                new_player_list.push((uid.clone(), name));
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
                    network::Message::Destroy(uuid) => {
                        uid = uuid.to_string();
                        player_entity = player_hash.hash.get(&uid.clone());
                        input = PlayerInput::DESTROY
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
                                if let Some(tile_content) =
                                    map.tile_content.get(&map.xy_idx(pos.x(), pos.y()))
                                {
                                    for item_entity in tile_content.iter() {
                                        if let Some(_item) = items.get(*item_entity) {
                                            target_item = Some(item_entity);
                                        }
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
                    network::Message::Consume(uuid, id, gen) => {
                        uid = uuid.to_string();
                        player_entity = player_hash.hash.get(&uid.clone());
                        if let Some(entity) = player_entity {
                            let player_info = player_infos.get(*entity).unwrap();
                            let interacted_entity = get_inventory_entity(id, gen, player_info);
                            println!("pass 1");
                            if let Some(inte_entity) = interacted_entity {
                                println!("pass 2");
                                input = PlayerInput::CONSUME(inte_entity)
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

                        match player_inputs.insert(*entity, PlayerInputComp { input }) {
                            Err(e) => {
                                println!("Error: Can't find the player. {}", e)
                                //TODO if the player have no entity I should either recreate one or suppress the entry from the hashmaps
                            }
                            Ok(_) => {
                                // if we received message of the player he is connected
                                //TODO have a timeout for the deconnection
                                connecteds
                                    .insert(*entity, Connected { uuid: uid.clone() })
                                    .expect("Unable to insert");
                            }
                        }
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
        for (uid, pseudo) in new_player_list {
            //connect to this pseudo if he is already used
            let player_entity;
            if let Some(&entity) = pseudo_player_hash.hash.get(&pseudo) {
                //If he is still connected remove the old entity of uid_hash
                if let Some(connected) = connecteds.get(entity) {
                    player_hash.hash.remove(&connected.uuid);
                }

                player_entity = entity
            } else {
                let new_player = entities.create();
                to_construct.request(
                    STARTING_POS_X,
                    STARTING_POS_Y,
                    "Online Player".to_string(),
                    new_player,
                );
                player_entity = new_player;
            }
            connecteds
                .insert(player_entity, Connected { uuid: uid.clone() })
                .expect("Unable to insert");
            pseudo_player_hash
                .hash
                .insert(pseudo.clone(), player_entity);
            player_hash.hash.insert(uid.clone(), player_entity);
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

fn get_inventory_entity(id: u32, gen: i32, player_info: &PlayerInfo) -> Option<Entity> {
    let mut item_entity: Option<Entity> = None;
    for item in player_info.inventaire.iter() {
        if id == item.index && gen == item.generation {
            item_entity = Some(item.entity.unwrap());
            break;
        }
    }
    item_entity
}

pub struct PlayerMessages {
    pub requests: Vec<(Entity, network::Message)>,
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

//link the uiid with the correct player entity
pub struct NamePlayerHash {
    pub hash: HashMap<String, Entity>,
}

impl NamePlayerHash {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        NamePlayerHash {
            hash: HashMap::new(),
        }
    }
}
