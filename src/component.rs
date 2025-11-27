use std::any::Any;

pub trait Component: Any + 'static + Send + Sync + Default {}
impl<T: Any + 'static + Send + Sync + Default> Component for T {}

pub trait ComponentBundle {}
