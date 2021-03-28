use bevy::prelude::*;
use bevy_window::WindowMode;

mod plugins;
use crate::plugins::{
    ActorPlugin, CashRegisterPlugin, DishPlugin, EmployeePlugin, MousePlugin, PatronPlugin,
    TasksPlugin, TitleScreenPlugin, BuildingPlugin
};

fn setup_camera(commands: &mut Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub const STAGE: &str = "game_state";
pub const SCREEN_WIDTH: f32 = 768.0;
pub const SCREEN_HEIGHT: f32 = 432.0;
pub const X_SCREEN_OFFSET: f32 = SCREEN_WIDTH / 2.0;
pub const Y_SCREEN_OFFSET: f32 = SCREEN_HEIGHT / 2.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameState {
    TitleScreen,
    Playing,
    // Paused
}

fn main() {
    App::build()
        .add_resource(State::new(GameState::TitleScreen))
        .add_stage_after(stage::UPDATE, STAGE, StateStage::<GameState>::default())
        .add_resource(WindowDescriptor {
            title: "The Ten Top".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            vsync: true,
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_startup_system(setup_camera.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(EmployeePlugin)
        .add_plugin(PatronPlugin)
        .add_plugin(DishPlugin)
        .add_plugin(TasksPlugin)
        .add_plugin(TitleScreenPlugin)
        .add_plugin(CashRegisterPlugin)
        .add_plugin(ActorPlugin)
        .add_plugin(MousePlugin)
        .add_plugin(BuildingPlugin)
        .run();
}
