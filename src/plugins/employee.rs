use crate::{
    plugins::{Actor, Velocity},
    GameState,
};
use bevy::prelude::*;

pub struct EmployeePlugin;

pub struct Employee;

impl Plugin for EmployeePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup.system()));
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
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform,
            ..Default::default()
        })
        .insert(Actor {
            name: "Gerald".to_string(),
        })
        .insert(Employee)
        .insert(Velocity(0.0, 0.0));
}
