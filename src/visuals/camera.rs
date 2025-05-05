use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerOnMap;

pub fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn(Camera2d);

	commands.spawn((
		PlayerOnMap,
		Sprite::from_image(
			asset_server.load("characters/soldier-neutral-down.png")
		),
		Transform::from_scale(Vec3::new(4.,4.,1.))
	));
}

pub fn update_camera(mut camera: Single<&mut Transform, (With<Camera2d>, Without<PlayerOnMap>)>,
	player: Single<&Transform, (With<PlayerOnMap>, Without<Camera2d>)>,
	time: Res<Time>,
	settings: Res<CameraSettings>
) {
	let Vec3 { x, y, .. } = player.translation;
	let direction = Vec3::new(x, y, camera.translation.z);

	camera.translation.smooth_nudge(&direction, settings.camera_decay_rate, time.delta_secs());
}

pub fn move_player(
	mut player: Single<&mut Transform, With<PlayerOnMap>>,
	time: Res<Time>,
	keyboard: Res<ButtonInput<KeyCode>>,
	settings: Res<CameraSettings>
) {
	let mut direction = Vec2::ZERO;

	if keyboard.pressed(KeyCode::ArrowLeft) {
		direction.x -= 1.;
	}

	if keyboard.pressed(KeyCode::ArrowRight) {
		direction.x += 1.;
	}

	if keyboard.pressed(KeyCode::ArrowDown) {
		direction.y -= 1.;
	}

	if keyboard.pressed(KeyCode::ArrowUp) {
		direction.y += 1.;
	}

	let move_delta = direction.normalize_or_zero() * settings.player_speed * time.delta_secs();
	player.translation += move_delta.extend(0.);
}

#[derive(Resource)]
pub struct CameraSettings {
	player_speed: f32,
	camera_decay_rate: f32
}

impl Default for CameraSettings {
	fn default() -> Self {
		CameraSettings {
			player_speed: 100.,
			camera_decay_rate: 2.
		}
	}
}

pub struct WorldCameraPlugin;

impl Plugin for WorldCameraPlugin {
	fn build(&self, app: &mut App) {
			app.insert_resource(CameraSettings { ..default() });
			app.add_systems(Startup, setup_camera);
			app.add_systems(Update, (move_player, update_camera).chain());
	}
}
