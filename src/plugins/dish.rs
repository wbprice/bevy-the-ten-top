use bevy::prelude::*;

enum DishType {
    HotDog,
}
struct Dish(DishType);
pub struct DishPlugin;

impl Plugin for DishPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("sprites/hotdog.png");
    let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    transform.scale = Vec3::splat(3.0);

    commands
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            transform,
            ..Default::default()
        })
        .with(Dish(DishType::HotDog));
}
