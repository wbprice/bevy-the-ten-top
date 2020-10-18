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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("assets/fonts/FiraSans-Bold.ttf").unwrap();
    commands
        // 2d camera
        .spawn(UiCameraComponents::default())
        // texture
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "FPS:".to_string(),
                font: font_handle,
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            },
            ..Default::default()
        })
        .with(FpsText);
}

fn spawn_employees(mut commands: Commands) {
    commands
        .spawn((
            Employee {
                name: "Gerald".to_string(),
            },
            Position(0.0, 0.0),
            Velocity(0.0, 0.0),
        ))
        .spawn((
            Employee {
                name: "Julie".to_string(),
            },
            Position(244.0, 244.0),
            Velocity(0.0, 0.0),
        ));
}

struct GreetTimer(Timer);
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, mut query: Query<(&Employee, &Position, &Velocity)>) {
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
        .add_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_startup_system(spawn_employees.system())
        .add_system(text_update_system.system())
        .add_system(greet_people.system())
        .run();
}
