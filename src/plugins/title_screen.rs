use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

use crate::GameState;
pub struct TitleScreenPlugin;
struct TitleData {
    title_entity: Entity,
}

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(SystemSet::on_enter(GameState::TitleScreen).with_system(setup.system()))
            .add_system_set(
                SystemSet::on_update(GameState::TitleScreen)
                    .with_system(keyboard_input_system.system()),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::TitleScreen).with_system(teardown.system()),
            );
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let id = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::rgba(1.0, 1.0, 1.0, 0.0).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(300.0), Val::Px(150.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::ColumnReverse,
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
                                color: Color::rgb(0.0, 0.0, 0.0),
                            },
                            Default::default(),
                        ),
                        style: Style {
                            ..Default::default()
                        },
                        ..Default::default()
                    });

                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Press <Enter> to start!",
                            TextStyle {
                                font: asset_server.load("fonts/04B_03__.ttf"),
                                font_size: 16.0,
                                color: Color::rgb(0.0, 0.0, 0.0),
                                ..Default::default()
                            },
                            Default::default(),
                        ),
                        style: Style {
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        }).id();

    commands.insert_resource(TitleData {
        title_entity: id
    });
}

fn keyboard_input_system(mut state: ResMut<State<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        state.set(GameState::Playing).unwrap();
    }
}

fn teardown(mut commands: Commands, title_data: Res<TitleData>) {
    commands.entity(title_data.title_entity).despawn_recursive();
}
