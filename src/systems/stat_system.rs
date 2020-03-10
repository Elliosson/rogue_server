extern crate specs;
use crate::gamelog::{GameLog, GeneralLog, SpeciesInstantLog, WorldStatLog};
use specs::prelude::*;

pub struct StatSystem {}

impl<'a> System<'a> for StatSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, WorldStatLog>,
        WriteExpect<'a, GeneralLog>,
        WriteExpect<'a, SpeciesInstantLog>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut log, mut wlog, mut glog, mut slog) = data;

        //todo suppress
        //for now i suppress log
        log.entries.clear();
        wlog.entries.clear();
        glog.entries.clear();
        slog.entries.clear();
    }
}
