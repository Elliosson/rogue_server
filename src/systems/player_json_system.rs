extern crate specs;
use crate::{gamelog::GameLog, Connected, OnlinePlayer, PlayerInfo};
use specs::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
//serialize PlayerInfo in a json
pub struct PlayerJsonSystem {}

//create a json to be send on the client
impl<'a> System<'a> for PlayerJsonSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, PlayerInfo>,
        ReadStorage<'a, Connected>,
        ReadStorage<'a, OnlinePlayer>,
        WriteExpect<'a, Arc<Mutex<HashMap<String, String>>>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, player_infos, connecteds, online_players, send_player_info) = data;

        let mut player_info_guard = send_player_info.lock().unwrap();
        player_info_guard.clear();

        //todo check in player is connected and find a way to handle local player
        for (_entity, player_info, _online_player, connected) in
            (&entities, &player_infos, &online_players, &connecteds).join()
        {
            let player_info_string = serde_json::to_string(&player_info).unwrap();

            let my_uiid = connected.uuid.clone();

            player_info_guard.insert(my_uiid, player_info_string);
            //test by writing in a real file
            //player_json(player_info_string).unwrap();
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn _player_json(player_inf: String) -> std::io::Result<()> {
    let mut file = File::create("player_info.txt")?;

    write!(file, "{}", player_inf)?;

    Ok(())
}
