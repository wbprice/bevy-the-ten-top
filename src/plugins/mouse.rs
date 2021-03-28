use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    input::ElementState,
    prelude::*,
    window::CursorMoved,
};

use crate::{
    plugins::{Building, BuildingQueue, BuildingType},
    GameState, STAGE, X_SCREEN_OFFSET, Y_SCREEN_OFFSET,
};

pub struct MousePlugin;
pub struct MouseTile;

#[derive(Default)]
struct MouseState {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    mouse_motion_event_reader: EventReader<MouseMotion>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
    mouse_wheel_event_reader: EventReader<MouseWheel>,
}

impl Plugin for MousePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(STAGE, GameState::Playing, setup.system())
            .on_state_update(STAGE, GameState::Playing, follow_mouse.system());
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/hotdog.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    transform.scale = Vec3::splat(2.0);

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform,
            ..Default::default()
        })
        .with(MouseTile);
}

fn follow_mouse(
    mut state: Local<MouseState>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    commands: &mut Commands,
    mut query: Query<(&MouseTile, &mut Transform)>,
    mut building_queue: ResMut<BuildingQueue>,
) {
    for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
        for (_mouse_tile, mut transform) in query.iter_mut() {
            // translate event coordinates to transform coordinates
            // The origin for events is relative to the lower left corner
            // The origin for transform is relative to the center of the screen

            // The cursor should snap in 48 x 48 chunks
            let x_pos = ((event.position.x - X_SCREEN_OFFSET) / 48.0).floor() * 48.0 + 24.0;
            let y_pos = ((event.position.y - Y_SCREEN_OFFSET) / 48.0).floor() * 48.0 + 24.0;

            transform.translation.x = x_pos;
            transform.translation.y = y_pos;

            for event in state
                .mouse_button_event_reader
                .iter(&mouse_button_input_events)
            {
                match event.button {
                    MouseButton::Left => match event.state {
                        ElementState::Pressed => {
                            building_queue
                                .0
                                .push((Building { x: x_pos, y: y_pos }, BuildingType::Wall));
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
}
