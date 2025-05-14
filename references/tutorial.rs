use bevy::prelude::*;

#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

fn add_people(mut commands: Commands) {
	commands.spawn((Person, Name("Elaina Proctor".to_string())));
	commands.spawn((Person, Name("Renzo Hume".to_string())));
	commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
	if timer.0.tick(time.delta()).just_finished() {
		for name in &query {
			println!("hello {}!", name.0);
		}
	}
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
	for mut name in &mut query {
		if name.0 == "Renzo Hume" {
			name.0 = "Renzo Proctor".to_string();
			break;
		}
	}	
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
	fn build(&self, app: &mut App) {
			app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
			app.add_systems(Startup, add_people);
			app.add_systems(Update, (update_people, greet_people).chain());
	}
}

	// let move_delta = (direction * camera_setting.tile_size.f / trigger.steps.f)
	// 	.extend(0.);
	// movable.movement_state = MovementState::Moving;
	// let mut animation_time = Time::from_duration(time.delta());

	// if trigger.orientation == Orientation::Down || trigger.orientation == Orientation::Up {
	// 	texture_atlas.index = if trigger.orientation == Orientation::Down { SPRITE_DOWN_INDEX }
	// 		else { SPRITE_UP_INDEX };

	// 	while step < trigger.steps.u {
	// 		let now = Instant::now();
	// 		thread::sleep(Duration::from_secs_f32(0.1));
	// 		trigger.timer.tick(now.elapsed());
	// 		if trigger.timer.just_finished() {
	// 			sprite.flip_x = !sprite.flip_x;
	// 			step += 1;
	// 			transform.translation += move_delta;
	// 		}
	// 	}
	// } else {
	// 	while step < trigger.steps.u {
	// 		trigger.timer.tick(time.delta());
	// 		if trigger.timer.just_finished() {
	// 			texture_atlas.index = if texture_atlas.index == SPRITE_LEFT1_INDEX { SPRITE_LEFT2_INDEX }
	// 				else { SPRITE_LEFT1_INDEX };
	// 			step += 1;
	// 			transform.translation += move_delta;
	// 		}
	// 	}
	// }

	// animation.add_curve_to_target(
	// 	animation_target_id,
	// 	AnimatableCurve::new(
	// 		animated_field!(Transform::translation),
	// 		SampleAutoCurve::new(
	// 			Interval::new(0., 1.).expect("appropriate interval in on_move"),
	// 			[
	// 				Vec3::ZERO,
	// 				move_delta.extend(0.)
	// 			]).expect("appropriate samples in on_move")
	// 		));