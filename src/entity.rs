use std::{any::TypeId, collections::HashMap};

use crate::{component::Component, world::World};

pub type Entity = u64;

pub struct EntityBuilder<'a> {
    world: &'a mut World,
    pub entity: Entity,
}

impl<'a> EntityBuilder<'a> {
    pub fn new(world: &'a mut World, entity: Entity) -> Self {
        Self { world, entity }
    }

    pub fn with<T: Component>(self, component: T) -> Self {
        self.world.add_component(self.entity, component);
        self
    }

    pub fn build(self) -> Entity {
        self.entity
    }
}
