extern crate specs;
use crate::{gamelog::GameLog, Connected, OnlinePlayer, PlayerInfo};
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use std::fs::File;
use std::io::Write;

//serialize PlayerInfo in a json
pub struct PlayerJsonSystem {}

impl<'a> System<'a> for PlayerJsonSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, PlayerInfo>,
        ReadStorage<'a, Connected>,
        ReadStorage<'a, OnlinePlayer>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, player_infos, _connecteds, online_players) = data;

        //todo check in player is connected and find a way to handle local player
        for (_entity, player_info, _online_player) in
            (&entities, &player_infos, &online_players).join()
        {
            let player_info_string = serde_json::to_string(&player_info).unwrap();

            //test by writing in a real file
            player_json(player_info_string).unwrap();
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn player_json(player_inf: String) -> std::io::Result<()> {
    let mut file = File::create("player_info.txt")?;

    write!(file, "{}", player_inf)?;

    Ok(())
}
