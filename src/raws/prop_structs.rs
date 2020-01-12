use serde::{Deserialize};
use super::{Renderable};
use std::collections::HashMap;
use crate::components::*; //todo maybe not ok

#[derive(Deserialize, Debug)]
pub struct Prop {
    pub name : String,
    pub renderable : Option<Renderable>,
    pub hidden : Option<bool>,
    pub blocks_tile : Option<bool>,
    pub blocks_visibility : Option<bool>,
    pub door_open : Option<bool>,
    pub entry_trigger : Option<EntryTrigger>,
    pub interactable : Option<bool>,
    pub interactable_object: Option<InteractableObject>

}

#[derive(Deserialize, Debug)]
pub struct EntryTrigger {
    pub effects : HashMap<String, String>
}