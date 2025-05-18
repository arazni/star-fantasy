mod common;
mod visuals;
mod inputs;

use bevy::{dev_tools::fps_overlay::FpsOverlayPlugin, prelude::*};
use crate::visuals::camera::WorldCameraPlugin;
use crate::inputs::input_settings::KeySettings;

fn main() {
	App::new()
		.insert_resource(KeySettings::default())
		.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
		.add_plugins(FpsOverlayPlugin::default())
		.add_plugins(WorldCameraPlugin)
		.run();
}

#[derive(Component)]
struct Player;
