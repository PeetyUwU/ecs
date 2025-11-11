use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

pub struct Entities {
    components: HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any>>>>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_an_entity() {}
}
