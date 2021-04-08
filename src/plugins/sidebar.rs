use bevy::prelude::*;

use crate::{GameState};
pub struct SidebarPlugin;

impl Plugin for SidebarPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup.system()));
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: materials.add(Color::rgb(1.00, 1.0, 1.0).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(176.0), Val::Percent(100.0)),
                        position_type: PositionType::Relative,
                        position: Rect {
                            top: Val::Px(12.0),
                            left: Val::Px(12.0),
                            ..Default::default()
                        },
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::FlexStart,
                        justify_content: JustifyContent::FlexStart,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "The Ten Top",
                            TextStyle {
                                font: asset_server.load("fonts/04B_03__.ttf"),
                                font_size: 24.0,
                                color: Color::BLACK,
                            },
                            Default::default(),
                        ),
                        style: Style {
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        });
}

struct StatusText;

// fn update_employee_status(mut commands)
