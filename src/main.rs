use bevy::prelude::*;
use bevy_window::WindowMode;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());
}

fn spawn_employees(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("assets/sprites/person.png").unwrap();
    commands
        .spawn(SpriteComponents {
            transform: Transform::from_translation(Vec3::new(-215.0, 0.0, 0.0)).with_scale(2.0),
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        .with(Employee {
            name: "Gerald".to_string(),
        })
        .spawn(SpriteComponents {
            transform: Transform::from_translation(Vec3::new(-180.0, 0.0, 0.0)).with_scale(2.0),
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        .with(Employee {
            name: "Julia".to_string(),
        });
}

struct GreetTimer(Timer);
fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    mut query: Query<(&Employee, &Transform)>,
) {
    timer.0.tick(time.delta_seconds);
    if timer.0.finished {
        for (employee, transform) in &mut query.iter() {
            let translation = transform.translation();
            println!("hello {}!", employee.name);
            println!("location {},{}!", translation.x(), translation.y());
        }
    }
}

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
        .add_startup_system(setup_camera.system())
        .add_startup_system(spawn_employees.system())
        .add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        .add_system(greet_people.system())
        .run();
}
