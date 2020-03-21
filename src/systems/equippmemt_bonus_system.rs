extern crate specs;
use crate::{CombatStats, Effect, EquipmentEffect, Equipped};
use specs::prelude::*;

pub struct EquBonusSystem {}

impl<'a> System<'a> for EquBonusSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Equipped>,
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, EquipmentEffect>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut equippeds, mut combat_stats, equipment_effects) = data;

        for (_entity, combat_stat) in (&entities, &mut combat_stats).join() {
            combat_stat.att = combat_stat.base_att;
            combat_stat.defense = combat_stat.base_def;
        }

        for (_entity, equipped, equi_effect) in
            (&entities, &mut equippeds, &equipment_effects).join()
        {
            if let Some(combat_stat) = combat_stats.get_mut(equipped.owner) {
                for effect in &equi_effect.effects {
                    match effect {
                        Effect::Defense(change) => combat_stat.defense += *change,
                        Effect::Melee(change) => combat_stat.att += *change,
                        _ => {}
                    }
                }
            }
        }
    }
}
