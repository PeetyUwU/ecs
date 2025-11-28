use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum KeyCode {
    W,
    A,
    S,
    D,
    Q,
    E,
    R,
    Space,
    // Add more as needed
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Default, Clone)]
pub struct Input {
    keys_pressed: HashSet<KeyCode>,
    keys_just_pressed: HashSet<KeyCode>,
    keys_just_released: HashSet<KeyCode>,
    mouse_buttons_pressed: HashSet<MouseButton>,
    mouse_buttons_just_pressed: HashSet<MouseButton>,
    mouse_buttons_just_released: HashSet<MouseButton>,
    cursor_position: (f32, f32),
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_key_down(&mut self, key: KeyCode) {
        if self.keys_pressed.insert(key) {
            self.keys_just_pressed.insert(key);
        }
    }

    pub fn set_key_up(&mut self, key: KeyCode) {
        if self.keys_pressed.remove(&key) {
            self.keys_just_released.insert(key);
        }
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_key_just_pressed(&self, key: KeyCode) -> bool {
        self.keys_just_pressed.contains(&key)
    }

    pub fn is_key_just_released(&self, key: KeyCode) -> bool {
        self.keys_just_released.contains(&key)
    }

    pub fn set_mouse_button_down(&mut self, button: MouseButton) {
        if self.mouse_buttons_pressed.insert(button) {
            self.mouse_buttons_just_pressed.insert(button);
        }
    }

    pub fn set_mouse_button_up(&mut self, button: MouseButton) {
        if self.mouse_buttons_pressed.remove(&button) {
            self.mouse_buttons_just_released.insert(button);
        }
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons_pressed.contains(&button)
    }

    pub fn is_mouse_button_just_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons_just_pressed.contains(&button)
    }

    pub fn is_mouse_button_just_released(&self, button: MouseButton) -> bool {
        self.mouse_buttons_just_released.contains(&button)
    }

    pub fn set_cursor_position(&mut self, x: f32, y: f32) {
        self.cursor_position = (x, y);
    }

    pub fn cursor_position(&self) -> (f32, f32) {
        self.cursor_position
    }

    pub fn clear_just_pressed_released(&mut self) {
        self.keys_just_pressed.clear();
        self.keys_just_released.clear();
        self.mouse_buttons_just_pressed.clear();
        self.mouse_buttons_just_released.clear();
    }
}

pub type PlayerId = u64;

#[derive(Default, Clone)]
pub struct PlayerInputMap {
    pub inputs: HashMap<PlayerId, Input>,
}

impl PlayerInputMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, player_id: PlayerId) -> Option<&Input> {
        self.inputs.get(&player_id)
    }

    pub fn get_mut(&mut self, player_id: PlayerId) -> Option<&mut Input> {
        self.inputs.get_mut(&player_id)
    }

    pub fn insert(&mut self, player_id: PlayerId, input: Input) {
        self.inputs.insert(player_id, input);
    }

    pub fn get_or_insert(&mut self, player_id: PlayerId) -> &mut Input {
        self.inputs.entry(player_id).or_insert_with(Input::new)
    }

    pub fn clear_all_just_pressed_released(&mut self) {
        for input in self.inputs.values_mut() {
            input.clear_just_pressed_released();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_keys() {
        let mut input = Input::new();

        input.set_key_down(KeyCode::Q);
        assert!(input.is_key_pressed(KeyCode::Q));
        assert!(input.is_key_just_pressed(KeyCode::Q));

        input.clear_just_pressed_released();
        assert!(input.is_key_pressed(KeyCode::Q));
        assert!(!input.is_key_just_pressed(KeyCode::Q));

        input.set_key_up(KeyCode::Q);
        assert!(!input.is_key_pressed(KeyCode::Q));
        assert!(input.is_key_just_released(KeyCode::Q));
    }

    #[test]
    fn test_input_mouse() {
        let mut input = Input::new();

        input.set_mouse_button_down(MouseButton::Right);
        assert!(input.is_mouse_button_pressed(MouseButton::Right));
        assert!(input.is_mouse_button_just_pressed(MouseButton::Right));

        input.set_cursor_position(100.0, 200.0);
        assert_eq!(input.cursor_position(), (100.0, 200.0));
    }
}
