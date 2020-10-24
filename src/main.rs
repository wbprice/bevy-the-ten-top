use bevy::prelude::*;
use bevy_window::WindowMode;

mod plugins;

use crate::plugins::EmployeePlugin;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "The Ten Top".to_string(),
            width: 500,
            height: 300,
            vsync: true,
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_default_plugins()
        .add_startup_system(setup_camera.system())
        .add_plugin(EmployeePlugin)
        .run();
}
