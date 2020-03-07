extern crate specs;
use super::{BlocksTile, Map, Position};
use specs::prelude::*;
use std::collections::HashSet;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, positions, blockers, _entities) = data;

        //hashset prevent duplicate
        let mut dirty_entities: HashSet<Entity> = HashSet::new();
        let dirty = map.dirty.clone();
        //reconvert all entity that were in a dirty tile
        for (x, y) in dirty.iter() {
            let idx = map.xy_idx(*x, *y);

            map.blocked.remove(&idx);

            if let Some(in_tile) = map.tile_content.remove(&idx) {
                for entity in in_tile.iter() {
                    dirty_entities.insert(*entity);
                }
            }
        }
        //store all the dirty entity in there corect tile, and set blocked map
        for entity in dirty_entities.drain() {
            if let Some(pos) = positions.get(entity) {
                let idx = map.xy_idx(pos.x(), pos.y());
                let tile_content = map.tile_content.entry(idx).or_insert(Vec::new());
                tile_content.push(entity);
                if let Some(_block) = blockers.get(entity) {
                    map.set_blocked(idx, true);
                }
            }
        }
        map.dirty.clear();
    }
}

//options
/*1. on stoque toute les tile ou il y a eu des modif
    pro: les entity supprimé sont facilement géré
    con: on risque de plusieur fois traité la meme Entity
        je stocke toute les entity a update , je les trie, je retire les duplicate -> lourd
    con les entity rique d'etre stoqué plusieur fois dans le vec! -> voir presedent
    con: on va devoir passé la map l'ors des modif de pos
    con: dificult to index new entity


2 on marque tout les enity qui on eut des modif
    pro: on ne parcour les entity que un fois
    con: compliqué pour la destruction d'entity
    con: dificil de s'assurer que l'entity a bien été retirer des la position presedente
    con: ono est pas sur que l'on a insérer le move lo'rs d'un deplacement


*/
