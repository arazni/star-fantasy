use bevy::prelude::*;
use crate::common::{size::*, states::*};
use super::settings::*;
use super::movement::{MovableOnMap, PlayerOnMap, Movement, MovementEvent, Orientation, move_player};

pub fn standard_atlas(setting: Res<SpriteSettings>, size: Size) -> TextureAtlasLayout {
	let sprite_size = setting.sprite_size * match size {
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

// pub fn update_camera(
// 	mut camera: Single<&mut Transform, (With<Camera2d>, Without<PlayerOnMap>)>,
// 	player: Single<&Transform, (With<PlayerOnMap>, Without<Camera2d>)>,
// 	time: Res<Time>,
// 	settings: Res<CameraSettings>
// ) {
// 	let Vec3 { x, y, .. } = player.translation;
// 	let direction = Vec3::new(x, y, camera.translation.z);

// 	camera.translation.smooth_nudge(&direction, settings.camera_decay_rate, time.delta_secs());
// }

pub fn on_move(
	trigger: Trigger<MovementEvent>,
	mut mover_query: Query<(&mut Transform, &mut Sprite, &mut MovableOnMap)>,
	camera_setting: Res<CameraSettings>
) {
	let Ok(mover) = mover_query.get_mut(trigger.target()) else {
		return;
	};
	let (mut transform, mut sprite, mut movable) = mover;

	let movement = Movement::new(trigger.event(), camera_setting.tile_size as f32);

	let Some(ref mut texture_atlas) = sprite.texture_atlas else {
		transform.translation += movement.move_vector;
		return;
	};

	movable.movement = movement;

	texture_atlas.index = match trigger.orientation {
		Orientation::Down => SPRITE_DOWN_INDEX,
		Orientation::Up => SPRITE_UP_INDEX,
		Orientation::Left | Orientation::Right => 
			if !matches!(texture_atlas.index, SPRITE_LEFT1_INDEX | SPRITE_LEFT2_INDEX) { SPRITE_LEFT1_INDEX }
			else { texture_atlas.index }
	};

	if trigger.orientation == Orientation::Left {
		sprite.flip_x = false;
	} else if trigger.orientation == Orientation::Right {
		sprite.flip_x = true;
	}
}

pub fn move_movables(
	time: Res<Time>,
	mut mover_query: Query<(&mut Transform, &mut Sprite, &mut MovableOnMap)>,
) 
{
	for (mut transform, mut sprite, mut movable) in &mut mover_query {
		if movable.movement.is_done() || movable.movement_state != MovementState::Moving {
			continue;
		}

		movable.movement.timer.tick(time.delta());

		if movable.movement.timer.just_finished() {
			movable.movement.steps_remaining -= 1;

			transform.translation += movable.movement.move_vector;
			
			if matches!(movable.movement.orientation, Orientation::Up | Orientation::Down) {
				sprite.flip_x = !sprite.flip_x;
			} else {
				if let Some(ref mut texture_atlas) = sprite.texture_atlas {
					texture_atlas.index = if texture_atlas.index == SPRITE_LEFT1_INDEX
					{ SPRITE_LEFT2_INDEX } else { SPRITE_LEFT1_INDEX }
				}
			}

			if movable.movement.is_done() {
				movable.movement_state = MovementState::Idle;
			}
		}
	}
}

pub struct WorldCameraPlugin;

impl Plugin for WorldCameraPlugin {
	fn build(&self, app: &mut App) {
			app.insert_resource(CameraSettings::default());
			app.insert_resource(SpriteSettings::default());
			app.insert_resource(MapMoveSettings::default());
			app.add_systems(Startup, (setup_camera, setup_player_on_map).chain());
			// app.add_systems(Update, (move_player, update_camera).chain());
			app.add_systems(Update, (move_player, move_movables).chain());
	}
}
