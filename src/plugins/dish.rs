use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum DishType {
    HotDog,
}
#[derive(Clone, Copy)]
pub struct Dish(pub DishType);
pub struct DishPlugin;

impl Plugin for DishPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("sprites/hotdog.png");
    let mut transform = Transform::from_translation(Vec3::new(100.0, 100.0, 0.0));
    transform.scale = Vec3::splat(6.0);

    commands
        .spawn(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform,
            ..Default::default()
        })
        .with(Dish(DishType::HotDog));
}
