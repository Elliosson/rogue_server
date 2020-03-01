//This file is handling inputs of the local player
//this way we can play the game directly on the server
use super::{PlayerInput, PlayerInputComp};

extern crate rltk;
use rltk::{Rltk, VirtualKeyCode};

extern crate specs;
use specs::prelude::*;

pub fn local_player_input(ecs: &World, ctx: &mut Rltk) {
    let mut player_inputs = ecs.write_storage::<PlayerInputComp>();
    let local_player_entity = ecs.fetch::<Entity>();

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
            /*
                        // Picking up items
                        VirtualKeyCode::G => get_item(&mut gs.ecs),
                        VirtualKeyCode::I => return RunState::ShowInventory,
                        VirtualKeyCode::D => return RunState::ShowDropItem,
                        VirtualKeyCode::R => return RunState::ShowRemoveItem,

                        // Environement interaction
                        VirtualKeyCode::F => {
                            //interact(&mut gs.ecs); //TODO suppresse when we have a true systeme
                            return RunState::ObjectInteraction;
                        }

                        //Show Temperature Map
                        VirtualKeyCode::T => {
                            return RunState::TemperatureMap;
                        }

                        // Save and Quit
                        VirtualKeyCode::Escape => return RunState::SaveGame,
            */
            _ => None,
        },
    };
    if let Some(input) = input_op {
        player_inputs
            .insert(*local_player_entity, PlayerInputComp { input })
            .expect("Unable to insert");
    }
}
