use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MovementState {
	#[default]
	Idle,
	Moving,
	// Busy
}