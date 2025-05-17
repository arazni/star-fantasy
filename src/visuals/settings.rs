use bevy::prelude::*;

pub const SPRITE_DOWN_INDEX: usize = 0;
pub const SPRITE_UP_INDEX: usize = 1;
pub const SPRITE_LEFT1_INDEX: usize = 2;
pub const SPRITE_LEFT2_INDEX: usize = 3;

#[derive(Resource)]
pub struct CameraSettings {
	// camera_decay_rate: f32,
	pub transform_scale: f32,
	pub tile_size: i32
}

impl Default for CameraSettings {
	fn default() -> Self {
		CameraSettings {
			// camera_decay_rate: 2.,
			transform_scale: 0.5,
			tile_size: 16
		}
	}
}

#[derive(Resource)]
pub struct MapMoveSettings {
	pub seconds_per_tile: f32,
	pub steps_per_tile: i32,
}

impl Default for MapMoveSettings {
	fn default() -> Self {
		MapMoveSettings {
			seconds_per_tile: 0.125,
			steps_per_tile: 3
		}
	}
}

#[derive(Resource)]
pub struct SpriteSettings {
	pub sprite_size: u32
}

impl Default for SpriteSettings {
	fn default() -> Self {
			SpriteSettings {
				sprite_size: 16
		 }
	}
}