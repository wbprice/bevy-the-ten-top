use bevy::prelude::*;
use bevy_window::WindowMode;

mod plugins;
use crate::plugins::{
    ActorPlugin, CashRegisterPlugin, DishPlugin, EmployeePlugin, IngredientPlugin, MousePlugin,
    PatronPlugin, TasksPlugin, TitleScreenPlugin,
};

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

pub const STAGE: &str = "game_state";
pub const SCREEN_WIDTH: f32 = 768.0;
pub const SCREEN_HEIGHT: f32 = 432.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum GameState {
    TitleScreen,
    Playing,
    // Paused
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "The Ten Top".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            vsync: true,
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_startup_system(setup_camera.system())
        .add_state(GameState::TitleScreen)
        .add_plugins(DefaultPlugins)
        .add_plugin(EmployeePlugin)
        .add_plugin(PatronPlugin)
        .add_plugin(DishPlugin)
        .add_plugin(IngredientPlugin)
        .add_plugin(TasksPlugin)
        .add_plugin(TitleScreenPlugin)
        .add_plugin(CashRegisterPlugin)
        .add_plugin(ActorPlugin)
        .add_plugin(MousePlugin)
        .run();
}
