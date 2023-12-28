use bevy::prelude::*;

#[derive(Resource)]
pub struct MapConfig {
    pub path: String,
    pub map_scene_root: Option<Entity>,
}

pub fn map_init(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut map_config: ResMut<MapConfig>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    if map_config.is_changed() || map_config.is_added() {
        // despawn old map scene
        if let Some(map_scene_root) = map_config.map_scene_root.take() {
            commands.entity(map_scene_root).despawn_recursive();
        }
        // spawn new map scene
        let map_gltf: Handle<Scene> = assets.load(format!("{}#Scene0", map_config.path));
        map_config.as_mut().map_scene_root = Some(
            commands
                .spawn(SceneBundle {
                    scene: map_gltf,
                    ..Default::default()
                })
                .id(),
        );
        *ambient_light = AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        };
    }
}
