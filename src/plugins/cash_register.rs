use crate::{
    plugins::{Craving, DishType, Fullness, Patron, Task, TaskVariants},
    GameState,
};
use bevy::prelude::*;

pub struct CashRegisterPlugin;
impl Plugin for CashRegisterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(attract_patrons.system()),
            );
    }
}

pub struct CashRegister;
struct Attracted;
pub struct Menu(Vec<DishType>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/cash-register.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, 1.0));
    transform.scale = Vec3::splat(3.0);
    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform,
            ..Default::default()
        })
        .insert(CashRegister)
        .insert(Menu(vec![DishType::HotDog]));
}

fn attract_patrons(
    mut commands: Commands,
    register_query: Query<(Entity, &CashRegister, &Menu, &Transform)>,
    mut patron_query: Query<(Entity, &Patron, &Fullness, &Craving, &Transform), Without<Attracted>>,
) {
    for (register, _register, menu, register_transform) in register_query.iter() {
        for (patron, _patron, fullness, craving, patron_transform) in patron_query.iter_mut() {
            if menu.0.contains(&craving.0) {
                if (register_transform.translation - patron_transform.translation).length() < 256.0
                {
                    if fullness.0 < 75.0 {
                        commands
                            .entity(patron)
                            .insert(Task::new(TaskVariants::GoToEntity(register)));
                        commands.entity(patron).insert(Attracted);
                    }
                }
            }
        }
    }
}
