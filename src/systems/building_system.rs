extern crate specs;
use crate::{gamelog::GameLog, ToSpawnList, WantBuild};
use specs::prelude::*;

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
            to_spawns.request(want_build.x, want_build.y, want_build.name.clone());
        }
        want_builds.clear()
    }
}
