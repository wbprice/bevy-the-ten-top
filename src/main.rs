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
        .with(Velocity(0.0, 0.0))
        .with(Destination(Vec3::new(100.0, 100.0, 0.0)))
        .spawn(SpriteComponents {
            transform: Transform::from_translation(Vec3::new(-180.0, 0.0, 0.0)).with_scale(2.0),
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        .with(Employee {
            name: "Julia".to_string(),
        })
        .with(Velocity(0.0, 0.0))
        .with(Destination(Vec3::new(100.0, 100.0, 0.0)));
}

fn move_employees(time: Res<Time>, mut query: Query<(&Employee, &mut Transform, &Velocity)>) {
    for (_employee, mut transform, velocity) in &mut query.iter() {
        let translation = transform.translation_mut();
        *translation.x_mut() += time.delta_seconds * velocity.0;
        *translation.y_mut() += time.delta_seconds * velocity.1;
    }
}

fn update_employee_sprites(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &Employee,
        &Velocity,
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (_employee, velocity, mut timer, mut sprite, texture_atlas_handle) in &mut query.iter() {

    }
}

fn move_to_destination(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut Velocity, &Destination)>,
) {
    for (entity, transform, mut velocity, destination) in &mut query.iter() {
        let translation = transform.translation();
        // How close is the entity to the destination?
        let close_enough = 4.0;
        let difference = translation - destination.0;
        let distance = difference.length();

        if distance < close_enough {
            commands.remove_one::<Destination>(entity);
            velocity.0 = 0.0;
            velocity.1 = 0.0;
        } else {
            let heading = (difference.y()).atan2(difference.x()) * 180.0 / 3.14;
            velocity.0 = 20.0 * heading.cos();
            velocity.1 = 20.0 * heading.sin();
        }
    }
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
struct Destination(Vec3);

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
        .add_system(move_employees.system())
        .add_system(move_to_destination.system())
        .run();
}
