use crate::BirthRegistery;
extern crate specs;
use specs::prelude::*;
use std::fs::File;
extern crate rltk;
use std::io::Write;

//Wrote a geneological tree in graphViz format
#[cfg(target_arch = "wasm32")]
pub fn write_genealogy(ecs: &mut World) -> std::io::Result<()> {
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn _write_genealogy(ecs: &mut World) -> std::io::Result<()> {
    let registery = ecs.fetch::<BirthRegistery>();

    let mut file = File::create("birth_registery.dot")?;

    write!(file, "digraph G {{\n")?;

    for certif in registery.get().iter() {
        /*
        println!("{} -> {}", certif.parents.id(), certif.entity.id());
        println!("{} [label=\"{}]", certif.entity.id(), certif.name.name);
        println!("{} [label=\"{}]", certif.parents.id(), certif.name.name);
        */
        write!(file, "    {} -> {};\n", certif.parent_id, certif.id)?;
        if let Some(male_id) = certif.male_parent_id {
            write!(file, "    {} -> {};\n", male_id, certif.id)?;
        }
        /*
        write!(
            file,
            "   {} [label=\"{}];\n",
            certif.entity.id(),
            certif.name.name
        )?;
        write!(
            file,
            "   {} [label=\"{}];\n",
            certif.parents.id(),
            certif.name.name
        )?;*/
    }
    write!(file, "}}\n")?;

    Ok(())
}
