use bevy::prelude::*;
use crate::common::{size::*, states::*};
use crate::inputs::input_settings::KeySettings;
use super::settings::*;
use super::movement::{MovableOnMap, PlayerOnMap, Movement, MovementEvent, Orientation, move_player};
use super::asset_constants::*;

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
			scale: settings.transform_scale_ring.current(),
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
			asset_server.load(MAP_MOVABLE_PLAYER_CHARACTER_MYSTIC),
			TextureAtlas {
				layout: handler,
				index: SPRITE_DOWN_INDEX
			}
		),
	));

	commands.add_observer(on_move);
}

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
			if matches!(texture_atlas.index, SPRITE_LEFT1_INDEX | SPRITE_LEFT2_INDEX) { texture_atlas.index }
			else { SPRITE_LEFT1_INDEX }
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

pub fn cycle_zoom(
	mut camera_settings: ResMut<CameraSettings>,
	key_settings: Res<KeySettings>,
	keyboard: Res<ButtonInput<KeyCode>>,
	projection: Single<&mut Projection, With<Camera2d>>
) {
	if !keyboard.any_just_pressed(key_settings.zoom.clone()) {
		return;
	}

	let scale = camera_settings.transform_scale_ring.next();
	match projection.into_inner().into_inner() {
		Projection::Orthographic(p2) => {
			p2.scale = scale;
		},
		_ => return
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
			app.add_systems(Update, cycle_zoom);
	}
}
