extern crate specs;
use crate::{GoByStep, Horde, HordeTarget, InHorde, ToSpawnList};
use specs::prelude::*;

pub struct HordeSystem {}

impl<'a> System<'a> for HordeSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, InHorde>,
        WriteExpect<'a, ToSpawnList>,
        WriteStorage<'a, Horde>,
        WriteStorage<'a, GoByStep>,
        WriteStorage<'a, HordeTarget>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, in_hordes, mut to_spawns, mut hordes, mut go_steps, horde_targets) = data;

        //for now I don't handle multiple horde
        //todo this is ugly, handling multiple  horde
        let mut my_horde: Option<Horde> = None;

        for (_entity, horde) in (&entities, &mut hordes).join() {
            horde.timer -= 1;
            my_horde = Some(horde.clone());
        }

        if let Some(horde) = my_horde {
            if entities.is_alive(horde.target) {
                //decremente time before next attaque
                // todo random spawn,
                let x = 20;
                let y = 20;
                //just limite the number of spawn
                if horde.timer % 10 == 0 {
                    //todo I need to have an easy way to give optional component to the spawner
                    to_spawns.request(x, y, "Basic Horde Monster".to_string());
                }
                // pos some monster time to time
                //if timer = 0 lauch the attack
                if horde.timer < 0 {
                    for (entity, _in_horde) in (&entities, &in_hordes).join() {
                        go_steps
                            .insert(
                                entity,
                                GoByStep {
                                    target: horde.target,
                                },
                            )
                            .expect("Unable to insert");
                    }
                }
            } else {
                println!(
                    "Error: An horde is still alive with a death target, this should not happend"
                )
            }
        } else {
            //if there no horde I try to creat one on a suitable target
            //todo find a got way to choose the new target , this is shit
            //to be clear only one is choosen here
            let mut new_target = None;
            for (entity, _horde_target) in (&entities, &horde_targets).join() {
                new_target = Some(entity);
            }
            if let Some(entity) = new_target {
                hordes
                    .insert(
                        entity,
                        Horde {
                            target: entity,
                            timer: 100,
                        },
                    )
                    .expect("Unable to inset");
            }
        }
    }
}
