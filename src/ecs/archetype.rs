use std::{any::TypeId, collections::HashMap};

use crate::ecs::{
    column::{self, Column, ColumnTrait},
    component::Component,
    entity::Entity,
};

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
    columns: HashMap<TypeId, Box<dyn ColumnTrait>>,
}

impl Archetype {
    pub fn new(component_types: Vec<TypeId>) -> Self {
        let id = ArchetypeId::new(component_types);
        Self {
            id,
            entities: Vec::new(),
            columns: HashMap::new(),
        }
    }

    pub fn remove_entity(&mut self, entity: Entity) -> Result<(), ArchetypeErrors> {
        if let Some(entity_idx) = self.entities.iter().position(|&e| e == entity) {
            for type_id in self.id.components.iter() {
                if let Some(column) = self.columns.get_mut(type_id) {
                    column.swap_remove(entity_idx);
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
            if let Some(column) = self.columns.get_mut(&type_id) {
                column.push_any(component);
            } else {
                // TODO create column based on type of the component
                // let mut column =
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct TestComponentA(i32);
    impl Component for TestComponentA {}

    #[derive(Debug, PartialEq)]
    struct TestComponentB(String);
    impl Component for TestComponentB {}

    #[test]
    fn test_add_two_entities() {
        let type_id_a = TypeId::of::<TestComponentA>();
        let type_id_b = TypeId::of::<TestComponentB>();

        let mut archetype = Archetype::new(vec![type_id_a, type_id_b]);

        let entity1 = Entity {
            id: 0,
            generation: 0,
        };
        let entity2 = Entity {
            id: 1,
            generation: 0,
        };

        let mut components1 = HashMap::new();
        components1.insert(
            type_id_a,
            Box::new(TestComponentA(42)) as Box<dyn Component>,
        );
        components1.insert(
            type_id_b,
            Box::new(TestComponentB("Hello".to_string())) as Box<dyn Component>,
        );

        let mut components2 = HashMap::new();
        components2.insert(
            type_id_a,
            Box::new(TestComponentA(99)) as Box<dyn Component>,
        );
        components2.insert(
            type_id_b,
            Box::new(TestComponentB("World".to_string())) as Box<dyn Component>,
        );

        // Add first entity
        assert!(archetype.add_entity(entity1, components1).is_ok());
        assert!(archetype.entities.contains(&entity1));

        // Add second entity
        assert!(archetype.add_entity(entity2, components2).is_ok());
        assert!(archetype.entities.contains(&entity2));

        // Verify the entities and components are added correctly
        assert_eq!(archetype.entities.len(), 2);
    }
}
