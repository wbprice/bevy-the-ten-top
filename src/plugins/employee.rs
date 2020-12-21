use crate::{
    GameState, STAGE,
    plugins::{DishType, Task, Tasks}
};
use bevy::prelude::*;

pub struct EmployeePlugin;

struct Velocity(f32, f32);

#[derive(Debug)]
pub struct Employee {
    name: String,
}

struct EmployeeAnimationTimer(Timer);

#[derive(Copy, Clone)]
pub struct Destination(pub Vec3);

impl Plugin for EmployeePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(EmployeeAnimationTimer(Timer::from_seconds(0.1, true)))
            .on_state_enter(STAGE, GameState::Playing, setup.system())
            .on_state_update(STAGE, GameState::Playing, animate_sprite_system.system())
            .on_state_update(STAGE, GameState::Playing, move_employees.system())
            .on_state_update(STAGE, GameState::Playing, move_to_destination.system());
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/person-run-cycle.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 6, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut transform = Transform::from_translation(Vec3::new(-215.0, 0.0, 0.0));
    transform.scale = Vec3::splat(3.0);
    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform,
            ..Default::default()
        })
        .with(Employee {
            name: "Gerald".to_string(),
        })
        .with(Velocity(0.0, 0.0))
        .with(Task::new(Tasks::FindDish(DishType::HotDog)));
}

fn animate_sprite_system(
    texture_atlases: Res<Assets<TextureAtlas>>,
    time: Res<Time>,
    mut timer: ResMut<EmployeeAnimationTimer>,
    mut query: Query<(
        &Velocity,
        &mut Transform,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (velocity, mut transform, mut sprite, texture_atlas_handle) in query.iter_mut() {
        if !timer.0.tick(time.delta_seconds()).just_finished() {
            return;
        }

        if velocity.1.abs() > 0.0 {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        } else {
            sprite.index = 0;
        }
        if velocity.0 < 0.0 {
            transform.scale = Vec3::new(-3.0, 3.0, 1.0);
        } else {
            transform.scale = Vec3::new(3.0, 3.0, 1.0);
        }
    }
}

fn move_employees(time: Res<Time>, mut query: Query<(&Employee, &mut Transform, &Velocity)>) {
    for (_employee, mut transform, velocity) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += time.delta_seconds() * velocity.0;
        translation.y += time.delta_seconds() * velocity.1;
    }
}

fn move_to_destination(
    commands: &mut Commands,
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
            let heading = (difference.y.atan2(difference.x)) * 180.0 / 3.14;
            velocity.0 = 50.0 * heading.cos();
            velocity.1 = 50.0 * heading.sin();
        }
    }
}
