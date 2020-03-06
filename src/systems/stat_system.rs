extern crate specs;
use crate::{
    gamelog::{GameLog, SpeciesInstantLog, WorldStatLog},
    Aging, Carnivore, CombatStats, Date, EnergyReserve, Herbivore, HumiditySensitive, Name,
    Renderable, Reproduction, Specie, Speed, TemperatureSensitive,
};
use specs::prelude::*;

pub struct StatSystem {}

impl<'a> System<'a> for StatSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, EnergyReserve>,
        ReadStorage<'a, Reproduction>,
        ReadExpect<'a, Date>,
        WriteExpect<'a, WorldStatLog>,
        ReadStorage<'a, Specie>,
        ReadStorage<'a, TemperatureSensitive>,
        ReadStorage<'a, Renderable>,
        WriteExpect<'a, SpeciesInstantLog>,
        ReadStorage<'a, HumiditySensitive>,
        ReadStorage<'a, Speed>,
        ReadStorage<'a, Herbivore>,
        ReadStorage<'a, Carnivore>,
        ReadStorage<'a, CombatStats>,
        ReadStorage<'a, Aging>,
    );

    fn run(&mut self, _data: Self::SystemData) {}
}
