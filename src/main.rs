use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_window::WindowMode;

// A unit struct to help identify the FPS UI component, since there may be many Text components
struct FpsText;
fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<(&mut Text, &FpsText)>) {
    for (mut text, _tag) in &mut query.iter() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
}

fn spawn_employees(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("assets/sprites/person.png").unwrap();
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            transform: Transform::from_translation(Vec3::new(-215.0, 0.0, 0.0)).with_scale(2.0),
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        .with(Employee {
            name: "Gerald".to_string(),
        });
}

struct GreetTimer(Timer);
fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    mut query: Query<(&Employee, &Position, &Velocity)>,
) {
    timer.0.tick(time.delta_seconds);
    if timer.0.finished {
        for (employee, position, velocity) in &mut query.iter() {
            println!("hello {}!", employee.name);
            println!("position {},{}!", position.0, position.1);
            println!("velocity {},{}!", velocity.0, velocity.1);
        }
    }
}

struct Position(f32, f32);
struct Velocity(f32, f32);
struct Employee {
    name: String,
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "The Ten Top".to_string(),
            width: 500,
            height: 300,
            vsync: true,
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_default_plugins()
        .add_startup_system(spawn_employees.system())
        .add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        .add_system(greet_people.system())
        .run();
}
