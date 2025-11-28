use std::any::{Any, TypeId};
use std::collections::HashMap;

pub trait Resource: Any + Send + Sync + 'static {}
impl<T: Any + Send + Sync + 'static> Resource for T {}

#[derive(Default)]
pub struct Resources {
    data: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Resources {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<R: Resource>(&mut self, resource: R) {
        self.data.insert(TypeId::of::<R>(), Box::new(resource));
    }

    pub fn get<R: Resource>(&self) -> Option<&R> {
        self.data
            .get(&TypeId::of::<R>())
            .and_then(|b| b.downcast_ref())
    }

    pub fn get_mut<R: Resource>(&mut self) -> Option<&mut R> {
        self.data
            .get_mut(&TypeId::of::<R>())
            .and_then(|b| b.downcast_mut())
    }

    pub fn remove<R: Resource>(&mut self) -> Option<R> {
        self.data
            .remove(&TypeId::of::<R>())
            .and_then(|b| b.downcast().ok())
            .map(|b| *b)
    }

    pub fn contains<R: Resource>(&self) -> bool {
        self.data.contains_key(&TypeId::of::<R>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct TestResource(i32);

    #[test]
    fn test_resources() {
        let mut resources = Resources::new();
        resources.insert(TestResource(123));

        assert!(resources.contains::<TestResource>());
        assert_eq!(resources.get::<TestResource>(), Some(&TestResource(123)));

        if let Some(res) = resources.get_mut::<TestResource>() {
            res.0 += 1;
        }

        assert_eq!(resources.get::<TestResource>(), Some(&TestResource(124)));

        let removed = resources.remove::<TestResource>();
        assert_eq!(removed, Some(TestResource(124)));
        assert!(!resources.contains::<TestResource>());
    }

    #[test]
    fn test_resources_string() {
        let mut resources = Resources::new();
        resources.insert("hello".to_string());

        assert!(resources.contains::<String>());
        assert_eq!(resources.get::<String>(), Some(&"hello".to_string()));

        if let Some(res) = resources.get_mut::<String>() {
            res.push_str(" world");
        }

        assert_eq!(resources.get::<String>(), Some(&"hello world".to_string()));

        let removed = resources.remove::<String>();
        assert_eq!(removed, Some("hello world".to_string()));
        assert!(!resources.contains::<String>());
    }
}
