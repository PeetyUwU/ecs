pub mod archetype;
pub mod column;
pub mod component;
pub mod entity;
pub mod event_bus;
pub mod game_state;
pub mod input;
pub mod query;
pub mod resources;
pub mod sheduler;
pub mod system;
pub mod world;

use std::time::{Duration, Instant};

pub use sheduler::Scheduler;
pub use world::World;

use crate::input::{KeyCode, MouseButton, PlayerId, PlayerInputMap};

#[derive(Debug, Clone)]
pub enum InputEvent {
    KeyDown {
        key: KeyCode,
        player_id: PlayerId,
    },
    KeyUp {
        key: KeyCode,
        player_id: PlayerId,
    },
    MouseButtonDown {
        button: MouseButton,
        player_id: PlayerId,
    },
    MouseButtonUp {
        button: MouseButton,
        player_id: PlayerId,
    },
    MouseMove {
        x: f32,
        y: f32,
        player_id: PlayerId,
    },
}

pub struct GameServer {
    world: World,
    tick: u64,
    scheduler: Scheduler,
    input: PlayerInputMap,
}

impl GameServer {
    pub fn new(tick: u64) -> Self {
        Self {
            world: World::new(),
            tick,
            scheduler: Scheduler::new(),
            input: PlayerInputMap::new(),
        }
    }

    pub fn handle_input(&mut self, input_events: Vec<InputEvent>) {
        for event in input_events {
            match event {
                InputEvent::KeyDown { key, player_id } => {
                    let input = self.input.get_or_insert(player_id);
                    input.set_key_down(key);
                }
                InputEvent::KeyUp { key, player_id } => {
                    let input = self.input.get_or_insert(player_id);
                    input.set_key_up(key);
                }
                InputEvent::MouseButtonDown { button, player_id } => {
                    let input = self.input.get_or_insert(player_id);
                    input.set_mouse_button_down(button);
                }
                InputEvent::MouseButtonUp { button, player_id } => {
                    let input = self.input.get_or_insert(player_id);
                    input.set_mouse_button_up(button);
                }
                InputEvent::MouseMove { x, y, player_id } => {
                    let input = self.input.get_or_insert(player_id);
                    input.set_cursor_position(x, y);
                }
            }
        }

        // Add input state as a resource to the world for systems to access
        self.world.insert_resource(self.input.clone());
    }

    pub fn get_input(&self) -> &PlayerInputMap {
        &self.input
    }

    pub fn get_input_mut(&mut self) -> &mut PlayerInputMap {
        &mut self.input
    }

    fn poll_input_events(&self) -> Vec<InputEvent> {
        // In a real implementation, this would poll the window system/network
        // For server-side, this might receive input from network packets
        // For client-side, this would poll keyboard/mouse events
        Vec::new()
    }

    pub fn run(&mut self) {
        let tick_duration = Duration::from_millis(self.tick);
        let mut last_frame_time = Instant::now();

        loop {
            let frame_start = Instant::now();

            let delta_time = frame_start.duration_since(last_frame_time).as_secs_f32();
            last_frame_time = frame_start;

            // Clear previous frame input events
            self.input.clear_all_just_pressed_released();

            // Poll and process input events
            let input_events = self.poll_input_events();
            self.handle_input(input_events);

            // Run all systems with updated input state
            self.scheduler.run(&mut self.world, delta_time);

            let frame_time = frame_start.elapsed();
            if frame_time < tick_duration {
                std::thread::sleep(tick_duration - frame_time);
            }

            if self.should_shutdown() {
                break;
            }
        }
    }

    fn should_shutdown(&self) -> bool {
        // Check if any player pressed escape or quit action
        for (_, input) in &self.input.inputs {
            if input.is_key_just_pressed(KeyCode::Q) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    fn test_simulation() {}
}
