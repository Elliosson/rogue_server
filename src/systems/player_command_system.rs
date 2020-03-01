extern crate specs;
use crate::{
    gamelog::GameLog, InteractionResquestListV2, OnlinePlayer, OnlineRunState, PlayerInput,
    PlayerInputComp, WantBuild, WantToMove, WantsToPickupItem,
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
        WriteStorage<'a, WantBuild>,
        WriteExpect<'a, InteractionResquestListV2>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            _log,
            mut want_to_moves,
            mut online_players,
            mut player_inputs,
            mut pickups,
            mut want_builds,
            mut interaction_requestsv2,
        ) = data;

        for (entity, player_input) in (&entities, &player_inputs).join() {
            let mut online_player = online_players.get_mut(entity).unwrap();
            //execute runstate
            let newrunstate = online_runstate_choice(
                online_player.runstate.clone(),
                entity,
                player_input.input.clone(),
                &mut want_to_moves,
                &mut pickups,
                &mut interaction_requestsv2,
                &mut want_builds,
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
    interations_req: &mut WriteExpect<'a, InteractionResquestListV2>,
    want_builds: &mut WriteStorage<'a, WantBuild>,
) -> OnlineRunState {
    let newrunstate;
    match runstate {
        OnlineRunState::AwaitingInput => {
            newrunstate = online_player_input(
                entity,
                message.clone(),
                want_to_moves,
                pickups,
                interations_req,
                want_builds,
            );
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
    interations_req: &mut WriteExpect<'a, InteractionResquestListV2>,
    want_builds: &mut WriteStorage<'a, WantBuild>,
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
        PlayerInput::INTERACT(x, y, name, target) => {
            interations_req.request(x, y, name, target, entity);
        }
        PlayerInput::BUILD(x, y, name) => {
            want_builds
                .insert(entity, WantBuild { x, y, name })
                .expect("Unable to insert ");
        }
        _ => {}
    }

    OnlineRunState::AwaitingInput
}
