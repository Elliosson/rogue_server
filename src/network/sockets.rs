use super::message::Message;
use crate::{Position, Renderable};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub struct Config {
    pub url: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let url = args[1].clone();

        Ok(Config { url })
    }
}

pub fn run(
    config: Config,
    message_list: Arc<Mutex<Vec<(Message, String)>>>,
    map_to_send: Arc<Mutex<HashMap<String, Vec<(Position, Renderable)>>>>,
) {
    ws::listen(config.url, move |out| {
        let message_list = message_list.clone();
        let map_to_send_clone = map_to_send.clone();

        move |msg: ws::Message| {
            let message_list = message_list.clone();
            let mut message_guard = message_list.lock().unwrap();

            let msg = match msg.as_text() {
                Ok(msg) => msg,
                Err(_) => "",
            };
            println!("message: {}", msg);

            //push the message in a struct that will be read by the game to do an action
            match Message::from(msg) {
                Some((msg, command)) => {
                    println!("message: {:?}", msg);
                    message_guard.push((msg, command));
                }
                None => {
                    println!("None message");
                }
            };

            out.send(ws::Message::Text(match Message::from(msg) {
                Some((msg, command)) => {
                    format!("{} {}", command, response(msg, map_to_send_clone.clone()))
                }
                None => "err".to_string(),
            }))
        }
    })
    .unwrap();
}

fn response(
    msg: Message,
    map_to_send: Arc<Mutex<HashMap<String, Vec<(Position, Renderable)>>>>,
) -> String {
    let map_guard = map_to_send.lock().unwrap();

    match msg {
        Message::Register => {
            println!("register");
            Uuid::new_v4().to_string()
        }
        Message::Map(uuid) => {
            let mut string_to_send = " ".to_string();
            if let Some(my_map) = map_guard.get(&uuid.to_string()) {
                for (pos, renderable) in my_map.iter() {
                    string_to_send = format!(
                        "{} {} {} {} {} {} {} {} {} {} {}",
                        string_to_send,
                        pos.x,
                        pos.y,
                        renderable.glyph,
                        (renderable.fg.r * 255.0) as u8,
                        (renderable.fg.g * 255.0) as u8,
                        (renderable.fg.b * 255.0) as u8,
                        (renderable.bg.r * 255.0) as u8,
                        (renderable.bg.g * 255.0) as u8,
                        (renderable.bg.b * 255.0) as u8,
                        renderable.render_order
                    );
                }
            }

            string_to_send
        }
        _ => "ok".to_string(),
    }
}
