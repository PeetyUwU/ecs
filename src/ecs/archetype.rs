use std::{any::TypeId, collections::HashMap};

use crate::ecs::{component::Component, entity::Entity};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct ArchetypeId {
    pub components: Vec<TypeId>,
}

impl ArchetypeId {
    pub fn new(mut components: Vec<TypeId>) -> Self {
        components.sort();
        components.dedup();
        Self { components }
    }

    pub fn contains<T: Component>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        self.components.contains(&type_id)
    }

    pub fn contains_all(&self, components: Vec<TypeId>) -> bool {
        components.iter().all(|c| self.components.contains(c))
    }
}

pub struct Archetype {
    id: ArchetypeId,
    entities: Vec<Entity>,
    components: HashMap<TypeId, Vec<Box<dyn Component>>>,
}

impl Archetype {
    pub fn new(components: Vec<TypeId>) -> Self {
        let id = ArchetypeId::new(components);

        Self {
            id,
            entities: Vec::new(),
            components: HashMap::new(),
        }
    }

    pub fn remove_entity(&mut self, entity: Entity) -> Result<(), ArchetypeErrors> {
        if let Some(entity_idx) = self.entities.iter().position(|&e| e == entity) {
            for type_id in self.id.components.iter() {
                if let Some(components) = self.components.get_mut(type_id) {
                    components.swap_remove(entity_idx);
                } else {
                    eprintln!("Warning: Component for TypeId {:?} not found", type_id);
                }
            }

            self.entities.swap_remove(entity_idx);
            Ok(())
        } else {
            Err(ArchetypeErrors::EntityNotFound)
        }
    }

    pub fn add_entity(
        &mut self,
        entity: Entity,
        components: HashMap<TypeId, Box<dyn Component>>,
    ) -> Result<(), ArchetypeErrors> {
        if self.entities.contains(&entity) {
            return Err(ArchetypeErrors::EntityAlreadyExists);
        }

        let missing: Vec<TypeId> = self
            .id
            .components
            .iter()
            .filter(|c| !components.contains_key(&c))
            .cloned()
            .collect();

        if !missing.is_empty() {
            return Err(ArchetypeErrors::MissingComponent(missing));
        }

        let extra: Vec<TypeId> = components
            .keys()
            .filter(|c| !self.id.components.contains(c))
            .cloned()
            .collect();
        if !extra.is_empty() {
            return Err(ArchetypeErrors::ExtraComponent(extra));
        }

        self.entities.push(entity);

        for (type_id, component) in components {
            self.components
                .entry(type_id)
                .or_insert_with(Vec::new)
                .push(component);
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ArchetypeErrors {
    EntityAlreadyExists,
    EntityNotFound,
    MissingComponent(Vec<TypeId>),
    ExtraComponent(Vec<TypeId>),
}
