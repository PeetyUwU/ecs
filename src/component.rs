use std::any::Any;

use serde::{Deserialize, Serialize};

pub trait Component: Any + 'static + Send + Sync + Default {}
impl<T: Any + 'static + Send + Sync + Default> Component for T {}

pub trait ComponentBundle {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Velocity {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
