use std::any::{Any, TypeId};

use crate::ecs::component::{self, Component};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub struct Column<T: Component> {
    data: Vec<T>,
}

impl<T: Component> Column<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

pub trait ColumnTrait {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn len(&self) -> usize;
    fn swap_remove(&mut self, index: usize);
    fn push_any(&mut self, value: Box<dyn Any>);
}

impl<T: Component> ColumnTrait for Column<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn swap_remove(&mut self, index: usize) {
        self.data.swap_remove(index);
    }

    fn push_any(&mut self, value: Box<dyn Any>) {
        match value.downcast::<T>() {
            Ok(v) => self.data.push(*v),
            Err(e) => panic!(
                "Type mismatch when pushing into column. Expected {}, got.",
                std::any::type_name::<T>(),
            ),
        }
    }
}
