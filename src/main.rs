mod visuals;

use bevy::prelude::*;
use crate::visuals::camera::WorldCameraPlugin;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(WorldCameraPlugin)
		.run();
}

#[derive(Component)]
struct Player;
