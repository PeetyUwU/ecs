use crate::ecs::component::Component;

pub struct Column<T: Component> {
    data: Vec<T>,
}

pub trait ColumnTrait {
    fn as_any(&self) -> &Self;
    fn as_any_mut(&mut self) -> &mut Self;
    fn len(&self) -> usize;
    fn swap_remove(&mut self, index: usize);
}

impl<T: Component> ColumnTrait for Column<T> {
    fn as_any(&self) -> &Self {
        self
    }
    fn as_any_mut(&mut self) -> &mut Self {
        self
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn swap_remove(&mut self, index: usize) {
        self.data.swap_remove(index);
    }
}
