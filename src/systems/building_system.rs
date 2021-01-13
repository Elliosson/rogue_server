extern crate specs;
use crate::{gamelog::GameLog, ToSpawnList, WantBuild};
use phf::phf_map;
use specs::prelude::*;

static ITEM_TO_BUILDING: phf::Map<&'static str, &'static str> = phf_map! {
    "WallBlock" => "Wall",
};

pub struct BuildingSystem {}

impl<'a> System<'a> for BuildingSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantBuild>,
        WriteExpect<'a, ToSpawnList>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, mut want_builds, mut to_spawns) = data;

        //TODO for now no verification of the right to build
        for (_entity, want_build) in (&entities, &want_builds).join() {
            let build_name = if let Some(name) = ITEM_TO_BUILDING.get(want_build.name.as_str()) {
                name.to_string()
            } else {
                want_build.name.clone()
            };
            to_spawns.request(want_build.x, want_build.y, build_name);
        }
        want_builds.clear()
    }
}
