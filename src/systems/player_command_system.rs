extern crate specs;
use crate::{
    gamelog::GameLog, InteractionResquestListV2, OnlinePlayer, OnlineRunState, PlayerInput,
    PlayerInputComp, WantBuild, WantConsume, WantDestroy, WantEquip, WantToMove, WantsToPickupItem,
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
        WriteStorage<'a, WantDestroy>,
        WriteStorage<'a, WantConsume>,
        WriteStorage<'a, WantEquip>,
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
            mut want_destroys,
            mut want_consumes,
            mut want_equips,
        ) = data;

        for (entity, player_input) in (&entities, &player_inputs).join() {
            if let Some(online_player) = online_players.get_mut(entity) {
                //execute runstate
                let newrunstate = online_runstate_choice(
                    online_player.runstate.clone(),
                    entity,
                    player_input.input.clone(),
                    &mut want_to_moves,
                    &mut pickups,
                    &mut interaction_requestsv2,
                    &mut want_builds,
                    &mut want_destroys,
                    &mut want_consumes,
                    &mut want_equips,
                );
                online_player.runstate = newrunstate;
            } else {
                println!("Warning: Input from a non OnlinePlayer")
            }
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
    want_destroys: &mut WriteStorage<'a, WantDestroy>,
    want_consumes: &mut WriteStorage<'a, WantConsume>,
    want_equips: &mut WriteStorage<'a, WantEquip>,
) -> OnlineRunState {
    let newrunstate;
    match runstate {
        OnlineRunState::AwaitingInput => {
            newrunstate = OnlineRunState::AwaitingInput;
            match message {
                PlayerInput::UP => {
                    want_to_moves
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
                    want_to_moves
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
                    want_to_moves
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
                    want_to_moves
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
                PlayerInput::DESTROY => {
                    want_destroys
                        .insert(entity, WantDestroy {})
                        .expect("Unable to insert ");
                }
                PlayerInput::CONSUME(target) => {
                    want_consumes
                        .insert(entity, WantConsume { target })
                        .expect("Unable to insert ");

                    //todo this is a provisory system util I have a specific equipment panel
                    want_equips
                        .insert(entity, WantEquip { target })
                        .expect("Unable to insert ");
                }
                PlayerInput::EQUIP(target) => {
                    want_equips
                        .insert(entity, WantEquip { target })
                        .expect("Unable to insert ");
                }
                _ => {}
            }
        }
        OnlineRunState::PlayerTurn => {
            newrunstate = OnlineRunState::AwaitingInput;
        }
    }
    newrunstate
}
