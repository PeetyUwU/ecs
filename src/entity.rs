use std::{any::TypeId, collections::HashMap};

use crate::component::Component;

pub struct Entity(u64);

pub struct EntityBuilder {
    entity: Entity,
    components: Vec<TypeId>,
}

impl EntityBuilder {
    pub fn new(id: u64) -> Self {
        Self {
            entity: Entity(id),
            components: Vec::new(),
        }
    }

    pub fn with_component<T: Component>(&mut self) {
        let type_id: TypeId = TypeId::of::<T>();
        self.components.push(type_id);
    }

    pub fn build() {}
}
