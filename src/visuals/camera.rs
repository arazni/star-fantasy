use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn(Camera2d);

	commands.spawn((
		Sprite::from_image(
			asset_server.load("characters/soldier-neutral-down.png")
		),
		Transform::from_scale(Vec3::new(4.,4.,1.))
	));
}
