use bevy::{
    gltf::{Gltf, GltfMesh, GltfNode},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct ColliderFromMesh {
    pub gltf_handle: Handle<Gltf>,
}

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
        let map_gltf: Handle<Gltf> = assets.load(format!("{}", map_config.path));
        let map_scene: Handle<Scene> = assets.load(format!("{}#Scene0", map_config.path));
        map_config.as_mut().map_scene_root = Some(
            commands
                .spawn((
                    ColliderFromMesh {
                        gltf_handle: map_gltf,
                    },
                    SceneBundle {
                        scene: map_scene,
                        ..Default::default()
                    },
                ))
                .id(),
        );
        *ambient_light = AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        };
    }
}

pub fn create_collider_for_node(
    depth: u32,
    gltf: &Gltf,
    scene: &Handle<Scene>,
    node: &GltfNode,
    nodeo: &Handle<GltfNode>,
    commands: &mut Commands,
    gltfs: &Res<Assets<Gltf>>,
    gltf_meshes: &Res<Assets<GltfMesh>>,
    gltf_nodes: &Res<Assets<GltfNode>>,
    meshes: &Res<Assets<Mesh>>,
    q_meshes: &Query<(Entity, &Handle<Mesh>)>,
    q_gltf_scenes: &Query<&Handle<Scene>, With<ColliderFromMesh>>,
    q_parents: &Query<&Parent>,
) {
    match &node.mesh {
        Some(mesh) => {
            let gltf_mesh = gltf_meshes.get(mesh).unwrap();
            for prims in &gltf_mesh.primitives {
                let mesh = meshes.get(&prims.mesh).unwrap();
                let positions = mesh
                    .attributes()
                    .find(|a| a.0 == Mesh::ATTRIBUTE_POSITION.id)
                    .unwrap()
                    .1;
                let positions: Vec<_> = match positions {
                    bevy::render::mesh::VertexAttributeValues::Float32x3(values) => {
                        values.iter().map(|v| Vec3::from_array(*v)).collect()
                    }
                    _ => panic!("unexpected position format"),
                };

                let mesh_indices = mesh.indices().unwrap();
                let indices = mesh_indices
                    .iter()
                    .map(|v| v as u32)
                    .array_chunks::<3>()
                    .collect();
                let collider = Collider::trimesh(positions, indices);
                let scene = gltf.default_scene.as_ref().unwrap();

                for (ent, c_mesh) in q_meshes.iter() {
                    let mut has_gltf_ancestor = false;
                    for ancestor in q_parents.iter_ancestors(ent) {
                        let parent_scene_handle = match q_gltf_scenes.get(ancestor) {
                            Ok(ancestor) => ancestor,
                            Err(_) => continue,
                        };

                        if parent_scene_handle == scene {
                            has_gltf_ancestor = true;
                            break;
                        }
                    }
                    if !has_gltf_ancestor {
                        continue;
                    }

                    if c_mesh == &prims.mesh {
                        let mut ent_commands = commands.entity(ent);
                        ent_commands.insert(collider.clone());
                    }
                }
            }
        }
        None => (),
    }

    for child in node.children.iter() {
        create_collider_for_node(
            depth + 2,
            gltf,
            scene,
            child,
            nodeo,
            commands,
            gltfs,
            gltf_meshes,
            gltf_nodes,
            meshes,
            q_meshes,
            q_gltf_scenes,
            q_parents,
        );
    }
}

pub fn attach_colliders(
    mut commands: Commands,
    mut ev_asset: EventReader<AssetEvent<Gltf>>,
    gltfs: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    gltf_nodes: Res<Assets<GltfNode>>,
    meshes: Res<Assets<Mesh>>,
    q_meshes: Query<(Entity, &Handle<Mesh>)>,
    q_gltf_scenes: Query<&Handle<Scene>, With<ColliderFromMesh>>,
    q_parents: Query<&Parent>,
) {
    for ev in ev_asset.read() {
        match ev {
            AssetEvent::LoadedWithDependencies { id } => {
                let gltf = gltfs.get(*id).unwrap();
                let scene = gltf.default_scene.as_ref().unwrap();

                for nodeo in gltf.nodes.iter() {
                    let node = gltf_nodes.get(nodeo).unwrap();

                    create_collider_for_node(
                        0,
                        gltf,
                        scene,
                        node,
                        nodeo,
                        &mut commands,
                        &gltfs,
                        &gltf_meshes,
                        &gltf_nodes,
                        &meshes,
                        &q_meshes,
                        &q_gltf_scenes,
                        &q_parents,
                    );
                }
            }
            _ => (),
        }
    }
}
