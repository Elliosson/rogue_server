extern crate specs;
use crate::{gamelog::GameLog, CombatStats, Faction, Name, PlayerLog, WantsToMelee};
use specs::prelude::*;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantsToMelee>,
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, PlayerLog>,
        WriteStorage<'a, Name>,
        WriteStorage<'a, Faction>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            _log,
            mut wants_to_melees,
            mut combat_stats,
            mut player_logs,
            names,
            factions,
        ) = data;

        let mut domage_dealed = Vec::new();

        for (entity, wants_to_melee) in (&entities, &wants_to_melees).join() {
            let attacker_stat = combat_stats.get(entity).unwrap();

            domage_dealed.push((attacker_stat.att, wants_to_melee.target, entity));
        }
        for (damage, defender, attacker) in domage_dealed {
            if enemy_faction(attacker, defender, &factions) {
                if let Some(defencer_stat) = combat_stats.get_mut(defender) {
                    defencer_stat.hp -= damage;
                    let def_name = names.get(defender).unwrap();
                    let att_name = names.get(attacker).unwrap();
                    //log the fight
                    if let Some(log) = player_logs.get_mut(attacker) {
                        log.set_logs(format!("You attack {} for {} hp.", def_name.name, damage));
                    }
                    if let Some(log) = player_logs.get_mut(defender) {
                        log.set_logs(format!(
                            "You receveid {} dommage from {}",
                            damage, att_name.name
                        ));
                    }
                } else {
                    println!("Error: the defender have no combat stats");
                }
            } else {
                if let Some(log) = player_logs.get_mut(attacker) {
                    log.set_logs(format!("You are trying to attack a friend"));
                }
            }
        }
        wants_to_melees.clear()
    }
}

pub fn enemy_faction<'a>(
    attacker: Entity,
    defender: Entity,
    factions: &WriteStorage<'a, Faction>,
) -> bool {
    let mut enemy = true;
    if let Some(att_fac) = factions.get(attacker) {
        if let Some(def_fac) = factions.get(defender) {
            if def_fac.name == att_fac.name {
                //can't attack
                enemy = false;
            }
        }
    }
    return enemy;
}
