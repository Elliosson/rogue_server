use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Message {
    Register,
    Play(Uuid),
    Positions(Uuid),
    UP(Uuid),
    DOWN(Uuid),
    RIGHT(Uuid),
    LEFT(Uuid),
    Exit(Uuid),
    Map(Uuid),
    Interact(Uuid, i32, i32, String, u32, i32), //x, y, name, id, gen
}

impl Message {
    pub fn from(msg: &str) -> Option<(Message, String)> {
        if msg.starts_with("register") {
            Some((Message::Register, "register".to_string()))
        } else {
            let mut parts = msg.split_whitespace();
            let id = match parts.next()?.parse() {
                Ok(id) => id,
                Err(_) => return None,
            };

            let command = parts.next()?;
            let msg = match command {
                "play" => Some(Message::Play(id)),
                "positions" => Some(Message::Positions(id)),
                "map" => Some(Message::Map(id)),
                "up" => Some(Message::UP(id)),
                "down" => Some(Message::DOWN(id)),
                "right" => Some(Message::RIGHT(id)),
                "left" => Some(Message::LEFT(id)),
                "exit" => Some(Message::Exit(id)),
                "interact" => {
                    let interact: Vec<&str> = parts.collect();
                    let x: i32 = interact[0].parse().unwrap();
                    let y: i32 = interact[1].parse().unwrap();
                    let name: String = interact[2].parse().unwrap();
                    let ent_id: u32 = interact[3].parse().unwrap();
                    let gen: i32 = interact[4].parse().unwrap();

                    Some(Message::Interact(id, x, y, name, ent_id, gen))
                }
                _ => None,
            };

            match msg {
                Some(msg) => Some((msg, command.to_string())),
                None => None,
            }
        }
    }
}
