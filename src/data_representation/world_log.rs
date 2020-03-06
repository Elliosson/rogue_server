use crate::Date;
extern crate specs;
use specs::prelude::*;
use std::fs::File;
extern crate rltk;
use crate::gamelog;
use std::io::Write;

//Wrote a geneological tree in graphViz format
#[cfg(target_arch = "wasm32")]
pub fn world_state_log(ecs: &mut World) -> std::io::Result<()> {
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn _world_state_log(ecs: &mut World) -> std::io::Result<()> {
    let logs = ecs.fetch_mut::<gamelog::WorldStatLog>();
    let date = ecs.fetch::<Date>();

    let mut file = File::create("world_log.txt")?;

    for log in &logs.entries {
        write!(
            file,
            "day {} year {}: {} \n",
            date.get_day(),
            date.get_year(),
            log
        )?;
    }
    //logs.entries.clear(); //TODO for now continuously write data
    Ok(())
}
