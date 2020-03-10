extern crate specs;
use crate::{
    gamelog::{GameLog, WorldStatLog},
    Name,
};
use specs::prelude::*;

pub struct NamedCounterSystem {}

impl<'a> System<'a> for NamedCounterSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Name>,
        WriteExpect<'a, WorldStatLog>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, names, mut world_logs) = data;

        let mut names_stats: Vec<(String, i32)> = Vec::new();

        //Count the number on entity by name
        //Very ugly, maybe it's would be more beautifull in a more fonctionnal approach
        for (_entity, name) in (&entities, &names).join() {
            let mut done = false;

            for (s_name, s_count) in &mut names_stats.iter_mut() {
                if *s_name == name.name {
                    *s_count += 1;
                    done = true;
                    break;
                }
            }
            if done == false {
                names_stats.push((name.name.clone(), 1))
            }
        }

        for (name, count) in names_stats.iter() {
            let buf = format!("There is {} {}", count, name);
            //println!("There is {} {}", count, name);
            world_logs.entries.push(buf);
        }
    }
}
