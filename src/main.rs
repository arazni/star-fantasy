mod common;
mod visuals;

use bevy::prelude::*;
use crate::visuals::camera::WorldCameraPlugin;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
		.add_plugins(WorldCameraPlugin)
		.run();
}

#[derive(Component)]
struct Player;
