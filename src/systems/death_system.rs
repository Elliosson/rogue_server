extern crate specs;
use crate::specs::saveload::{SimpleMarker, SimpleMarkerAllocator};
use crate::{
    gamelog::{GameLog, GeneralLog},
    Dead, DeathCause, EnergyReserve, Meat, MyChoosenFood, Point, Position, Renderable, SerializeMe,
    Specie, TargetedForEat, ToDelete,
};
use rltk::RGB;
use specs::prelude::*;

pub struct DeathSystem {}

impl<'a> System<'a> for DeathSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, GeneralLog>,
        WriteStorage<'a, ToDelete>,
        WriteStorage<'a, Dead>,
        WriteStorage<'a, Meat>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, EnergyReserve>,
        WriteStorage<'a, TargetedForEat>,
        WriteStorage<'a, MyChoosenFood>,
        WriteStorage<'a, Specie>,
        WriteStorage<'a, SimpleMarker<SerializeMe>>,
        WriteExpect<'a, SimpleMarkerAllocator<SerializeMe>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            _log,
            mut general_logs,
            mut to_deletes,
            mut deads,
            mut meats,
            mut positions,
            mut renderables,
            energies,
            mut targeted_eats,
            mut my_choosen_foods,
            species,
            mut simple_markers,
            mut simple_marker_allocators,
        ) = data;

        for (entity, dead) in (&entities, &mut deads).join() {
            to_deletes
                .insert(entity, ToDelete {})
                .expect("Unable to insert");
        }

        deads.clear();
        /*
        for (entity, dead, energy) in (&entities, &mut deads, &energies).join() {
            //create meat entity
            let mut dirty = Vec::new();
            let x;
            let y;
            {
                let pos = positions.get(entity).unwrap();
                x = pos.x();
                y = pos.y();
            }
            let mut eb = entities
                .build_entity()
                .marked(&mut simple_markers, &mut simple_marker_allocators)
                .with(
                    Renderable {
                        glyph: rltk::to_cp437('*'),
                        fg: RGB::named(rltk::RED),
                        bg: RGB::named(rltk::BLACK),
                        render_order: 1,
                    },
                    &mut renderables,
                )
                .with(Position::new(x, y, &mut dirty), &mut positions)
                .with(
                    //add animal energy to the meat
                    Meat {
                        nutriments: energy.body_energy + energy.reserve,
                    },
                    &mut meats,
                );

            //if the entity have been killed, mark the food for the killer
            let meat_entity;
            match dead.cause {
                DeathCause::Killed { killer } => {
                    eb = eb.with(
                        //tag the meat as targeted by the killer
                        TargetedForEat {
                            predator: killer,
                            distance: 0.0,
                            predator_pos: Point::new(x, y),
                        },
                        &mut targeted_eats,
                    );
                    meat_entity = eb.build();

                    //tell the killer that the meat is here
                    my_choosen_foods
                        .insert(
                            killer,
                            MyChoosenFood {
                                target: meat_entity,
                            },
                        )
                        .expect("Unable to insert");

                    let target_specie = species.get(entity).unwrap();
                    let killer_specie = species.get(killer).unwrap();
                    general_logs.entries.push(format!(
                        "entity {}, specie {} have been killed by the entity {} specie {} ",
                        entity.id(),
                        target_specie.name,
                        killer.id(),
                        killer_specie.name
                    ));
                }
                DeathCause::Natural => {
                    eb.build();
                }
            }

            //remove the living entiy
            to_deletes
                .insert(entity, ToDelete {})
                .expect("Unable to insert");
        }
        */
    }
}
