extern crate rltk;
extern crate serde;
use rltk::{Console, GameState, Point, Rltk};
extern crate specs;
use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};
#[macro_use]
extern crate specs_derive;
mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
mod rect;
pub use rect::Rect;
mod visibility_system;
use visibility_system::VisibilitySystem;
mod map_indexing_system;
use map_indexing_system::MapIndexingSystem;
mod gamelog;
mod gui;
mod inventory_system;
mod spawner;
use inventory_system::{ItemCollectionSystem, ItemDropSystem, ItemRemoveSystem, ItemUseSystem};
mod movement_system;
mod object_deleter;
pub mod random_table;
pub mod raws;
pub mod saveload_system;
use movement_system::MovementSystem;
pub mod ai;
use ai::*;
mod tiletype;
use tiletype::{tile_walkable, TileType};
pub mod systems;
use systems::*;
mod algo;
mod birth;
use birth::{BirthForm, BirthRegistery, BirthRequetList, Mutations};
mod atomic_funtions;
mod data_representation;
//use std::time::Instant;
mod network;
use network::Config;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;
use std::{env, process};
mod local_player;

#[macro_use]
extern crate lazy_static;

rltk::add_wasm_support!();

pub const WINDOWWIDTH: usize = 200;
pub const WINDOWHEIGHT: usize = 120;
pub const MOVE_COST: i32 = 100;
pub const TICK_TIME: time::Duration = time::Duration::from_millis(50);

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    ShowInventory,
    ShowDropItem,
    ShowTargeting {
        range: i32,
        item: Entity,
    },
    MainMenu {
        menu_selection: gui::MainMenuSelection,
    },
    SaveGame,
    NextLevel,
    ShowRemoveItem,
    GameOver,
    ObjectInteraction,
    TemperatureMap,
}

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        //let now = Instant::now();

        let mut date = DateSystem {};
        date.run_now(&self.ecs);
        let mut food_pref = FoodPreferenceSystem {};
        food_pref.run_now(&self.ecs);
        let mut temperature = TemperatureSystem {};
        temperature.run_now(&self.ecs);
        let mut humidity = HumiditySystem {};
        humidity.run_now(&self.ecs);
        let mut temperature_sens = TemperatureSensitivitySystem {};
        temperature_sens.run_now(&self.ecs);
        let mut humidity_sens = HumiditySensitivitySystem {};
        humidity_sens.run_now(&self.ecs);
        let mut specie = SpecieSystem {};
        specie.run_now(&self.ecs);
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);

        /***player turn ****/
        let mut online_player = OnlinePlayerSystem {};
        online_player.run_now(&self.ecs);
        let mut player_command = PlayerCommandSystem {};
        player_command.run_now(&self.ecs);

        //let mut cow = CowAI {};
        //cow.run_now(&self.ecs);
        //let mut carnivore_ai = CarnivorousAI {};
        //carnivore_ai.run_now(&self.ecs);
        let mut eating_killing_ai = EatingKillingAI {};
        eating_killing_ai.run_now(&self.ecs);

        let mut targeting_ai = TargetingAI {};
        targeting_ai.run_now(&self.ecs);

        //let mut omnivore_ai = OmnivoreAI {};
        //omnivore_ai.run_now(&self.ecs);
        let mut flee_ai = FleeAI {};
        flee_ai.run_now(&self.ecs);
        let mut search_partner = SearchParterAI {};
        search_partner.run_now(&self.ecs);
        let mut mapindex = MapIndexingSystem {};
        mapindex.run_now(&self.ecs);

        let mut pickup = ItemCollectionSystem {};
        pickup.run_now(&self.ecs);
        let mut itemuse = ItemUseSystem {};
        itemuse.run_now(&self.ecs);
        let mut drop_items = ItemDropSystem {};
        drop_items.run_now(&self.ecs);
        let mut item_remove = ItemRemoveSystem {};
        item_remove.run_now(&self.ecs);
        let mut object_spawn = ObjectSpawnSystem {};
        object_spawn.run_now(&self.ecs);

        //let mut interaction = InteractionSystem {};
        //interaction.run_now(&self.ecs);
        let mut interactionv2 = Interationv2System {};
        interactionv2.run_now(&self.ecs);

        let mut want_move = WantToMoveSystem {};
        want_move.run_now(&self.ecs);
        let mut go_target = GoTargetSystem {};
        go_target.run_now(&self.ecs);
        let mut movement = MovementSystem {};
        movement.run_now(&self.ecs);
        let mut eating = EatingSystem {};
        eating.run_now(&self.ecs);
        let mut veg_grow = VegetableGrowSystem {};
        veg_grow.run_now(&self.ecs);
        let mut energy = EnergySystem {};
        energy.run_now(&self.ecs);
        //let mut solo_reprod = ReproductionSystem {};
        //solo_reprod.run_now(&self.ecs);
        let mut death_system = DeathSystem {};
        death_system.run_now(&self.ecs);
        let mut gendered_reprod = GenderedReproductionSystem {};
        gendered_reprod.run_now(&self.ecs);
        let mut prop_spawmer = PropSpawnerSystem {};
        prop_spawmer.run_now(&self.ecs);
        let mut aging = AgingSystem {};
        aging.run_now(&self.ecs);
        let mut named_counter = NamedCounterSystem {};
        named_counter.run_now(&self.ecs);
        let mut action_point = ActionPointSystem {};
        action_point.run_now(&self.ecs);
        let mut stat = StatSystem {};
        stat.run_now(&self.ecs);
        let mut map_send = SendMapSystem {};
        map_send.run_now(&self.ecs);

        self.ecs.maintain();
        // println!("systems time = {}", now.elapsed().as_micros());
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        let start = time::Instant::now();

        ctx.cls();

        draw_map(&self.ecs, ctx);

        {
            let positions = self.ecs.read_storage::<Position>();
            let renderables = self.ecs.read_storage::<Renderable>();

            let mut data = (&positions, &renderables).join().collect::<Vec<_>>();
            data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));
            for (pos, render) in data.iter() {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }

            gui::draw_ui(&self.ecs, ctx);
        }

        //handle input of the local player
        local_player::local_player_input(&self.ecs, ctx);

        //run game
        self.run_systems();
        self.ecs.maintain();

        object_deleter::delete_entity_to_delete(&mut self.ecs);

        let end = time::Instant::now();

        let time_spend = end - start;

        if TICK_TIME > time_spend {
            let time_left = TICK_TIME - time_spend;
            thread::sleep(time_left);
        } else {
            println!("WARNING: tick is too slow !");
            println!("{:?}", time_spend);
        }
    }
}

impl State {}

fn main() {
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();
    gs.ecs.register::<Item>();
    gs.ecs.register::<ProvidesHealing>();
    gs.ecs.register::<InflictsDamage>();
    gs.ecs.register::<AreaOfEffect>();
    gs.ecs.register::<Consumable>();
    gs.ecs.register::<Ranged>();
    gs.ecs.register::<InBackpack>();
    gs.ecs.register::<WantsToPickupItem>();
    gs.ecs.register::<WantsToUseItem>();
    gs.ecs.register::<WantsToDropItem>();
    gs.ecs.register::<Confusion>();
    gs.ecs.register::<SimpleMarker<SerializeMe>>();
    gs.ecs.register::<SerializationHelper>();
    gs.ecs.register::<Equippable>();
    gs.ecs.register::<Equipped>();
    gs.ecs.register::<MeleePowerBonus>();
    gs.ecs.register::<DefenseBonus>();
    gs.ecs.register::<WantsToRemoveItem>();
    gs.ecs.register::<Interactable>();
    gs.ecs.register::<InteractableObject>();
    gs.ecs.register::<WantsToInteract>();
    gs.ecs.register::<Interaction>();
    gs.ecs.register::<ToDelete>();
    gs.ecs.register::<Herbivore>();
    gs.ecs.register::<Leaf>();
    gs.ecs.register::<WantToEat>();
    gs.ecs.register::<ApplyMove>();
    gs.ecs.register::<ApplyTeleport>();
    gs.ecs.register::<MyTurn>();
    gs.ecs.register::<MoveMode>();
    gs.ecs.register::<EntityMoved>();
    gs.ecs.register::<EntryTrigger>();
    gs.ecs.register::<Tree>();
    gs.ecs.register::<EnergyReserve>();
    gs.ecs.register::<Reproduction>();
    gs.ecs.register::<WantsToDuplicate>();
    gs.ecs.register::<UniqueId>();
    gs.ecs.register::<Aging>();
    gs.ecs.register::<InUse>();
    gs.ecs.register::<TemperatureSensitive>();
    gs.ecs.register::<Specie>();
    gs.ecs.register::<HumiditySensitive>();
    gs.ecs.register::<Speed>();
    gs.ecs.register::<GoOnTarget>();
    gs.ecs.register::<TargetReached>();
    gs.ecs.register::<Carnivore>();
    gs.ecs.register::<WantsToFlee>();
    gs.ecs.register::<TargetedForEat>();
    gs.ecs.register::<Animal>();
    gs.ecs.register::<Male>();
    gs.ecs.register::<Female>();
    gs.ecs.register::<MyChoosenFood>();
    gs.ecs.register::<InHeat>();
    gs.ecs.register::<Meat>();
    gs.ecs.register::<Dead>();
    gs.ecs.register::<FoodPreference>();
    gs.ecs.register::<WantToMove>();
    gs.ecs.register::<PlayerInputComp>();
    gs.ecs.register::<OnlinePlayer>();
    gs.ecs.register::<Connected>();

    gs.ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());

    raws::load_raws();

    let map: Map = Map::new_map();
    let (player_x, player_y) = map.rooms[0].center();

    let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);

    gs.ecs.insert(rltk::RandomNumberGenerator::new());
    for room in map.rooms.iter() {
        spawner::spawn_trees(&mut gs.ecs, room);
    }

    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::AwaitingInput);
    gs.ecs.insert(gamelog::GameLog {
        entries: vec!["Welcome to Rusty Roguelike".to_string()],
    });
    gs.ecs.insert(ObjectBuilder::new());
    gs.ecs.insert(InteractionResquest::new());
    gs.ecs.insert(Date::new());
    gs.ecs.insert(BirthRequetList::new());
    gs.ecs.insert(BirthRegistery::new());
    gs.ecs.insert(UuidPlayerHash::new());
    gs.ecs.insert(PlayerMessages::new());
    gs.ecs.insert(LocalClientInfo::new());
    gs.ecs.insert(InteractionResquestListV2::new());
    gs.ecs.insert(gamelog::WorldStatLog {
        entries: vec!["Rust Roguelike World Stat log file".to_string()],
    });
    gs.ecs.insert(gamelog::GeneralLog {
        entries: vec!["Rust Roguelike General log file".to_string()],
    });
    gs.ecs
        .insert(gamelog::SpeciesInstantLog { entries: vec![] });

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error creating Config: {}", err);
        println!("Usage: poeng_server url");
        process::exit(1);
    });

    println!("url: {}", config.url);
    let message_list: Arc<Mutex<Vec<(network::Message, String)>>> =
        Arc::new(Mutex::new(Vec::new()));

    let map_to_send: Arc<Mutex<HashMap<String, Vec<(Position, Renderable)>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    gs.ecs.insert(message_list.clone());

    gs.ecs.insert(map_to_send.clone());

    thread::spawn(move || {
        network::run(config, message_list, map_to_send);
    });

    game_loop(gs);

    //rltk::main_loop(context, gs);
}

#[cfg(not(feature = "no_rltk"))]
fn game_loop(mut gs: State) {
    let mut context = Rltk::init_simple8x8(
        WINDOWWIDTH as u32,
        WINDOWHEIGHT as u32,
        "Ecosystem simulator",
        "resources",
    );
    context.with_post_scanlines(true);

    rltk::main_loop(context, gs);
}

#[cfg(feature = "no_rltk")]
fn game_loop(mut gs: State) {
    loop {
        let start = time::Instant::now();

        //run game
        gs.run_systems();
        gs.ecs.maintain();

        object_deleter::delete_entity_to_delete(&mut gs.ecs);

        let end = time::Instant::now();

        let time_spend = end - start;

        if TICK_TIME > time_spend {
            let time_left = TICK_TIME - time_spend;
            thread::sleep(time_left);
        } else {
            println!("WARNING: tick is too slow !");
        }
    }
}
