use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion},
    input::ElementState,
    prelude::*,
    window::CursorMoved,
};

use crate::{GameState, SCREEN_HEIGHT, SCREEN_WIDTH};

const X_OFFSET: f32 = SCREEN_WIDTH / 2.0;
const Y_OFFSET: f32 = SCREEN_HEIGHT / 2.0;

pub struct MousePlugin;
pub struct MouseTile;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(follow_mouse.system()),
            );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/hotdog.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    transform.scale = Vec3::splat(2.0);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform,
            ..Default::default()
        })
        .insert(MouseTile);
}

fn follow_mouse(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut commands: Commands,
    mut query: Query<(&MouseTile, &mut Transform)>,
) {
    for event in cursor_moved_events.iter() {
        for (_mouse_tile, mut transform) in query.iter_mut() {
            // translate event coordinates to transform coordinates
            // The origin for events is relative to the lower left corner
            // The origin for transform is relative to the center of the screen

            // The cursor should snap in 48 x 48 chunks
            let x_pos = ((event.position.x - X_OFFSET) / 48.0).floor() * 48.0 + 24.0;
            let y_pos = ((event.position.y - Y_OFFSET) / 48.0).floor() * 48.0 + 24.0;

            transform.translation.x = x_pos;
            transform.translation.y = y_pos;

            for event in mouse_button_input_events.iter() {
                match event.button {
                    MouseButton::Left => match event.state {
                        ElementState::Pressed => {
                            dbg!(event);
                            dbg!(x_pos);
                            dbg!(y_pos);
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
}
