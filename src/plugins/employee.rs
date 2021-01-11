use crate::{
    plugins::{Actor, Velocity},
    GameState, STAGE,
};
use bevy::prelude::*;

pub struct EmployeePlugin;

pub struct Employee;

#[derive(Copy, Clone)]
pub struct Destination(pub Vec3);

impl Plugin for EmployeePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(STAGE, GameState::Playing, setup.system());
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
        .with(Actor {
            name: "Gerald".to_string(),
        })
        .with(Employee {})
        .with(Velocity(0.0, 0.0));
}
