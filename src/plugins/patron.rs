use crate::{
    plugins::{Actor, DishType, Velocity},
    GameState, STAGE,
};
use bevy::prelude::*;
pub struct PatronPlugin;
pub struct Patron;

#[derive(Debug)]
pub struct Fullness(pub f32);
pub struct Craving(pub DishType);
struct FullnessTimer(Timer);

impl Plugin for PatronPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(FullnessTimer(Timer::from_seconds(1.0, true)))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(decrement_fullness.system()),
            );
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

    let mut transform = Transform::from_translation(Vec3::new(-215.0, -100.0, 0.0));
    transform.scale = Vec3::splat(3.0);
    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform,
            ..Default::default()
        })
        .insert(Actor {
            name: "Susan".to_string(),
        })
        .insert(Patron)
        .insert(Craving(DishType::HotDog))
        .insert(Fullness(50.0))
        .insert(Velocity(96.0, 0.0));
}

fn decrement_fullness(
    time: Res<Time>,
    mut timer: ResMut<FullnessTimer>,
    mut query: Query<(Entity, &mut Fullness)>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    for (_entity, mut fullness) in query.iter_mut() {
        fullness.0 -= 5.0;
    }
}
