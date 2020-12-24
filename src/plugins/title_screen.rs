use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

use crate::{GameState, STAGE};
pub struct TitleScreenPlugin;
struct TitleData {
    title_entity: Entity,
}

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(STAGE, GameState::TitleScreen, setup.system())
            .on_state_update(
                STAGE,
                GameState::TitleScreen,
                keyboard_input_system.system(),
            )
            .on_state_exit(STAGE, GameState::TitleScreen, teardown.system());
    }
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(CameraUiBundle::default())
        .spawn(NodeBundle {
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
                .spawn(NodeBundle {
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
                    parent.spawn(TextBundle {
                        text: Text {
                            value: "The Ten Top".to_string(),
                            font: asset_server.load("fonts/04B_03__.ttf"),
                            style: TextStyle {
                                font_size: 24.0,
                                color: Color::rgb(0.0, 0.0, 0.0),
                                ..Default::default()
                            },
                        },
                        ..Default::default()
                    });
                    parent.spawn(TextBundle {
                        text: Text {
                            value: "Press <Enter> to start!".to_string(),
                            font: asset_server.load("fonts/04B_03__.ttf"),
                            style: TextStyle {
                                font_size: 16.0,
                                color: Color::rgb(0.0, 0.0, 0.0),
                                ..Default::default()
                            },
                        },
                        ..Default::default()
                    });
                });
        });

    commands.insert_resource(TitleData {
        title_entity: commands.current_entity().unwrap(),
    });
}

fn keyboard_input_system(mut state: ResMut<State<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        state.set_next(GameState::Playing).unwrap();
    }
}

fn teardown(commands: &mut Commands, title_data: Res<TitleData>) {
    commands.despawn_recursive(title_data.title_entity);
}
