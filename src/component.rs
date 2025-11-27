use std::any::Any;

pub trait Component: Any + 'static + Send + Sync {}
