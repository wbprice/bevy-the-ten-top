use crate::{
    plugins::{Craving, Destination, DishType, Fullness, Patron, Task, Tasks},
    GameState, STAGE,
};
use bevy::prelude::*;

pub struct CashRegisterPlugin;
impl Plugin for CashRegisterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(STAGE, GameState::Playing, setup.system())
            .on_state_update(STAGE, GameState::Playing, attract_patrons.system());
    }
}

pub struct CashRegister;
struct Attracted;
pub struct Menu(Vec<DishType>);

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/cash-register.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, 1.0));
    transform.scale = Vec3::splat(3.0);
    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform,
            ..Default::default()
        })
        .with(CashRegister)
        .with(Menu(vec![DishType::HotDog]));
}

fn attract_patrons(
    commands: &mut Commands,
    register_query: Query<(Entity, &CashRegister, &Menu, &Transform)>,
    mut patron_query: Query<(Entity, &Patron, &Fullness, &Craving, &Transform), Without<Attracted>>,
) {
    for (register, _register, menu, register_transform) in register_query.iter() {
        for (patron, _patron, fullness, craving, patron_transform) in patron_query.iter_mut() {
            if menu.0.contains(&craving.0) {
                if (register_transform.translation - patron_transform.translation).length() < 256.0
                {
                    if fullness.0 < 75.0 {
                        commands.insert_one(
                            patron,
                            Task::new(Tasks::RequestOrder(craving.0, register)),
                        );
                        commands.insert_one(patron, Attracted);
                    }
                }
            }
        }
    }
}
