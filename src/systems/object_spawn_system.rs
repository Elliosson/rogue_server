extern crate specs;
use crate::raws::*;
use crate::{components::*, Name, ToSpawnList};
use specs::prelude::*;

//Trully item spawn system, to rename or to extend
pub struct ObjectSpawnSystem {}

impl<'a> System<'a> for ObjectSpawnSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, Name>,
        WriteStorage<'a, Item>,
        WriteExpect<'a, ObjectBuilder>,
        WriteExpect<'a, ToSpawnList>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut positions,
            mut renderables,
            mut names,
            mut items,
            mut object_builder,
            mut to_spawn,
        ) = data;
        for request in object_builder.requests.iter() {
            //for now it's just call to_spawn, but it's could be useful to have this level of abstraction in case I have something to do here
            to_spawn.request(request.x, request.y, request.name.clone());
        }

        object_builder.requests.clear();
    }
}

pub struct ObjectBuilder {
    requests: Vec<ObjectRequest>,
}

impl ObjectBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ObjectBuilder {
        ObjectBuilder {
            requests: Vec::new(),
        }
    }

    pub fn request(&mut self, x: i32, y: i32, name: String) {
        self.requests.push(ObjectRequest { x, y, name });
    }
}

//TODO traveaux en cour pour object builder
struct ObjectRequest {
    x: i32,
    y: i32,
    name: String,
}
