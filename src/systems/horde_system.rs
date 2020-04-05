extern crate specs;
use crate::{GoByStep, Horde, HordeTarget, InHorde, Point, ToSpawnList};
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
        WriteExpect<'a, rltk::RandomNumberGenerator>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, in_hordes, mut to_spawns, mut hordes, mut go_steps, horde_targets, mut rng) =
            data;

        //for now I don't handle multiple horde
        //todo this is ugly, handling multiple  horde
        let mut my_horde: Option<&mut Horde> = None;
        let base_cooldown = 100;
        let spawn_periode = 10;
        let spawn_center = point_in_ring(Point { x: 500, y: 500 }, 100, 200, &mut rng);

        for (_entity, horde) in (&entities, &mut hordes).join() {
            horde.timer -= 1;
            my_horde = Some(horde);
        }

        if let Some(horde) = my_horde {
            if entities.is_alive(horde.target) {
                //decremente time before next attaque
                // todo random spawn,
                let pos = point_in_area(spawn_center, 30, &mut rng);
                //just limite the number of spawn
                if horde.timer % spawn_periode == 0 {
                    //todo I need to have an easy way to give optional component to the spawner
                    to_spawns.request(pos.x, pos.y, "Basic Horde Monster".to_string());
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
                    horde.timer = base_cooldown * horde.wave;
                    horde.wave += 1;
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
                            timer: base_cooldown,
                            wave: 1,
                        },
                    )
                    .expect("Unable to inset");
            }
        }
    }
}

pub fn point_in_area<'a>(
    center: Point,
    radius: i32,
    rng: &mut WriteExpect<'a, rltk::RandomNumberGenerator>,
) -> Point {
    let x = center.x + rng.range(-radius, radius);
    let y = center.y + rng.range(-radius, radius);
    Point { x, y }
}

//this function generate a point in a squarred ring
pub fn point_in_ring<'a>(
    center: Point,
    inner_bound: i32,
    outter_bound: i32,
    rng: &mut WriteExpect<'a, rltk::RandomNumberGenerator>,
) -> Point {
    let a = rng.roll_dice(1, 2);
    let x;
    if a < 2 {
        x = center.x + rng.range(-outter_bound, -inner_bound);
    } else {
        x = center.x + rng.range(inner_bound, outter_bound);
    }

    let b = rng.roll_dice(1, 2);
    let y;
    if b < 2 {
        y = center.y + rng.range(-outter_bound, -inner_bound);
    } else {
        y = center.y + rng.range(inner_bound, outter_bound);
    }
    Point { x, y }
}
