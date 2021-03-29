use crate::{GameState, STAGE};
use bevy::prelude::*;

pub struct Building {
    pub x: f32,
    pub y: f32,
}

pub enum BuildingType {
    Wall,
    Floor,
}

pub struct BuildingPlugin;
#[derive(Default)]
pub struct BuildingQueue(pub Vec<(Building, BuildingType)>);

pub struct Wall;
pub struct Floor;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(BuildingQueue(vec![]))
            .on_state_enter(STAGE, GameState::Playing, setup.system())
            .on_state_update(STAGE, GameState::Playing, add_buildings.system());
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/person-run-cycle.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 6, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut transform = Transform::from_translation(Vec3::new(-215.0, -100.0, 0.0));
    transform.scale = Vec3::splat(3.0);

    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform,
        ..Default::default()
    });
}

fn add_buildings(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut building_queue: ResMut<BuildingQueue>,
    query: Query<&Building>,
) {
    if let Some((building, building_type)) = building_queue.0.pop() {
        // There can only be one building at a location
        for (b) in query.iter() {
            if building.x == b.x && building.y == b.y {
                dbg!("redundant");
                return;
            }
        }

        let texture_handle = asset_server.load("sprites/wall-tiles.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 1, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let mut transform = Transform::from_translation(Vec3::new(building.x, building.y, 0.0));

        transform.scale = Vec3::splat(2.0);
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform,
                ..Default::default()
            })
            .with(Wall);
    }
}
