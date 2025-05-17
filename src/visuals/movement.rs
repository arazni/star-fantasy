use bevy::prelude::*;
use super::settings::MapMoveSettings;

use crate::common::{states::MovementState};

#[derive(Clone, PartialEq, Eq, Copy)]
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
	pub seconds: f32,
	pub steps: i32,
}

pub struct Movement {
	pub move_vector: Vec3,
	pub orientation: Orientation,
	pub timer: Timer,
	pub steps_remaining: i32,
}

impl Movement {
	pub fn is_done(&self) -> bool {
		self.steps_remaining < 1
	}

	pub fn new(event: &MovementEvent, tile_size: f32) -> Self {
		let base_vector = Vec2::new(event.change_x, event.change_y);
		let magnitude_modifier = tile_size / event.steps as f32;
		
		Movement {
			move_vector: (base_vector * magnitude_modifier).extend(0.),
			orientation: event.orientation,
			timer: Timer::from_seconds(event.seconds / event.steps as f32, TimerMode::Repeating),
			steps_remaining: event.steps
		}
	}
}

impl Default for Movement {
	fn default() -> Self {
			Movement {
				move_vector: Vec3::ZERO,
				orientation: Orientation::Down,
				timer: Timer::default(),
				steps_remaining: 0
			}
	}
}

#[derive(Component)]
pub struct PlayerOnMap;

#[derive(Component, Default)]
pub struct MovableOnMap {
	pub movement_state: MovementState,
	pub movement: Movement
}

pub fn move_player(
	player_query: Single<(Entity, &mut MovableOnMap), With<PlayerOnMap>>,
	keyboard: Res<ButtonInput<KeyCode>>,
	move_setting: Res<MapMoveSettings>,
	mut commands: Commands
) {
	let (player, mut movable) = player_query.into_inner();

	if movable.movement_state != MovementState::Idle {
		return;
	}

	if keyboard.pressed(KeyCode::ArrowLeft) {
		movable.movement_state = MovementState::Moving;
		commands.trigger_targets(
			MovementEvent {
				change_x: -1.,
				change_y: 0.,
				orientation: Orientation::Left,
				seconds: move_setting.seconds_per_tile,
				steps: move_setting.steps_per_tile
			},
			player
		);
	}

	else if keyboard.pressed(KeyCode::ArrowRight) {
		movable.movement_state = MovementState::Moving;
		commands.trigger_targets(
			MovementEvent {
				change_x: 1.,
				change_y: 0.,
				orientation: Orientation::Right,
				seconds: move_setting.seconds_per_tile,
				steps: move_setting.steps_per_tile
			},
			player
		);
	}

	else if keyboard.pressed(KeyCode::ArrowDown) {
		movable.movement_state = MovementState::Moving;
		commands.trigger_targets(
			MovementEvent {
				change_x: 0.,
				change_y: -1.,
				orientation: Orientation::Down,
				seconds: move_setting.seconds_per_tile,
				steps: move_setting.steps_per_tile
			},
			player
		);
	}

	else if keyboard.pressed(KeyCode::ArrowUp) {
		movable.movement_state = MovementState::Moving;
		commands.trigger_targets(
			MovementEvent {
				change_x: 0.,
				change_y: 1.,
				orientation: Orientation::Up,
				seconds: move_setting.seconds_per_tile,
				steps: move_setting.steps_per_tile
			},
			player
		);	
	}
}
