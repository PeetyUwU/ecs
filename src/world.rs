use std::{any::TypeId, collections::HashMap};

use crate::{
    archetype::Archetype,
    component::{Component, ComponentBundle},
    entity::{self, Entity, EntityBuilder},
};

#[derive(Default)]
pub struct World {
    next_entity: u64,
    pub archetypes: Vec<Archetype>,
    pub entity_index: HashMap<Entity, (usize, usize)>, // (Archetypeindex, row)
    archetype_map: HashMap<Vec<TypeId>, usize>,
    scheduler: Scheduler,
    event_bus: EventBus,
}

impl World {
    pub fn new() -> Self {
        let mut world = Self {
            next_entity: 0,
            archetypes: Vec::new(),
            entity_index: HashMap::new(),
            archetype_map: HashMap::new(),
        };
        // Create empty archetype
        world.create_archetype(vec![]);
        world
    }

    fn create_archetype(&mut self, mut types: Vec<TypeId>) -> usize {
        types.sort();
        if let Some(&index) = self.archetype_map.get(&types) {
            return index;
        }

        let id = self.archetypes.len() as u64;
        let archetype = Archetype::new(id, types.clone());
        self.archetypes.push(archetype);
        let index = self.archetypes.len() - 1;
        self.archetype_map.insert(types, index);
        index
    }

    fn create_entity(&mut self) -> Entity {
        let entity = self.next_entity;
        self.next_entity += 1;

        let arch_index = 0;
        let row = self.archetypes[arch_index].push_entity(entity);
        self.entity_index.insert(entity, (arch_index, row));

        entity
    }

    pub fn spawn(&mut self) -> EntityBuilder<'_> {
        let entity = self.create_entity();
        EntityBuilder::new(self, entity)
    }

    pub fn add_component<T: Component>(&mut self, entity: Entity, component: T) {
        let (old_arch_index, old_row) = *self.entity_index.get(&entity).expect("Entity not found");

        let mut new_types = self.archetypes[old_arch_index].types.clone();
        if new_types.contains(&TypeId::of::<T>()) {
            if let Some(col) = self.archetypes[old_arch_index].get_column_mut::<T>() {
                col[old_row] = component;
            }
            return;
        }
        new_types.push(TypeId::of::<T>());
        let new_arch_index = self.create_archetype(new_types);

        self.move_entity(entity, old_arch_index, old_row, new_arch_index);

        self.archetypes[new_arch_index].push_component(component);
    }

    fn move_entity(&mut self, entity: Entity, old_arch: usize, old_row: usize, new_arch: usize) {
        let (min, max) = if old_arch < new_arch {
            (old_arch, new_arch)
        } else {
            (new_arch, old_arch)
        };
        let (left, right) = self.archetypes.split_at_mut(max);
        let (arch1, arch2) = (&mut left[min], &mut right[0]);

        let (source, target) = if old_arch < new_arch {
            (arch1, arch2)
        } else {
            (arch2, arch1)
        };

        if let Some(swapped_entity) = source.move_entity_to(target, old_row) {
            self.entity_index
                .insert(swapped_entity, (old_arch, old_row));
        }

        let new_row = target.entities.len() - 1;
        self.entity_index.insert(entity, (new_arch, new_row));
    }

    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        let (arch_index, row) = *self.entity_index.get(&entity)?;
        self.archetypes[arch_index]
            .get_column::<T>()
            .map(|col| &col[row])
    }

    pub fn get_component_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        let (arch_index, row) = *self.entity_index.get(&entity)?;
        self.archetypes[arch_index]
            .get_column_mut::<T>()
            .map(|col| &mut col[row])
    }

    pub fn query_archetypes<T: Component>(&self) -> Vec<usize> {
        self.archetypes
            .iter()
            .enumerate()
            .filter(|(_, arch)| arch.has::<T>())
            .map(|(i, _)| i)
            .collect()
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
