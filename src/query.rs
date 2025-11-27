use std::marker::PhantomData;

use crate::{component::Component, entity::Entity, world::World};

pub struct Query<'a, T> {
    world: &'a World,
    _marker: PhantomData<T>,
}

impl<'a, T: Component> Query<'a, T> {
    pub fn new(world: &'a World) -> Self {
        Self {
            world,
            _marker: PhantomData,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.world
            .archetypes
            .iter()
            .filter(|arch| arch.has::<T>())
            .flat_map(|arch| {
                let entities = arch.entities.as_slice();
                let components = arch.get_column::<T>().unwrap().as_slice();
                entities.iter().copied().zip(components.iter())
            })
    }
}

pub struct QueryMut<'a, T> {
    world: &'a mut World,
    _marker: PhantomData<T>,
}

impl<'a, T: Component> QueryMut<'a, T> {
    pub fn new(world: &'a mut World) -> Self {
        Self {
            world,
            _marker: PhantomData,
        }
    }

    // To make iter_mut() we would have to use unsafe code

    pub fn for_each<F>(&mut self, mut f: F)
    where
        F: FnMut(Entity, &mut T),
    {
        for arch in &mut self.world.archetypes {
            if arch.has::<T>() {
                // Clone entities first to avoid double borrow of arch
                let entities = arch.entities.clone();

                // Then borrow column mutably
                if let Some(col) = arch.get_column_mut::<T>() {
                    for (i, &entity) in entities.iter().enumerate() {
                        f(entity, &mut col[i]);
                    }
                }
            }
        }
    }
}
