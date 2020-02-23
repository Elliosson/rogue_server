extern crate rltk;
use rltk::{Point, Rltk, VirtualKeyCode};
extern crate specs;
use super::{
    gamelog::GameLog, network, CombatStats, Item, Map, Monster, Player, Position, RunState, State,
    TileType, Viewshed, WantsToMelee, WantsToPickupItem, WINDOWHEIGHT, WINDOWWIDTH,
};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World, entity: Entity) {
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.read_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();

    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();

    let mut pos = positions.get_mut(entity).unwrap();
    let mut viewshed = viewsheds.get_mut(entity).unwrap();

    if pos.x + delta_x < 1
        || pos.x + delta_x > map.width - 1
        || pos.y + delta_y < 1
        || pos.y + delta_y > map.height - 1
    {
        return;
    }
    let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

    for potential_target in map.tile_content[destination_idx].iter() {
        let target = combat_stats.get(*potential_target);
        if let Some(_target) = target {
            wants_to_melee
                .insert(
                    entity,
                    WantsToMelee {
                        target: *potential_target,
                    },
                )
                .expect("Add target failed");
            return;
        }
    }

    if !map.blocked[destination_idx] {
        pos.x = min(WINDOWWIDTH as i32 - 1, max(0, pos.x + delta_x));
        pos.y = min(WINDOWHEIGHT as i32 - 1, max(0, pos.y + delta_y));

        viewshed.dirty = true;
        let mut ppos = ecs.write_resource::<Point>();
        ppos.x = pos.x;
        ppos.y = pos.y;
    }
}

pub fn try_next_level(ecs: &mut World) -> bool {
    let player_pos = ecs.fetch::<Point>();
    let map = ecs.fetch::<Map>();
    let player_idx = map.xy_idx(player_pos.x, player_pos.y);
    if map.tiles[player_idx] == TileType::DownStairs {
        true
    } else {
        let mut gamelog = ecs.fetch_mut::<GameLog>();
        gamelog
            .entries
            .insert(0, "There is no way down from here.".to_string());
        false
    }
}

fn get_item(ecs: &mut World) {
    let player_pos = ecs.fetch::<Point>();
    let player_entity = ecs.fetch::<Entity>();
    let entities = ecs.entities();
    let items = ecs.read_storage::<Item>();
    let positions = ecs.read_storage::<Position>();
    let mut gamelog = ecs.fetch_mut::<GameLog>();

    let mut target_item: Option<Entity> = None;
    for (item_entity, _item, position) in (&entities, &items, &positions).join() {
        if position.x == player_pos.x && position.y == player_pos.y {
            target_item = Some(item_entity);
        }
    }

    match target_item {
        None => gamelog
            .entries
            .insert(0, "There is nothing here to pick up.".to_string()),
        Some(item) => {
            let mut pickup = ecs.write_storage::<WantsToPickupItem>();
            pickup
                .insert(
                    *player_entity,
                    WantsToPickupItem {
                        collected_by: *player_entity,
                        item,
                    },
                )
                .expect("Unable to insert want to pickup");
        }
    }
}

fn skip_turn(ecs: &mut World) -> RunState {
    let player_entity = ecs.fetch::<Entity>();
    let viewshed_components = ecs.read_storage::<Viewshed>();
    let monsters = ecs.read_storage::<Monster>();

    let worldmap_resource = ecs.fetch::<Map>();

    let mut can_heal = true;
    let viewshed = viewshed_components.get(*player_entity).unwrap();
    for tile in viewshed.visible_tiles.iter() {
        let idx = worldmap_resource.xy_idx(tile.x, tile.y);
        for entity_id in worldmap_resource.tile_content[idx].iter() {
            let mob = monsters.get(*entity_id);
            match mob {
                None => {}
                Some(_) => {
                    can_heal = false;
                }
            }
        }
    }

    if can_heal {
        let mut health_components = ecs.write_storage::<CombatStats>();
        let player_hp = health_components.get_mut(*player_entity).unwrap();
        player_hp.hp = i32::min(player_hp.hp + 1, player_hp.max_hp);
    }

    RunState::PlayerTurn
}
