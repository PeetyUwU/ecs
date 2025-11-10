use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[derive(Default)]
pub struct Resources {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    pub fn add_resource(&mut self, resource: impl Any) {
        let type_id = resource.type_id();
        self.data.insert(type_id, Box::new(resource));
    }

    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.data.get(&type_id)?.downcast_ref()
    }

    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.data.get_mut(&type_id)?.downcast_mut()
    }

    pub fn remove_resource<T: Any>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.data.remove(&type_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_resource() {
        let mut resources = Resources::default();
        let world_size = WorldSize { x: 10.0, y: 11.0 };
        resources.add_resource(world_size);

        assert_eq!(resources.data.len(), 1);
        let stored_world_size = resources.data.get(&TypeId::of::<WorldSize>()).unwrap();
        let extracted_world_size = stored_world_size.downcast_ref::<WorldSize>().unwrap();
        assert_eq!(extracted_world_size.x, 10.0);
    }

    #[test]
    fn get_resource() {
        let resources = init_resources();
        let world_size = resources.get_resource::<WorldSize>().unwrap();

        assert_eq!(world_size.x, 10.0);
    }

    #[test]
    fn get_resource_mut() {
        let mut resources = init_resources();

        {
            let world_size = resources.get_resource_mut::<WorldSize>().unwrap();
            world_size.x += 1.0;
        }
        let world_size = resources.get_resource::<WorldSize>().unwrap();
        assert_eq!(world_size.x, 11.0);
    }

    #[test]
    fn remove_resource() {
        let mut resources = init_resources();

        resources.remove_resource::<WorldSize>();

        assert!(!resources.data.contains_key(&TypeId::of::<WorldSize>()));
    }

    fn init_resources() -> Resources {
        let mut resources = Resources::default();

        let world_size = WorldSize { x: 10.0, y: 11.0 };

        resources.add_resource(world_size);

        resources
    }

    struct WorldSize {
        pub x: f32,
        pub y: f32,
    }
}
