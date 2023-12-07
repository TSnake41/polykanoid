use std::collections::HashMap;

use bevy_ecs::system::Resource;
use raylib::{core::RaylibHandle, ffi::KeyboardKey};

#[derive(Resource, Clone, Copy)]
pub struct ScreenInfo {
    pub width: f32,
    pub height: f32,
}

#[derive(Default)]
pub struct KeyState {
    pub pressed: bool,
    pub down: bool,
    pub up: bool,
}

#[derive(Default, Resource)]
pub struct InputManager(HashMap<KeyboardKey, KeyState>);

impl InputManager {
    pub fn update(&mut self, rl: &RaylibHandle) {
        self.0.iter_mut().for_each(|(key, value)| {
            *value = KeyState {
                pressed: rl.is_key_pressed(*key),
                down: rl.is_key_down(*key),
                up: rl.is_key_pressed(*key),
            }
        })
    }

    pub fn track(&mut self, key: KeyboardKey) {
        self.0.insert(key, KeyState::default());
    }

    pub fn untrack(&mut self, key: KeyboardKey) {
        self.0.remove(&key);
    }

    pub fn is_key_down(&self, key: KeyboardKey) -> bool {
        self.0.get(&key).map(|state| state.down).unwrap_or_default()
    }

    pub fn is_key_pressed(&self, key: KeyboardKey) -> bool {
        self.0
            .get(&key)
            .map(|state| state.pressed)
            .unwrap_or_default()
    }

    pub fn is_key_up(&self, key: KeyboardKey) -> bool {
        self.0.get(&key).map(|state| state.up).unwrap_or_default()
    }
}
