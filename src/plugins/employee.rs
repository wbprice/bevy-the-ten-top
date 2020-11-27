use crate::plugins::{DishType, Task, Tasks};
use bevy::prelude::*;

pub struct EmployeePlugin;

struct Velocity(f32, f32);
#[derive(Debug)]
pub struct Employee {
    name: String,
}
#[derive(Copy, Clone)]
pub struct Destination(pub Vec3);

impl Plugin for EmployeePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(animate_sprite_system.system())
            .add_system(move_employees.system())
            .add_system(move_to_destination.system());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/person-run-cycle.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 6, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut transform = Transform::from_translation(Vec3::new(-215.0, 0.0, 0.0));
    transform.scale = Vec3::splat(3.0);
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform,
            ..Default::default()
        })
        .with(Employee {
            name: "Gerald".to_string(),
        })
        .with(Velocity(0.0, 0.0))
        .with(Task::new(Tasks::GoToDish(DishType::HotDog)))
        .with(Timer::from_seconds(0.1, true));
}

fn animate_sprite_system(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &Velocity,
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (velocity, timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        if timer.finished {
            if velocity.1.abs() > 0.0 {
                let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
            } else {
                sprite.index = 0;
            }
        }
    }
}

fn move_employees(time: Res<Time>, mut query: Query<(&Employee, &mut Transform, &Velocity)>) {
    for (_employee, mut transform, velocity) in query.iter_mut() {
        let translation = &mut transform.translation;
        *translation.x_mut() += time.delta_seconds * velocity.0;
        *translation.y_mut() += time.delta_seconds * velocity.1;
    }
}

fn move_to_destination(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut Velocity, &Destination)>,
) {
    for (entity, transform, mut velocity, destination) in query.iter_mut() {
        let translation = transform.translation;
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
