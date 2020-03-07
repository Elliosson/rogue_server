extern crate specs;
use super::{Map, Position, ToDelete};
use specs::prelude::*;

pub fn delete_entity_to_delete(ecs: &mut World) {
    let mut dead: Vec<Entity> = Vec::new();
    // Using a scope to make the borrow checker happy
    {
        let to_deletes = ecs.write_storage::<ToDelete>();
        let entities = ecs.entities();

        for (entity, _to_delete) in (&entities, &to_deletes).join() {
            dead.push(entity);
        }
    }

    for victim in dead {
        {
            //need to refressh map
            let positions = ecs.read_storage::<Position>();
            if let Some(pos) = positions.get(victim) {
                let mut map = ecs.write_resource::<Map>();
                map.dirty.push((pos.x(), pos.y()));
            }
        }
        ecs.delete_entity(victim).expect("Unable to delete");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn object_deleter_test() {
        //create a new world
        let mut ecs: World = World::new();
        ecs.register::<ToDelete>();

        ecs.create_entity().with(ToDelete {}).build();

        {
            let to_deletes = ecs.read_storage::<ToDelete>();
            let composant_count = to_deletes.join().count();

            assert_eq!(composant_count, 1);
        }

        delete_entity_to_delete(&mut ecs);

        let to_deletes = ecs.read_storage::<ToDelete>();
        let composant_count = to_deletes.join().count();

        assert_eq!(composant_count, 0);
    }
}
