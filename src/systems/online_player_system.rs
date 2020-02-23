extern crate specs;
use crate::{
    gamelog::{GameLog, WorldStatLog},
    network, CombatStats, Connected, Name, OnlinePlayer, OnlineRunState, Position, Renderable,
    SerializeMe, Viewshed, WantToMove,
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
        WriteStorage<'a, WantToMove>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, OnlinePlayer>,
        WriteStorage<'a, SimpleMarker<SerializeMe>>,
        WriteExpect<'a, SimpleMarkerAllocator<SerializeMe>>,
        WriteStorage<'a, Connected>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut entities,
            _log,
            mut names,
            mut _world_logs,
            mut player_hash,
            message_mutex,
            mut want_to_moves,
            mut positions,
            mut renderables,
            mut combat_stats,
            mut viewsheds,
            mut online_players,
            mut storage,
            mut alloc,
            mut connecteds,
        ) = data;

        let mut player_messages: Vec<(Entity, network::Message)> = Vec::new();

        let mut new_player_list = Vec::new();

        {
            let mut message_list_guard = message_mutex.lock().unwrap();

            //todo hash map to get player entity

            for (net_mes, command) in message_list_guard.iter() {
                println!("message list: {:?}, uid {}", net_mes, command);
                let mes = net_mes.clone();

                let mut uid = "".to_string();

                match mes {
                    network::Message::RIGHT(uuid) => uid = uuid.to_string(),
                    network::Message::LEFT(uuid) => uid = uuid.to_string(),
                    network::Message::UP(uuid) => uid = uuid.to_string(),
                    network::Message::DOWN(uuid) => uid = uuid.to_string(),
                    _ => {}
                }

                match player_hash.hash.get(&uid.clone()) {
                    Some(entity) => {
                        player_messages.push((*entity, mes));

                        // if we received message of the player he is connected
                        //TODO have a timeout for the deconnection
                        connecteds
                            .insert(*entity, Connected { uuid: uid.clone() })
                            .expect("Unable to insert");
                    }
                    None => {
                        new_player_list.push(uid.clone());
                    }
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
                    &mut storage,
                    &mut alloc,
                    5,
                    5,
                );
            }

            player_hash.hash.insert(uid.clone(), new_player);
        }

        // player_messages.push((player_entity, network::Message::Register));

        for (entity, message) in player_messages {
            let mut online_player = online_players.get_mut(entity).unwrap();
            //execute runstate
            let newrunstate = online_runstate_choice(
                online_player.runstate.clone(),
                entity,
                message,
                &mut want_to_moves,
            );
            online_player.runstate = newrunstate;
        }
    }
}

pub fn online_runstate_choice<'a>(
    runstate: OnlineRunState,
    entity: Entity,
    message: network::Message,
    want_to_moves: &mut WriteStorage<'a, WantToMove>,
) -> OnlineRunState {
    let newrunstate;
    match runstate {
        OnlineRunState::AwaitingInput => {
            newrunstate = online_player_input(entity, message, want_to_moves);
        }
        OnlineRunState::PlayerTurn => {
            newrunstate = OnlineRunState::AwaitingInput;
        }
    }
    newrunstate
}

pub fn online_player_input<'a>(
    entity: Entity,
    message: network::Message,
    want_to_move: &mut WriteStorage<'a, WantToMove>,
) -> OnlineRunState {
    // Player movement

    //get the last input for the online player

    match message {
        network::Message::UP(_uuid) => {
            want_to_move
                .insert(
                    entity,
                    WantToMove {
                        delta_x: 0,
                        delta_y: -1,
                    },
                )
                .expect("Unable to insert");
        }
        network::Message::DOWN(_uuid) => {
            want_to_move
                .insert(
                    entity,
                    WantToMove {
                        delta_x: 0,
                        delta_y: 1,
                    },
                )
                .expect("Unable to insert");
        }
        network::Message::LEFT(_uuid) => {
            want_to_move
                .insert(
                    entity,
                    WantToMove {
                        delta_x: -1,
                        delta_y: 0,
                    },
                )
                .expect("Unable to insert");
        }
        network::Message::RIGHT(_uuid) => {
            want_to_move
                .insert(
                    entity,
                    WantToMove {
                        delta_x: 1,
                        delta_y: 0,
                    },
                )
                .expect("Unable to insert");
        }
        _ => {}
    }

    OnlineRunState::PlayerTurn
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
pub fn spawn_online_player<'a>(
    entities: &mut Entities<'a>,
    positions: &mut WriteStorage<'a, Position>,
    renderables: &mut WriteStorage<'a, Renderable>,
    combat_stats: &mut WriteStorage<'a, CombatStats>,
    viewsheds: &mut WriteStorage<'a, Viewshed>,
    online_players: &mut WriteStorage<'a, OnlinePlayer>,
    names: &mut WriteStorage<'a, Name>,
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
                range: 8,
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
        .marked(storage, alloc)
        .build()
}
