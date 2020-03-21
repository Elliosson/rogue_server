extern crate specs;
use crate::{Equippable, Equipped, InBackpack, WantEquip};
use specs::prelude::*;

pub struct EquipSystem {}

impl<'a> System<'a> for EquipSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantEquip>,
        WriteStorage<'a, Equippable>,
        WriteStorage<'a, Equipped>,
        WriteStorage<'a, InBackpack>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut want_equips, equippables, mut equippeds, mut backpack) = data;

        let mut to_equip = Vec::new();
        let mut to_unequip = Vec::new();

        //get all equipable to equip
        for (entity, want_equip) in (&entities, &mut want_equips).join() {
            if let Some(equippable) = equippables.get(want_equip.target) {
                //desequip celui du slot qui etait deja la
                to_equip.push((entity, want_equip.target, equippable.slot))
            }
        }

        //check if something is already equipped
        for (entity, equipped) in (&entities, &equippeds).join() {
            for (owner, _item_entity, slot) in to_equip.iter() {
                if equipped.owner == *owner && equipped.slot == *slot {
                    //remove
                    to_unequip.push((entity, *owner));
                }
            }
        }

        for (item, owner) in to_unequip {
            equippeds.remove(item);
            backpack
                .insert(item, InBackpack { owner })
                .expect("Unable to insert backpack entry");
        }

        for (owner, item_entity, slot) in to_equip {
            equippeds
                .insert(item_entity, Equipped { owner, slot })
                .expect("Unable to insert equipped component");
            backpack.remove(item_entity);
        }

        want_equips.clear();
    }
}
