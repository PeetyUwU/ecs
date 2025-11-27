use std::{any::TypeId, collections::HashMap};

use crate::component::Component;

#[derive(Default)]
pub struct World {
    next_id: u64,
    archetypes: Archetype,
    scheduler: Scheduler,
    entities: Vec<Entity>,
    event_bus: EventBus,
}

impl World {
    pub fn spawn_entity(&mut self) -> EntityBuilder {
        let builder = EntityBuilder::new(self.next_id);
        self.next_id += 1;
        builder
    }
}

#[cfg(test)]
mod test {

    use super::*;

    struct Position {
        x: f32,
        y: f32,
        z: f32,
    }

    struct Velocity {
        x: f32,
        y: f32,
        z: f32,
    }

    #[test]
    fn create_entity() {
        let mut world = World::default();

        // world.register_component::<Position>();
        // world.register_component::<Velocity>();

        let entity = world.spawn_entity().with_component::<Position>().build();
        let entity2 = world
            .spawn_entity()
            .with_component::<Position>()
            .with_component::<Velocity>()
            .build();

        assert_eq!(entity.id, 0);
        assert_eq!(entity2.id, 1);
    }

    #[test]
    fn query_components() {}

    #[test]
    fn move_archetype() {}
}
