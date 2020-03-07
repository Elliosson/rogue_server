extern crate specs;
use specs::prelude::*;
extern crate rltk;
extern crate specs_derive;
use super::MOVE_COST;
use rltk::RGB;
use serde::{Deserialize, Serialize};
use specs::error::NoError;
use specs::saveload::{ConvertSaveload, Marker};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn moving(&mut self, x: i32, y: i32, dirty: &mut Vec<(i32, i32)>) {
        dirty.push((self.x, self.y));
        dirty.push((x, y));
        self.x = x;
        self.y = y;
    }
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn new(x: i32, y: i32, dirty: &mut Vec<(i32, i32)>) -> Self {
        dirty.push((x, y));
        Position { x, y }
    }
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Player {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Monster {}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Name {
    pub name: String,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct BlocksTile {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Connected {
    pub uuid: String,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToMelee {
    pub target: Entity,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct SufferDamage {
    pub amount: i32,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Item {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Interactable {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct InteractableObject {
    pub interactions: Vec<Interaction>,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Interaction {
    pub name: String,
}

impl Interaction {
    pub fn new(name: String) -> Interaction {
        Interaction { name: name }
    }
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Consumable {}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Ranged {
    pub range: i32,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct InflictsDamage {
    pub damage: i32,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct AreaOfEffect {
    pub radius: i32,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Confusion {
    pub turns: i32,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct ProvidesHealing {
    pub heal_amount: i32,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct InBackpack {
    pub owner: Entity,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToPickupItem {
    pub collected_by: Entity,
    pub item: Entity,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToInteract {
    pub interacted_by: Entity,
    pub interacted: Entity,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToUseItem {
    pub item: Entity,
    pub target: Option<rltk::Point>,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToDropItem {
    pub item: Entity,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToRemoveItem {
    pub item: Entity,
}

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum EquipmentSlot {
    Melee,
    Shield,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Equippable {
    pub slot: EquipmentSlot,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Equipped {
    pub owner: Entity,
    pub slot: EquipmentSlot,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct MeleePowerBonus {
    pub power: i32,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct DefenseBonus {
    pub defense: i32,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct ToDelete {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Herbivore {
    pub digestion: f32,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Leaf {
    pub nutriments: i32,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantToEat {
    pub target: Entity,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct ApplyMove {
    pub dest_idx: i32,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct MyTurn {}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub enum Movement {
    Static,
    Random,
    RandomWaypoint { path: Option<Vec<i32>> },
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct MoveMode {
    pub mode: Movement,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct EntityMoved {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct ApplyTeleport {
    pub dest_x: i32,
    pub dest_y: i32,
    pub dest_depth: i32,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct EntryTrigger {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Tree {}

#[derive(PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Hunger {
    Full,
    Hungry,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct EnergyReserve {
    pub reserve: f32,
    pub body_energy: f32,
    pub max_reserve: f32, //TODO max reserve never checked. for now just triger hunger
    pub base_consumption: f32,
    pub hunger: Hunger,
}

impl EnergyReserve {
    pub fn get_relative_reserve(&self) -> f32 {
        self.reserve / self.max_reserve
    }
    pub fn get_eating_gain(&self) -> f32 {
        self.reserve + self.body_energy
    }
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Reproduction {
    //energy of the new born
    pub birth_energy: u32,
    //offset of energy used by the birth
    pub offset_cost: u32,
    //amoount of energy that the creature must have left  after reproduction
    pub offset_threshold: u32,
}

impl Reproduction {
    //Minimum of energy to have to reproduce
    pub fn threshold(&self) -> u32 {
        self.birth_energy + self.offset_cost + self.offset_threshold
    }

    //cost of giving birth
    pub fn cost(&self) -> u32 {
        self.birth_energy + self.offset_cost
    }
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct WantsToDuplicate {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct UniqueId {
    _id: usize,
}

impl UniqueId {
    pub fn get(&self) -> usize {
        self._id
    }

    //generate an unique id Id
    pub fn new() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        UniqueId {
            _id: COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Aging {
    pub life_expectancy: i32,
    pub age: i32,
}

//Mark the component as in use for this turn to prevent other to use it
//Must delete at the end of each turn
#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct InUse {}

//optimum of temperature and  plage of acceptebality
#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct TemperatureSensitive {
    pub optimum: f32,
    pub k: f32, //ecrasement de la gaussiene ou carré
}

//optimum of humidity plage and of acceptebality
#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct HumiditySensitive {
    pub optimum: f32,
    pub k: f32, //ecrasement de la gaussiene ou carré
}

//if a a star search must be small or with a lot of iteration
//TODO faire un truc pour carlculer le chemin selon la distance
#[derive(PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum SearchScope {
    Small,
    Big,
}

//Componant to set the target, with it pathfind and move will be made at the same time
#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct GoOnTarget {
    pub target: Entity,
    pub scope: SearchScope,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Speed {
    pub move_point: i32,
    pub point_per_turn: i32,
    pub base_point_per_turn: i32,
    pub max_point: i32,
}

impl Speed {
    pub fn add_move_point(&mut self, new_point: i32) {
        if (self.move_point + new_point) > self.max_point {
            self.move_point = self.max_point;
        } else {
            self.move_point += new_point;
        }
    }
    pub fn speed(&self) -> f32 {
        self.point_per_turn as f32 / MOVE_COST as f32
    }
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct TargetReached {
    pub target: Entity,
}

//mark the specie of the creature
#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Specie {
    pub name: String,
    pub id: i32,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Carnivore {
    pub digestion: f32, //TODO, limite between 0 and 1 ?
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct WantsToFlee {
    pub indices: Vec<i32>,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct TargetedForEat {
    pub predator: Entity,
    pub distance: f32,
    pub predator_pos: rltk::Point,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Animal {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Male {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Female {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct InHeat {}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct MyChoosenFood {
    pub target: Entity,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub enum DeathCause {
    Natural,
    Killed { killer: Entity },
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Dead {
    pub cause: DeathCause,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Meat {
    pub nutriments: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FoodType {
    Meat,
    Animal,
    Vegetable,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct FoodPreference {
    //associate a level of hunger with a food type
    pub choices: BTreeMap<i32, FoodType>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OnlineRunState {
    AwaitingInput,
    PlayerTurn,
}

#[derive(Debug, ConvertSaveload, Clone)]
pub enum PlayerInput {
    NONE,
    UP,
    DOWN,
    LEFT,
    RIGHT,
    INVENTORY,
    PICKUP(Entity),
    INTERACT(i32, i32, String, Entity),
    BUILD(i32, i32, String),
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct OnlinePlayer {
    pub runstate: OnlineRunState,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct PlayerInputComp {
    pub input: PlayerInput,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct WantToMove {
    pub delta_x: i32,
    pub delta_y: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LocalClientRunstate {
    BaseState,
    Inventory,
    Interaction,
    Build,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct LocalClientInfo {
    pub local_runstate: LocalClientRunstate,
}

impl LocalClientInfo {
    pub fn new() -> Self {
        LocalClientInfo {
            local_runstate: LocalClientRunstate::BaseState,
        }
    }
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct BuildingChoice {
    pub plans: Vec<String>,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct WantBuild {
    pub x: i32,
    pub y: i32,
    pub name: String,
}

//Special component for network, Do NOT serialize, it's could go badly
#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerInfo {
    pub inventaire: Vec<InventaireItem>,
    pub close_interations: Vec<CloseInteration>,
    pub my_info: MyInfo,
    pub possible_builds: Vec<BuildingPlan>,
}

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct BuildingPlan {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MyInfo {
    pub pos: Position,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InventaireItem {
    pub name: String,
    pub index: u32,
    pub generation: i32,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub entity: Option<Entity>, //entity do not implement default, so I use option to be able to skip serialization, it suck
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CloseInteration {
    pub interaction_name: String,
    pub object_name: String,
    pub index: u32,
    pub generation: i32,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub entity: Option<Entity>, //entity do not implement default, so I use option to be able to skip serialization, it suck
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct EntityToConvert {
    pub command: CommandToConvert,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CommandToConvert {
    INVENTORY,
    PICKUP(u32, i32),
    INTERACT(i32, i32, String, u32, i32),
    BUILD(i32, i32, String),
}

// Serialization helper code. We need to implement ConvertSaveLoad for each type that contains an
// Entity.
pub struct SerializeMe;

// Special component that exists to help serialize the game data
#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SerializationHelper {
    pub map: super::map::Map,
}
