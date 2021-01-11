use crate::{plugins::Destination, GameState, SCREEN_HEIGHT, SCREEN_WIDTH, STAGE};

use bevy::prelude::*;

#[derive(Debug)]
pub struct Actor {
    pub name: String,
}

#[derive(Debug)]
pub struct Velocity(pub f32, pub f32);

pub struct ActorAnimationTimer(Timer);

const LEFT_BOUND: f32 = (-SCREEN_WIDTH / 2.0) - 48.0;
const RIGHT_BOUND: f32 = (SCREEN_WIDTH / 2.0) + 48.0;
const TOP_BOUND: f32 = (SCREEN_HEIGHT / 2.0) + 48.0;
const BOTTOM_BOUND: f32 = (-SCREEN_HEIGHT / 2.0) - 48.0;

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ActorAnimationTimer(Timer::from_seconds(0.1, true)))
            .on_state_update(STAGE, GameState::Playing, animate_actor_sprites.system())
            .on_state_update(STAGE, GameState::Playing, move_actors.system())
            .on_state_update(STAGE, GameState::Playing, warp_around.system())
            .on_state_update(STAGE, GameState::Playing, move_to_destination.system());
    }
}

fn animate_actor_sprites(
    texture_atlases: Res<Assets<TextureAtlas>>,
    time: Res<Time>,
    mut timer: ResMut<ActorAnimationTimer>,
    mut query: Query<(
        &Actor,
        &Velocity,
        &mut Transform,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }

    for (_actor, velocity, mut transform, mut sprite, texture_atlas_handle) in query.iter_mut() {
        if velocity.0.abs() > 0.0 {
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

fn move_to_destination(
    commands: &mut Commands,
    mut query: Query<(Entity, &Actor, &Transform, &mut Velocity, &Destination)>,
) {
    for (entity, _actor, transform, mut velocity, destination) in query.iter_mut() {
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

fn move_actors(time: Res<Time>, mut query: Query<(&Actor, &mut Transform, &Velocity)>) {
    for (_actor, mut transform, velocity) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += time.delta_seconds() * velocity.0;
        translation.y += time.delta_seconds() * velocity.1;
    }
}

fn warp_around(mut query: Query<(&Actor, &mut Transform)>) {
    for (_actor, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        if translation.x < LEFT_BOUND {
            translation.x = RIGHT_BOUND;
        } else if translation.x > RIGHT_BOUND {
            translation.x = LEFT_BOUND;
        }

        if translation.y < BOTTOM_BOUND {
            translation.y = TOP_BOUND;
        } else if translation.y > TOP_BOUND {
            translation.y = BOTTOM_BOUND;
        }
    }
}
