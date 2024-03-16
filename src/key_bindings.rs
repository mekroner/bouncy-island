use bevy::{ecs::system::Resource, input::keyboard::KeyCode};

#[derive(Resource)]
pub struct KeyBindings {
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub move_jump: KeyCode,
    pub attack: KeyCode,
    pub shield: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            move_jump: KeyCode::Space,
            attack: KeyCode::KeyH,
            shield: KeyCode::KeyJ,
        }
    }
}
