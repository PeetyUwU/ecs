use serde::{Deserialize, Serialize};

use crate::component::{Position, Velocity};

pub type PlayerId = u64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub tick: u64,
    pub entities: Vec<EntityState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityState {
    pub id: u64,
    pub position: Position,
    pub velocity: Velocity, // Add more fields as needed
}

impl GameState {
    pub fn new(tick: u64) -> Self {
        Self {
            tick,
            entities: Vec::new(),
        }
    }

    /// Placeholder for vision filtering
    /// In the future, this will filter entities based on player vision
    pub fn filter_by_vision(self, _player_id: PlayerId) -> Self {
        // TODO: Implement vision filtering
        // For now, return the full state
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_creation() {
        let state = GameState::new(100);
        assert_eq!(state.tick, 100);
        assert!(state.entities.is_empty());
    }

    #[test]
    fn test_vision_filter_placeholder() {
        let mut state = GameState::new(1);
        state.entities.push(EntityState {
            id: 1,
            position: Position {
                x: 10.0,
                y: 20.0,
                z: 30.0,
            },
            velocity: Velocity {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        });

        let filtered = state.filter_by_vision(1);
        assert_eq!(filtered.entities.len(), 1);
    }
}
