use std::time::{Duration, Instant};

use bevy::{platform::thread, prelude::*};
use crate::common::{u_int_float::*, size::*, states::*};
use super::movement::*;

pub fn standard_atlas(setting: Res<SpriteSettings>, size: Size) -> TextureAtlasLayout {
	let sprite_size = setting.sprite_size.u * match size {
		Size::Small => 1,
		Size::Medium => 1,
		Size::Large => 2,
		Size::Huge => 3,
		Size::Gargantuan => 4
	};

	TextureAtlasLayout::from_grid(UVec2::splat(sprite_size), 4, 1, None, None)
}

#[derive(Component)]
pub struct CameraComponent;

pub fn setup_camera(
	mut commands: Commands,
	settings: Res<CameraSettings>,
) {
	commands.spawn((
		Camera2d, 
		Projection::from(OrthographicProjection {
			scale: settings.transform_scale,
			..OrthographicProjection::default_2d()
	})));
}

pub fn setup_player_on_map(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	sprite_settings: Res<SpriteSettings>,
	mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
)	{
	let handler = texture_atlas_layouts.add(standard_atlas(sprite_settings, Size::Medium));

	commands.spawn((
		PlayerOnMap,
		MovableOnMap::default(),
		Sprite::from_atlas_image(
			asset_server.load("characters/mystic-animation.png"),
			TextureAtlas {
				layout: handler,
				index: SPRITE_DOWN_INDEX
			}
		),
	));

	commands.add_observer(on_move);
}

pub fn update_camera(
	mut camera: Single<&mut Transform, (With<Camera2d>, Without<PlayerOnMap>)>,
	player: Single<&Transform, (With<PlayerOnMap>, Without<Camera2d>)>,
	time: Res<Time>,
	settings: Res<CameraSettings>
) {
	let Vec3 { x, y, .. } = player.translation;
	let direction = Vec3::new(x, y, camera.translation.z);

	camera.translation.smooth_nudge(&direction, settings.camera_decay_rate, time.delta_secs());
}

pub fn on_move(
	trigger: Trigger<MovementEvent>,
	mut mover_query: Query<(&mut Transform, &mut Sprite, &mut MovableOnMap)>,
	camera_setting: Res<CameraSettings>
) {
	let direction = Vec2::new(trigger.change_x, trigger.change_y);
	let move_delta = direction * camera_setting.tile_size.f;

	let Ok(mover) = mover_query.get_mut(trigger.target()) else {
		return;
	};
	let (mut transform, mut sprite, mut movable) = mover;

	transform.translation += move_delta.extend(0.);
	let Some(ref mut texture_atlas) = sprite.texture_atlas else {
		return;
	};

	if trigger.orientation == Orientation::Down {
		texture_atlas.index = SPRITE_DOWN_INDEX;
		sprite.flip_x = !sprite.flip_x;
		movable.movement_state = MovementState::Idle;
		return;
	}
	if trigger.orientation == Orientation::Up {
		texture_atlas.index = SPRITE_UP_INDEX;
		sprite.flip_x = !sprite.flip_x;
		movable.movement_state = MovementState::Idle;
		return;
	}

	texture_atlas.index = if texture_atlas.index == SPRITE_LEFT1_INDEX { SPRITE_LEFT2_INDEX } 
		else { SPRITE_LEFT1_INDEX };
	
	sprite.flip_x = if trigger.orientation == Orientation::Left { false }
		else { true };

	movable.movement_state = MovementState::Idle;
}

#[derive(Resource)]
pub struct CameraSettings {
	// player_speed: f32,
	camera_decay_rate: f32,
	transform_scale: f32,
	tile_size: UIntFloat
}

const SPRITE_DOWN_INDEX: usize = 0;
const SPRITE_UP_INDEX: usize = 1;
const SPRITE_LEFT1_INDEX: usize = 2;
const SPRITE_LEFT2_INDEX: usize = 3;

impl Default for CameraSettings {
	fn default() -> Self {
		CameraSettings {
			// player_speed: 100.,
			camera_decay_rate: 2.,
			transform_scale: 0.5,
			tile_size: UIntFloat::new(16)
		}
	}
}

#[derive(Resource)]
pub struct SpriteSettings {
	sprite_size: UIntFloat
}

impl Default for SpriteSettings {
	fn default() -> Self {
			SpriteSettings {
				sprite_size: UIntFloat::new(16)
		 }
	}
}

pub struct WorldCameraPlugin;

impl Plugin for WorldCameraPlugin {
	fn build(&self, app: &mut App) {
			app.insert_resource(CameraSettings { ..default() });
			app.insert_resource(SpriteSettings { ..default() });
			app.add_systems(Startup, (setup_camera, setup_player_on_map).chain());
			// app.add_systems(Update, (move_player, update_camera).chain());
			app.add_systems(Update, move_player);
	}
}
