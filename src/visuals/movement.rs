use bevy::prelude::*;

use crate::common::{u_int_float::UIntFloat, states::MovementState};

#[derive(Clone, PartialEq, Eq)]
pub enum Orientation {
	Up,
	Left,
	Right,
	Down
}

#[derive(Event, Clone)]
pub struct MovementEvent {
	pub change_x: f32,
	pub change_y: f32,
	pub orientation: Orientation,
	// pub timer: Timer,
	// pub steps: UIntFloat,
}

#[derive(Component)]
pub struct PlayerOnMap;

#[derive(Component, Default)]
pub struct MovableOnMap {
	pub movement_state: MovementState
}

pub fn move_player(
	player_query: Single<(Entity, &mut MovableOnMap), With<PlayerOnMap>>,
	keyboard: Res<ButtonInput<KeyCode>>,
	mut commands: Commands
) {
	let (player, mut movable) = player_query.into_inner();

	if movable.movement_state != MovementState::Idle {
		return;
	}
	// movable.movement_state = MovementState::Moving;

	if keyboard.just_pressed(KeyCode::ArrowLeft) {
		commands.trigger_targets(
			MovementEvent {
				change_x: -1.,
				change_y: 0.,
				orientation: Orientation::Left,
				// timer: Timer::from_seconds(2., TimerMode::Repeating),
				// steps: UIntFloat::new(10)
			},
			player
		);
	}

	else if keyboard.just_pressed(KeyCode::ArrowRight) {
		commands.trigger_targets(
			MovementEvent {
				change_x: 1.,
				change_y: 0.,
				orientation: Orientation::Right,
				// timer: Timer::from_seconds(4., TimerMode::Repeating),
				// steps: UIntFloat::new(8)
			},
			player
		);
	}

	else if keyboard.just_pressed(KeyCode::ArrowDown) {
		commands.trigger_targets(
			MovementEvent {
				change_x: 0.,
				change_y: -1.,
				orientation: Orientation::Down,
				// timer: Timer::from_seconds(0.04, TimerMode::Repeating),
				// steps: UIntFloat::new(8)
			},
			player
		);
	}

	else if keyboard.just_pressed(KeyCode::ArrowUp) {
		commands.trigger_targets(
			MovementEvent {
				change_x: 0.,
				change_y: 1.,
				orientation: Orientation::Up,
				// timer: Timer::from_seconds(1., TimerMode::Repeating),
				// steps: UIntFloat::new(8)
			},
			player
		);	
	}
}
