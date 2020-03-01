//This file is handling inputs of the local player
//this way we can play the game directly on the server
use super::{
    gui, LocalClientInfo, LocalClientRunstate, PlayerInput, PlayerInputComp, Ranged, WantsToUseItem,
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
    }
}

pub fn local_client_base_state(ecs: &World, ctx: &mut Rltk) -> LocalClientRunstate {
    let mut player_inputs = ecs.write_storage::<PlayerInputComp>();
    let local_player_entity = ecs.fetch::<Entity>();

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
            VirtualKeyCode::Numpad9 | VirtualKeyCode::U => None,

            VirtualKeyCode::Numpad7 | VirtualKeyCode::Y => None,

            VirtualKeyCode::Numpad3 | VirtualKeyCode::N => None,

            VirtualKeyCode::Numpad1 | VirtualKeyCode::B => None,

            // Skip Turn
            VirtualKeyCode::Numpad5 | VirtualKeyCode::Space => None,

            // Level changes
            VirtualKeyCode::Period => None,

            // Picking up items
            // VirtualKeyCode::G => get_item(&mut gs.ecs),
            VirtualKeyCode::I => {
                //we must print the inventory here since the inventory in only print client side
                //this is the getion of the gui that is client side normaly
                //the probleme of this thing is that it's stop all system but it's not a probleme for now

                newrunstate = LocalClientRunstate::Inventory;
                Some(PlayerInput::INVENTORY)
            }
            // VirtualKeyCode::D => return RunState::ShowDropItem,
            // VirtualKeyCode::R => return RunState::ShowRemoveItem,

            // // Environement interaction
            // VirtualKeyCode::F => {
            //     //interact(&mut gs.ecs); //TODO suppresse when we have a true systeme
            //     return RunState::ObjectInteraction;
            // }

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
            .insert(*local_player_entity, PlayerInputComp { input })
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
