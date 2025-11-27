use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::{component::Component, entity::Entity};

// Helper for type-erased Vec operations
pub trait AnyVec: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn swap_remove_any(&mut self, index: usize);
    fn remove_as_box(&mut self, index: usize) -> Box<dyn Any>;
    fn push_box(&mut self, item: Box<dyn Any>);
    fn new_empty(&self) -> Box<dyn AnyVec>;
}

impl<T: Component> AnyVec for Vec<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn swap_remove_any(&mut self, index: usize) {
        self.swap_remove(index);
    }
    fn remove_as_box(&mut self, index: usize) -> Box<dyn Any> {
        Box::new(self.swap_remove(index))
    }
    fn push_box(&mut self, item: Box<dyn Any>) {
        if let Ok(val) = item.downcast::<T>() {
            self.push(*val);
        } else {
            panic!("Type mismatch in push_box");
        }
    }
    fn new_empty(&self) -> Box<dyn AnyVec> {
        Box::new(Vec::<T>::new())
    }
}

pub struct Archetype {
    pub id: u64,
    pub types: Vec<TypeId>,
    pub columns: HashMap<TypeId, Box<dyn AnyVec>>,
    pub entities: Vec<Entity>,
}

impl Archetype {
    pub fn new(id: u64, types: Vec<TypeId>) -> Self {
        Self {
            id,
            types,
            columns: HashMap::new(),
            entities: Vec::new(),
        }
    }

    pub fn has<T: Component>(&self) -> bool {
        self.types.contains(&TypeId::of::<T>())
    }

    pub fn add_column<T: Component>(&mut self) {
        self.columns
            .insert(TypeId::of::<T>(), Box::new(Vec::<T>::new()));
    }

    pub fn push_entity(&mut self, entity: Entity) -> usize {
        self.entities.push(entity);
        self.entities.len() - 1
    }

    pub fn push_component<T: Component>(&mut self, component: T) {
        let type_id = TypeId::of::<T>();
        if let Some(col) = self.columns.get_mut(&type_id) {
            if let Some(vec) = col.as_any_mut().downcast_mut::<Vec<T>>() {
                vec.push(component);
            } else {
                panic!("Column type mismatch");
            }
        } else {
            let mut vec = Vec::new();
            vec.push(component);
            self.columns.insert(type_id, Box::new(vec));
        }
    }

    pub fn swap_remove(&mut self, row: usize) -> Option<Entity> {
        let last_row = self.entities.len() - 1;
        self.entities.swap(row, last_row);
        let moved_entity = if row != last_row {
            Some(self.entities[row])
        } else {
            None
        };
        self.entities.pop();

        for col in self.columns.values_mut() {
            col.swap_remove_any(row);
        }

        moved_entity
    }

    pub fn get_column<T: Component>(&self) -> Option<&Vec<T>> {
        self.columns
            .get(&TypeId::of::<T>())?
            .as_any()
            .downcast_ref::<Vec<T>>()
    }

    pub fn get_column_mut<T: Component>(&mut self) -> Option<&mut Vec<T>> {
        self.columns
            .get_mut(&TypeId::of::<T>())?
            .as_any_mut()
            .downcast_mut::<Vec<T>>()
    }

    pub fn move_entity_to(&mut self, target: &mut Archetype, row: usize) -> Option<Entity> {
        let entity = self.entities[row];

        let type_ids_to_move: Vec<TypeId> = self.columns.keys().cloned().collect();

        for type_id in type_ids_to_move {
            if target.types.contains(&type_id) {
                if !target.columns.contains_key(&type_id) {
                    let source_col = self.columns.get(&type_id).unwrap();
                    target.columns.insert(type_id, source_col.new_empty());
                }

                let source_col = self.columns.get_mut(&type_id).unwrap();
                let target_col = target.columns.get_mut(&type_id).unwrap();

                let val = source_col.remove_as_box(row);
                target_col.push_box(val);
            } else {
                self.columns.get_mut(&type_id).unwrap().swap_remove_any(row);
            }
        }

        let last_row = self.entities.len() - 1;
        self.entities.swap(row, last_row);
        let moved_entity = if row != last_row {
            Some(self.entities[row])
        } else {
            None
        };
        self.entities.pop();

        target.entities.push(entity);

        moved_entity
    }
}
