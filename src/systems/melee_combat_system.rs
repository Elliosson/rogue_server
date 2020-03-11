extern crate specs;
use crate::{gamelog::GameLog, CombatStats, WantsToMelee};
use specs::prelude::*;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantsToMelee>,
        WriteStorage<'a, CombatStats>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, mut wants_to_melees, mut combat_stats) = data;

        let mut domage_dealed = Vec::new();

        for (entity, wants_to_melee) in (&entities, &wants_to_melees).join() {
            let attacker_stat = combat_stats.get(entity).unwrap();

            domage_dealed.push((attacker_stat.power, wants_to_melee.target));
        }
        for (domage, defender) in domage_dealed.iter() {
            if let Some(defencer_stat) = combat_stats.get_mut(*defender) {
                defencer_stat.hp -= domage;
            } else {
                println!("Error: the defender have no combat stats");
            }
        }
        wants_to_melees.clear()
    }
}
