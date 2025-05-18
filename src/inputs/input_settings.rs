use bevy::prelude::*;

#[derive(Resource)]
pub struct KeySettings {
	pub left: Vec<KeyCode>,
	pub right: Vec<KeyCode>,
	pub down: Vec<KeyCode>,
	pub up: Vec<KeyCode>
}

impl Default for KeySettings {
	fn default() -> Self {
		KeySettings {
			left: vec![KeyCode::KeyA, KeyCode::ArrowLeft],
			right: vec![KeyCode::KeyD, KeyCode::ArrowRight],
			down: vec![KeyCode::KeyS, KeyCode::ArrowDown],
			up: vec![KeyCode::KeyW, KeyCode::ArrowUp]
		}
	}
}