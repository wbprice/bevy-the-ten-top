use crate::GameState;
use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DishType {
    HotDog,
}
#[derive(Clone, Copy)]
pub struct Dish(pub DishType);
pub struct DishPlugin;

impl Plugin for DishPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup.system()));
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("sprites/hotdog.png");
    let mut transform = Transform::from_translation(Vec3::new(100.0, 100.0, 0.0));
    transform.scale = Vec3::splat(3.0);

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform,
            ..Default::default()
        })
        .insert(Dish(DishType::HotDog));
}
