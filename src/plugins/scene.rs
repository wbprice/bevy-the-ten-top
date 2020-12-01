use bevy::prelude::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.register_component::<FloorTile>()
            .add_startup_system(setup.system())
            .add_system(render_kitchen.system());
    }
}

#[derive(Properties, Default)]
struct FloorTile {
    x: f32,
    y: f32,
}

fn setup(
    _commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
) {
    // Scenes are loaded just like any other asset.
    let scene_handle: Handle<DynamicScene> = asset_server.load("scenes/kitchen.scn");

    // SceneSpawner can "spawn" scenes. "Spawning" a scene creates a new instance of the scene in the World with new entity ids.
    // This guarantees that it will not overwrite existing entities.
    scene_spawner.spawn_dynamic(scene_handle);

    // This tells the AssetServer to watch for changes to assets.
    // It enables our scenes to automatically reload in game when we modify their files
    asset_server.watch_for_changes().unwrap();
}

fn render_kitchen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    query: Query<(Entity, Changed<FloorTile>)>,
) {
    for (index, (_entity, tile)) in query.iter().enumerate() {
        let texture_handle = asset_server.load("sprites/floor-tiles.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 2, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let mut transform = Transform::from_translation(Vec3::new(tile.x, tile.y, 0.0));
        transform.scale = Vec3::splat(1.0);

        commands
            .spawn(SpriteSheetComponents {
                texture_atlas: texture_atlas_handle,
                transform,
                sprite: TextureAtlasSprite {
                    index: index as u32 % 2,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(FloorTile {
                x: tile.x,
                y: tile.y,
            });
    }
}
