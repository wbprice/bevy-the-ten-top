use bevy::prelude::*;
use bevy_window::WindowMode;

mod plugins;
use crate::plugins::{DishPlugin, EmployeePlugin, PatronPlugin, TasksPlugin, TitleScreenPlugin};

fn setup_camera(commands: &mut Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub const STAGE: &str = "game_state";
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
            width: 768.0,
            height: 432.0,
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
        .run();
}
