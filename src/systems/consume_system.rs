extern crate specs;
use crate::{gamelog::GameLog, CombatStats, Consumable, Effect, ToDelete, WantConsume};
use specs::prelude::*;

pub struct ConsumeSystem {}

impl<'a> System<'a> for ConsumeSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantConsume>,
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, Consumable>,
        WriteStorage<'a, ToDelete>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, mut want_consumes, mut combat_stats, consumables, mut to_deletes) =
            data;

        for (_entity, want_consume, combat_stat) in
            (&entities, &mut want_consumes, &mut combat_stats).join()
        {
            //for now I think it's ok to do everything in this system

            // if truc est consumable
            if let Some(consumable) = consumables.get(want_consume.target) {
                for effect in consumable.effects.iter() {
                    match effect {
                        Effect::Heal(hp_regain) => {
                            combat_stat.change_hp(*hp_regain);
                        }
                    }
                }
                to_deletes
                    .insert(want_consume.target, ToDelete {})
                    .expect("Unable to insert");
            }
        }
    }
}
