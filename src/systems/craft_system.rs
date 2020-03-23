extern crate specs;
use crate::{InventaireItem, PlayerInfo, Position, ToDelete, ToSpawnList, WantCraft};
use specs::prelude::*;

use std::collections::HashMap;

pub struct CraftSystem {}

impl<'a> System<'a> for CraftSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantCraft>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, PlayerInfo>,
        WriteExpect<'a, ToSpawnList>,
        WriteStorage<'a, ToDelete>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut want_crafts, mut positions, player_infos, mut to_spawns, mut to_deletes) =
            data;

        //todo this must be deserialised from a json
        //for now this a list a all craftable thing and ressource needed
        //it sould be moved in a json as soon as possible
        let craft_cost: HashMap<String, Vec<(String, i32)>> =
            vec![("WoodenSpear".to_string(), vec![("Wood".to_string(), 2)])]
                .into_iter()
                .collect();

        for (_entity, want_craft, pos, info) in
            (&entities, &mut want_crafts, &mut positions, &player_infos).join()
        {
            if let Some(cost) = craft_cost.get(&want_craft.name) {
                //remove from inventory if everithing is in, else return an error
                match remove_from_inventory(cost, &info.inventaire, &mut to_deletes) {
                    Ok(()) => {
                        //creat item
                        to_spawns.request(pos.x(), pos.y(), want_craft.name.clone());
                    }
                    Err(lacking) => {
                        // log all the lackin component
                        for (name, quantity) in lacking {
                            println!("You lack of {} {}", quantity, name);
                        }
                    }
                }
            }
        }

        want_crafts.clear();
    }
}

//This could probably be reuse
pub fn remove_from_inventory<'a>(
    cost: &Vec<(String, i32)>,
    inventory: &Vec<InventaireItem>,
    to_deletes: &mut WriteStorage<'a, ToDelete>,
) -> Result<(), Vec<(String, i32)>> {
    //convert the inventory into an hashmap
    let mut inventory_hash: HashMap<String, Vec<InventaireItem>> = HashMap::new();
    for item in inventory {
        let entry = inventory_hash
            .entry(item.name.clone())
            .or_insert(Vec::new());
        entry.push(item.clone());
    }

    //check if we lack of ressource
    let mut lacking = Vec::new();

    for (name, quantity) in cost {
        match inventory_hash.get(name) {
            Some(dispo) => {
                let diff = *quantity - dispo.len() as i32;
                if diff <= 0 {
                    //ok
                } else {
                    lacking.push((name.clone(), diff));
                }
            }
            None => {
                lacking.push((name.clone(), *quantity));
            }
        }
    }

    //if ok remove from inventory, else return want is lacking
    let ret = if lacking.len() == 0 {
        //consume the ressource
        for (name, quantity) in cost {
            let ressouces = inventory_hash.get_mut(name).unwrap();
            for _ in 0..*quantity {
                let resource = ressouces.pop().unwrap();
                to_deletes
                    .insert(resource.entity.unwrap(), ToDelete {})
                    .expect("Unable to insert");
            }
        }
        Ok(())
    } else {
        Err(lacking)
    };

    return ret;
}
