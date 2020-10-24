use bevy::prelude::*;
use bevy_window::WindowMode;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());
}

fn spawn_employees(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server
        .load("assets/sprites/person-run-cycle.png")
        .unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(144.0, 24.0), 6, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(Vec3::new(-215.0, 0.0, 0.0)).with_scale(5.0),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true))
        .with(Employee {
            name: "Gerald".to_string(),
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

fn animate_employees(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &Velocity,
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (velocity, timer, mut sprite, texture_atlas_handle) in &mut query.iter() {
        if timer.finished {
            // If the person is moving, show the moving animation
            if velocity.1.abs() > 0.0 {
                let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
                sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
            // Otherwise, idle
            } else {
                sprite.index = 0;
            }
        }
    }
}

fn move_to_destination(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut Velocity, &Destination)>,
) {
    for (entity, transform, mut velocity, destination) in &mut query.iter() {
        let translation = transform.translation();
        // How close is the entity to the destination?
        let close_enough = 32.0;
        let difference = translation - destination.0;
        let distance = difference.length();

        if distance < close_enough {
            commands.remove_one::<Destination>(entity);
            velocity.0 = 0.0;
            velocity.1 = 0.0;
        } else {
            let heading = (difference.y()).atan2(difference.x()) * 180.0 / 3.14;
            velocity.0 = 50.0 * heading.cos();
            velocity.1 = 50.0 * heading.sin();
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
        .add_system(move_employees.system())
        .add_system(move_to_destination.system())
        .add_system(animate_employees.system())
        .run();
}
