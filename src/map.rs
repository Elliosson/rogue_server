extern crate rltk;
use super::Rect;
use rltk::{Algorithm2D, BaseMap, Console, Point, Rltk, RGB};

extern crate specs;
use crate::TileType;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use std::collections::HashMap;

pub const MAPWIDTH: usize = 1000;
pub const MAPHEIGHT: usize = 1000;
pub const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Map {
    pub tiles: HashMap<usize, TileType>, //tile handle the apparence of the soil and the opacity, desactived for now
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: HashMap<usize, bool>, //visible and revelated is a survivance of a solo player game, devactivated for now
    pub visible_tiles: HashMap<usize, bool>,
    pub blocked: HashMap<usize, bool>,
    pub depth: i32,
    //pub dirty: Vec<usize>, //a vec of the index that must be checked by map indexing
    pub dirty: Vec<(i32, i32)>, //a vec of the x, y that must be checked by map indexing

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub tile_content: HashMap<usize, Vec<Entity>>,
    pub tile_temperature: HashMap<usize, f32>,
    pub tile_humidity: HashMap<usize, f32>,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn is_blocked(&self, idx: usize) -> bool {
        if let Some(blocked) = self.blocked.get(&idx) {
            if *blocked {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    pub fn set_blocked(&mut self, idx: usize, is_blocked: bool) {
        let blocked = self.blocked.entry(idx).or_insert(is_blocked);
        *blocked = is_blocked;
    }

    pub fn idx_xy(&self, idx: usize) -> (i32, i32) {
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        (x, y)
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let _idx = self.xy_idx(x, y);
                //self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }
        let idx = self.xy_idx(x, y);
        !self.is_blocked(idx)
    }

    //add a wall on a boquer tile to render hime opaque
    /*pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter().enumerate() {
            self.set_blocked(i, *tile == TileType::Wall);
        }
    }*/

    pub fn clear_content_index(&mut self) {
        self.tile_content.clear()
    }

    /// Make a simple new map
    pub fn new_map() -> Map {
        let mut map = Map {
            tiles: HashMap::new(),
            rooms: Vec::new(),
            width: MAPWIDTH as i32,
            height: MAPHEIGHT as i32,
            revealed_tiles: HashMap::new(),
            visible_tiles: HashMap::new(),
            blocked: HashMap::new(),
            tile_content: HashMap::new(),
            tile_temperature: HashMap::new(),
            tile_humidity: HashMap::new(),
            depth: 0,
            dirty: Vec::new(),
        };

        let new_room = Rect::new(1, 1, map.width - 4, map.height - 4);

        map.apply_room_to_map(&new_room);
        map.rooms.push(new_room);

        map
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: i32) -> bool {
        //self.tiles[idx as usize] == TileType::Wall
        //TODO, to implement
        false
    }

    fn get_available_exits(&self, idx: i32) -> Vec<(i32, f32)> {
        let mut exits: Vec<(i32, f32)> = Vec::new();
        let x = idx % self.width;
        let y = idx / self.width;

        // Cardinal directions
        if self.is_exit_valid(x - 1, y) {
            exits.push((idx - 1, 1.0))
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((idx + 1, 1.0))
        };
        if self.is_exit_valid(x, y - 1) {
            exits.push((idx - self.width, 1.0))
        };
        if self.is_exit_valid(x, y + 1) {
            exits.push((idx + self.width, 1.0))
        };

        // Diagonals
        if self.is_exit_valid(x - 1, y - 1) {
            exits.push(((idx - self.width) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y - 1) {
            exits.push(((idx - self.width) + 1, 1.45));
        }
        if self.is_exit_valid(x - 1, y + 1) {
            exits.push(((idx + self.width) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y + 1) {
            exits.push(((idx + self.width) + 1, 1.45));
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: i32, idx2: i32) -> f32 {
        let p1 = Point::new(idx1 % self.width, idx1 / self.width);
        let p2 = Point::new(idx2 % self.width, idx2 / self.width);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

impl Algorithm2D for Map {
    fn in_bounds(&self, pos: Point) -> bool {
        pos.x > 0 && pos.x < self.width - 1 && pos.y > 0 && pos.y < self.height - 1
    }

    fn point2d_to_index(&self, pt: Point) -> i32 {
        (pt.y * self.width) + pt.x
    }

    fn index_to_point2d(&self, idx: i32) -> Point {
        Point {
            x: idx % self.width,
            y: idx / self.width,
        }
    }
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;
    for (_idx, tile) in map.tiles.iter() {
        // Render a tile depending upon the tile type

        let glyph;
        let fg;
        match tile {
            TileType::Floor => {
                glyph = rltk::to_cp437('.');
                fg = RGB::from_f32(0.0, 0.5, 0.5);
            }
            TileType::Wall => {
                glyph = rltk::to_cp437('#');
                fg = RGB::from_f32(0., 1.0, 0.);
            }
            TileType::DownStairs => {
                glyph = rltk::to_cp437('>');
                fg = RGB::from_f32(0., 1.0, 1.0);
            }
            _ => {
                glyph = rltk::to_cp437('?');
                fg = RGB::from_f32(0., 1.0, 1.0);
            }
        }
        ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);

        // Move the coordinates
        x += 1;
        if x > MAPWIDTH as i32 - 1 {
            x = 0;
            y += 1;
        }
    }
}
