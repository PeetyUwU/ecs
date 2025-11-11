use std::{any::Any, collections::HashMap};

use crate::ecs::resources::Resources;

#[derive(Default)]
pub struct World {
    resources: Resources,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_resource(&mut self, resource: impl Any) {
        self.resources.add_resource(resource);
    }

    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_resource::<T>()
    }

    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources.get_resource_mut::<T>()
    }

    pub fn remove_resource<T: Any>(&mut self) {
        self.resources.remove_resource::<T>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_resource() {
        let mut world = World::new();
        let fps = FpsResource(60);
        world.add_resource(fps);

        let fps = world.resources.get_resource::<FpsResource>().unwrap();

        assert_eq!(fps.0, 60);
    }

    #[test]
    fn get_resource() {
        let world = init_world();

        let fps = world.get_resource::<FpsResource>().unwrap();
        assert_eq!(fps.0, 60);
    }

    #[test]
    fn get_resource_mut() {
        let mut world = init_world();

        {
            let fps = world.get_resource_mut::<FpsResource>().unwrap();
            fps.0 += 1;
        }

        let fps = world.get_resource::<FpsResource>().unwrap();
        assert_eq!(fps.0, 61);
    }

    fn init_world() -> World {
        let mut world = World::new();
        let fps = FpsResource(60);
        world.add_resource(fps);

        world
    }

    struct FpsResource(pub u32);
}
