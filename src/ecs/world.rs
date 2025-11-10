use std::{any::Any, collections::HashMap};

use crate::ecs::{
    archetype::{Archetype, ArchetypeId},
    resources::Resources,
};

#[derive(Default)]
pub struct World {
    resources: Resources,
    archetypes: HashMap<ArchetypeId, Archetype>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_resource(&mut self, resource: impl Any) {
        self.resources.add_resource(resource);
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

    fn init_world() -> World {
        let mut world = World::new();
        let fps = FpsResource(60);
        world.add_resource(fps);

        world
    }

    struct FpsResource(pub u32);
}
