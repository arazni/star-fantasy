use std::ops::Deref;

use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerOnMap;

#[derive(Component)]
pub struct MovableOnMap;

#[derive(Clone)]
pub enum Orientation {
	Up,
	Left,
	Right,
	Down
}

#[derive(Event, Clone)]
pub struct MovementEvent {
	change_x: f32,
	change_y: f32,
	orientation: Orientation
}

#[derive(Component)]
pub struct CameraComponent;

pub fn setup_camera(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	settings: Res<CameraSettings>
) {
	commands.spawn((
		Camera2d, 
		Projection::from(OrthographicProjection {
			scale: settings.transform_scale,
			..OrthographicProjection::default_2d()
	})));

	commands.spawn((
		PlayerOnMap,
		MovableOnMap,
		Sprite::from_image(
			asset_server.load("characters/mystic-walk-down.png")
		),
		// Transform::from_scale(Vec3::new(settings.transform_scale,settings.transform_scale,1.))
	));

	commands.add_observer(on_move);
}

// pub fn setup_zoom(
// 	mut query: Query<&mut OrthographicProjection, With<CameraComponent>>,
// 	settings: Res<CameraSettings>
// ) {
// 	let mut projection = query.single_mut();
// 	projection.scaling_mode = ScalingMode::WindowSize(settings.transform_scale);
// }

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
	mut mover_query: Query<&mut Transform, With<MovableOnMap>>,
	camera_setting: Res<CameraSettings>,
	sprite_setting: Res<SpriteSettings>
) {
	let direction = Vec2::new(trigger.change_x, trigger.change_y);
	let move_delta = direction * camera_setting.tile_size_f;

	if let Ok(mut mover) = mover_query.get_mut(trigger.target()) {
		mover.translation += move_delta.extend(0.);
	}
}

pub fn move_player(
	player_query: Single<Entity, With<PlayerOnMap>>,
	keyboard: Res<ButtonInput<KeyCode>>,
	mut commands: Commands
) {
	let player = player_query.into_inner();

	if keyboard.pressed(KeyCode::ArrowLeft) {
		commands.trigger_targets(
			MovementEvent {
				change_x: -1.,
				change_y: 0.,
				orientation: Orientation::Left
			},
			player
		);
	}

	else if keyboard.pressed(KeyCode::ArrowRight) {
		commands.trigger_targets(
			MovementEvent {
				change_x: 1.,
				change_y: 0.,
				orientation: Orientation::Right
			},
			player
		);
	}

	else if keyboard.pressed(KeyCode::ArrowDown) {
		commands.trigger_targets(
			MovementEvent {
				change_x: 0.,
				change_y: -1.,
				orientation: Orientation::Down
			},
			player
		);
	}

	else if keyboard.pressed(KeyCode::ArrowUp) {
		commands.trigger_targets(
			MovementEvent {
				change_x: 0.,
				change_y: 1.,
				orientation: Orientation::Up
			},
			player
		);	
	}
}

#[derive(Resource)]
pub struct CameraSettings {
	player_speed: f32,
	camera_decay_rate: f32,
	transform_scale: f32,
	tile_size: i32,
	tile_size_f: f32,
}

const SPRITE_UP_INDEX: i32 = 0;
const SPRITE_DOWN_INDEX: i32 = 1;
const SPRITE_LEFT1_INDEX: i32 = 2;
const SPRITE_LEFT2_INDEX: i32 = 3;

impl Default for CameraSettings {
	fn default() -> Self {
		CameraSettings {
			player_speed: 100.,
			camera_decay_rate: 2.,
			transform_scale: 0.25,
			tile_size: 16,
			tile_size_f: 16.,
		}
	}
}

#[derive(Resource)]
pub struct SpriteSettings {
	sprite_size: i32,
	sprite_size_f: f32,
}

impl Default for SpriteSettings {
	fn default() -> Self {
			SpriteSettings {
				sprite_size: 16,
				sprite_size_f: 16.
		 }
	}
}

#[derive(Component)]
struct AnimationConfig {
	first_sprite_index: usize,
	last_sprite_index: usize,
	fps: u8,
	frame_timer: Timer,
}

pub struct WorldCameraPlugin;

impl Plugin for WorldCameraPlugin {
	fn build(&self, app: &mut App) {
			app.insert_resource(CameraSettings { ..default() });
			app.insert_resource(SpriteSettings { ..default() });
			app.add_systems(Startup, setup_camera);
			app.add_systems(Update, (move_player, update_camera).chain());
	}
}
