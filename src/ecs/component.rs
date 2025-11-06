use std::any::Any;

pub trait Component: 'static + Send + Sync + Any {}
