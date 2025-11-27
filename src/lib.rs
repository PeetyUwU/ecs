pub mod archetype;
pub mod column;
pub mod component;
pub mod entity;
pub mod event_bust;
pub mod query;
pub mod sheduler;
pub mod system;
pub mod world;

pub use sheduler::Scheduler;
pub use world::World;

pub struct GameServer {
    world: World,
    tick: u64,
    scheduler: Scheduler,
}

#[cfg(test)]
mod test {
    fn test_simulation() {}
}
