use std::{any::TypeId, collections::HashMap};

use crate::{
    archetype::Archetype,
    component::{Component, ComponentBundle},
    entity::{self, Entity, EntityBuilder},
    resources::{Resource, Resources},
};

#[derive(Default)]
pub struct World {
    next_entity: u64,
    pub archetypes: Vec<Archetype>,
    pub entity_index: HashMap<Entity, (usize, usize)>, // (Archetypeindex, row)
    archetype_map: HashMap<Vec<TypeId>, usize>,
    pub resources: Resources,
    // event_bus: EventBus,
}

impl World {
    pub fn new() -> Self {
        let mut world = Self {
            next_entity: 0,
            archetypes: Vec::new(),
            entity_index: HashMap::new(),
            archetype_map: HashMap::new(),
            resources: Resources::new(),
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

    pub fn insert_resource<R: Resource>(&mut self, resource: R) {
        self.resources.insert(resource);
    }

    pub fn get_resource<R: Resource>(&self) -> Option<&R> {
        self.resources.get::<R>()
    }

    pub fn get_resource_mut<R: Resource>(&mut self) -> Option<&mut R> {
        self.resources.get_mut::<R>()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[derive(Default)]
    struct Position {
        x: f32,
        y: f32,
        z: f32,
    }

    #[derive(Default)]
    struct Velocity {
        x: f32,
        y: f32,
        z: f32,
    }

    #[derive(Default)]
    struct Team(u8);

    #[test]
    fn create_entity() {
        let mut world = World::new();

        let entity = world
            .spawn()
            .with(Position {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            })
            .build();
        let entity2 = world
            .spawn()
            .with(Position {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            })
            .with(Velocity {
                x: 0.1,
                y: 0.2,
                z: 0.3,
            })
            .build();

        assert_eq!(entity, 0);
        assert_eq!(entity2, 1);
    }

    #[test]
    fn query_components() {
        let world = init_world();

        // Test querying for Position component
        let pos_archetypes = world.query_archetypes::<Position>();
        assert_eq!(pos_archetypes.len(), 2); // Should find 2 archetypes with Position

        // Test querying for Velocity component
        let vel_archetypes = world.query_archetypes::<Velocity>();
        assert_eq!(vel_archetypes.len(), 1); // Should find 1 archetype with Velocity

        // Test querying for Team component (none exist yet)
        let team_archetypes = world.query_archetypes::<Team>();
        assert_eq!(team_archetypes.len(), 0); // Should find 0 archetypes with Team

        // Test getting components
        let pos = world.get_component::<Position>(0).unwrap();
        assert_eq!(pos.x, 1.0);
        assert_eq!(pos.y, 2.0);
        assert_eq!(pos.z, 3.0);

        let vel = world.get_component::<Velocity>(1).unwrap();
        assert_eq!(vel.x, 0.1);
        assert_eq!(vel.y, 0.2);
        assert_eq!(vel.z, 0.3);

        // Test getting non-existent component
        assert!(world.get_component::<Velocity>(0).is_none());
        assert!(world.get_component::<Team>(0).is_none());
    }

    #[test]
    fn move_archetype() {
        let mut world = init_world();

        world.add_component(1, Team(2));
        // Verify entity 1 moved to new archetype with Team component
        let team = world.get_component::<Team>(1).unwrap();
        assert_eq!(team.0, 2);

        // Verify entity 1 still has its original components
        let pos = world.get_component::<Position>(1).unwrap();
        assert_eq!(pos.x, 4.0);
        assert_eq!(pos.y, 5.0);
        assert_eq!(pos.z, 6.0);

        let vel = world.get_component::<Velocity>(1).unwrap();
        assert_eq!(vel.x, 0.1);
        assert_eq!(vel.y, 0.2);
        assert_eq!(vel.z, 0.3);

        // Verify the entity moved to a different archetype
        let (arch_index, _) = world.entity_index.get(&1).unwrap();
        assert_ne!(*arch_index, 0); // Should not be in empty archetype anymore
    }

    fn init_world() -> World {
        let mut world = World::new();

        world
            .spawn()
            .with(Position {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            })
            .build();
        world
            .spawn()
            .with(Position {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            })
            .with(Velocity {
                x: 0.1,
                y: 0.2,
                z: 0.3,
            })
            .build();

        world
    }
}
