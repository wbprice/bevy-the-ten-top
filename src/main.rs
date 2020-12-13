use bevy::prelude::*;
use bevy_window::WindowMode;

mod plugins;
use crate::plugins::{DishPlugin, EmployeePlugin, ScenePlugin, SidebarPlugin, TasksPlugin};

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "The Ten Top".to_string(),
            width: 768,
            height: 432,
            vsync: true,
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_startup_system(setup_camera.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(EmployeePlugin)
        .add_plugin(DishPlugin)
        .add_plugin(TasksPlugin)
        .add_plugin(ScenePlugin)
        .add_plugin(SidebarPlugin)
        .run();
}
