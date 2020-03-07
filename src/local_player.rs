//This file is handling inputs of the local player
//this way we can play the game directly on the server
use super::{
    gui, CommandToConvert, EntityToConvert, Item, LocalClientInfo, LocalClientRunstate,
    PlayerInput, PlayerInputComp, Position, WantsToUseItem,
};

extern crate rltk;
use rltk::{Rltk, VirtualKeyCode};

extern crate specs;
use specs::prelude::*;

pub fn local_player_input(ecs: &World, ctx: &mut Rltk) {
    let mut client_info = ecs.write_resource::<LocalClientInfo>();

    client_info.local_runstate = match client_info.local_runstate {
        LocalClientRunstate::BaseState => local_client_base_state(ecs, ctx),
        LocalClientRunstate::Inventory => local_client_inventory(ecs, ctx),
        LocalClientRunstate::Interaction => local_client_interaction(ecs, ctx),
        LocalClientRunstate::Build => local_client_build(ecs, ctx),
    }
}

pub fn local_client_base_state(ecs: &World, ctx: &mut Rltk) -> LocalClientRunstate {
    let mut player_inputs = ecs.write_storage::<PlayerInputComp>();
    let local_player_entity = *ecs.fetch::<Entity>();

    let mut newrunstate = LocalClientRunstate::BaseState;

    // Player input
    let input_op: Option<PlayerInput> = match ctx.key {
        None => None,
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
                Some(PlayerInput::LEFT)
            }

            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
                Some(PlayerInput::RIGHT)
            }

            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                Some(PlayerInput::UP)
            }

            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                Some(PlayerInput::DOWN)
            }

            // Diagonals
            VirtualKeyCode::Numpad9 => None,

            VirtualKeyCode::Numpad7 => None,

            VirtualKeyCode::Numpad3 => None,

            VirtualKeyCode::Numpad1 => None,

            // Skip Turn
            VirtualKeyCode::Numpad5 | VirtualKeyCode::Space => None,

            // Level changes
            VirtualKeyCode::Period => None,

            // Picking up items
            VirtualKeyCode::G => {
                let target_item = get_item(ecs);
                if let Some(item) = target_item {
                    Some(PlayerInput::PICKUP(item))
                } else {
                    None
                }
            }
            VirtualKeyCode::I => {
                //we must print the inventory here since the inventory in only print client side
                //this is the getion of the gui that is client side normaly
                //the probleme of this thing is that it's stop all system but it's not a probleme for now

                newrunstate = LocalClientRunstate::Inventory;
                Some(PlayerInput::INVENTORY)
            }
            // VirtualKeyCode::D => return RunState::ShowDropItem,
            // VirtualKeyCode::R => return RunState::ShowRemoveItem,

            // Environement interaction
            VirtualKeyCode::F => {
                newrunstate = LocalClientRunstate::Interaction;
                Some(PlayerInput::NONE)
            }

            // Environement interaction
            VirtualKeyCode::B => {
                newrunstate = LocalClientRunstate::Build;
                Some(PlayerInput::NONE)
            }

            // //Show Temperature Map
            // VirtualKeyCode::T => {
            //     return RunState::TemperatureMap;
            // }

            // // Save and Quit
            // VirtualKeyCode::Escape => return RunState::SaveGame,
            _ => None,
        },
    };
    if let Some(input) = input_op {
        player_inputs
            .insert(local_player_entity, PlayerInputComp { input })
            .expect("Unable to insert");
    }

    newrunstate
}

pub fn local_client_inventory(ecs: &World, ctx: &mut Rltk) -> LocalClientRunstate {
    let mut newrunstate = LocalClientRunstate::Inventory;
    let result = gui::show_inventory(ecs, ctx);
    match result.0 {
        gui::ItemMenuResult::Cancel => newrunstate = LocalClientRunstate::BaseState,
        gui::ItemMenuResult::NoResponse => {}
        gui::ItemMenuResult::Selected => {
            let item_entity = result.1.unwrap();

            let mut intent = ecs.write_storage::<WantsToUseItem>();
            intent
                .insert(
                    *ecs.fetch::<Entity>(),
                    WantsToUseItem {
                        item: item_entity,
                        target: None,
                    },
                )
                .expect("Unable to insert intent");
            newrunstate = LocalClientRunstate::BaseState;
        }
    }

    newrunstate
}

pub fn local_client_interaction(ecs: &World, ctx: &mut Rltk) -> LocalClientRunstate {
    let local_player_entity = *ecs.fetch::<Entity>();
    let mut command_converts = ecs.write_storage::<EntityToConvert>();

    let mut newrunstate = LocalClientRunstate::Interaction;

    let result = gui::show_object_interaction_choice(ecs, ctx);
    match result.0 {
        gui::InteractionMenuResult::Cancel => newrunstate = LocalClientRunstate::BaseState,
        gui::InteractionMenuResult::NoResponse => {}
        gui::InteractionMenuResult::Selected => {
            let interaction_tuple = result.1.unwrap();
            let (x, y, interaction) = interaction_tuple;

            command_converts
                .insert(
                    local_player_entity,
                    EntityToConvert {
                        command: CommandToConvert::INTERACT(
                            x,
                            y,
                            interaction.interaction_name,
                            interaction.index,
                            interaction.generation,
                        ),
                    },
                )
                .expect("Unable to insert");

            newrunstate = LocalClientRunstate::BaseState;
        }
    }
    newrunstate
}

fn get_item(ecs: &World) -> Option<Entity> {
    let mut target_item: Option<Entity> = None;
    let positions = ecs.read_storage::<Position>();
    let items = ecs.read_storage::<Item>();
    let local_player_entity = *ecs.fetch::<Entity>();
    let player_pos = positions.get(local_player_entity).unwrap();
    let entities = ecs.entities();

    for (item_entity, _item, position) in (&entities, &items, &positions).join() {
        if position.x() == player_pos.x() && position.y() == player_pos.y() {
            target_item = Some(item_entity);
        }
    }
    target_item
}

pub fn local_client_build(ecs: &World, ctx: &mut Rltk) -> LocalClientRunstate {
    let local_player_entity = *ecs.fetch::<Entity>();
    let mut player_inputs = ecs.write_storage::<PlayerInputComp>();

    let mut newrunstate = LocalClientRunstate::Build;
    let result = gui::show_building_choice(ecs, ctx);

    match result.0 {
        gui::BuildingMenuResult::Cancel => newrunstate = LocalClientRunstate::BaseState,
        gui::BuildingMenuResult::NoResponse => {}
        gui::BuildingMenuResult::Selected => {
            let interaction_tuple = result.1.unwrap();
            let (x, y, building) = interaction_tuple;

            player_inputs
                .insert(
                    local_player_entity,
                    PlayerInputComp {
                        input: PlayerInput::BUILD(x, y, building),
                    },
                )
                .expect("Unable to insert");

            newrunstate = LocalClientRunstate::BaseState;
        }
    }
    newrunstate
}
