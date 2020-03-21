use crate::components;
use serde::Deserialize;
use std::collections::HashMap;
#[derive(Deserialize, Debug)]
pub struct Item {
    pub name: String,
    pub renderable: Option<Renderable>,
    pub consumable: Option<components::Consumable>,
    pub weapon: Option<Weapon>,
    pub shield: Option<Shield>,
}

#[derive(Deserialize, Debug)]
pub struct Renderable {
    pub glyph: String,
    pub fg: String,
    pub bg: String,
    pub order: i32,
}

#[derive(Deserialize, Debug)]
pub enum SexeChoice {
    MaleOnly,
    FemaleOnly,
    MaleStart,   //first entity is a male, but the rest is random
    FemaleStart, //first entity is a female, but the rest is random
    Random,
}

#[derive(Deserialize, Debug)]
pub struct Consumable {
    pub effects: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct Weapon {
    pub range: String,
    pub power_bonus: i32,
}

#[derive(Deserialize, Debug)]
pub struct Shield {
    pub defense_bonus: i32,
}

#[derive(Deserialize, Debug)]
pub struct RawViewshed {
    pub range: i32,
}

#[derive(Deserialize, Debug)]
pub struct RawEnergyReserve {
    pub reserve: f32,
    pub body_energy: f32,
    pub max_reserve: f32,
    pub base_consumption: f32,
}
