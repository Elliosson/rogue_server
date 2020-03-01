extern crate specs;
use crate::{
    gamelog::GameLog, OnlinePlayer, OnlineRunState, PlayerInput, PlayerInputComp, WantToMove,
    WantsToPickupItem,
};
use specs::prelude::*;

pub struct PlayerCommandSystem {}

//this system system will take the command from a player entity activate the proper action
//this can be interpreted as a state machine to decide of the correct action  to make according to previous action
impl<'a> System<'a> for PlayerCommandSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantToMove>,
        WriteStorage<'a, OnlinePlayer>,
        WriteStorage<'a, PlayerInputComp>,
        WriteStorage<'a, WantsToPickupItem>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, mut want_to_moves, mut online_players, mut player_inputs, mut pickups) =
            data;

        for (entity, player_input) in (&entities, &player_inputs).join() {
            let mut online_player = online_players.get_mut(entity).unwrap();
            //execute runstate
            let newrunstate = online_runstate_choice(
                online_player.runstate.clone(),
                entity,
                player_input.input.clone(),
                &mut want_to_moves,
                &mut pickups,
            );
            online_player.runstate = newrunstate;
        }
        player_inputs.clear();
    }
}

pub fn online_runstate_choice<'a>(
    runstate: OnlineRunState,
    entity: Entity,
    message: PlayerInput,
    want_to_moves: &mut WriteStorage<'a, WantToMove>,
    pickups: &mut WriteStorage<'a, WantsToPickupItem>,
) -> OnlineRunState {
    let newrunstate;
    match runstate {
        OnlineRunState::AwaitingInput => {
            newrunstate = online_player_input(entity, message.clone(), want_to_moves, pickups);
        }
        OnlineRunState::PlayerTurn => {
            newrunstate = OnlineRunState::AwaitingInput;
        }
    }
    newrunstate
}

pub fn online_player_input<'a>(
    entity: Entity,
    message: PlayerInput,
    want_to_move: &mut WriteStorage<'a, WantToMove>,
    pickups: &mut WriteStorage<'a, WantsToPickupItem>,
) -> OnlineRunState {
    // Player movement

    //get the last input for the online player

    match message {
        PlayerInput::UP => {
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
        PlayerInput::DOWN => {
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
        PlayerInput::LEFT => {
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
        PlayerInput::RIGHT => {
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
        PlayerInput::INVENTORY => {}
        PlayerInput::PICKUP(item) => {
            pickups
                .insert(
                    entity,
                    WantsToPickupItem {
                        collected_by: entity,
                        item,
                    },
                )
                .expect("Unable to insert want to pickup");
        }
        _ => {}
    }

    OnlineRunState::AwaitingInput
}
