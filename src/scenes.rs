use bevy::{prelude::*, scene::{InstanceId, SceneInstance}};

pub struct ScenesExample;

impl Plugin for ScenesExample {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_scene);
        app.init_resource::<SceneData>();
        app.add_system(spawn_scene_bundle);
        app.add_startup_system(load_gltf);
        app.add_system(gltf_scene);
        app.add_system(save_scene);
        app.add_startup_system(load_scene);
    }
}

struct SceneData([Handle<Scene>; 10]);
struct DynamicSceneData(Handle<DynamicScene>);
struct GltfHandle(Handle<bevy::gltf::Gltf>);

impl FromWorld for SceneData {
    fn from_world(world: &mut World) -> Self {
        let mesh_handle = world.resource_mut::<Assets<Mesh>>().add(shape::Box::new(1., 1., 1.).into());
        let mut assets = world.resource_mut::<Assets<Scene>>();
        let mut data = SceneData(Default::default());
        for i in 0..10 {
            let mut new_world = World::new();
            new_world.spawn().insert_bundle(PbrBundle {
                mesh: mesh_handle.clone(),
                transform: Transform::from_translation(Vec3 { x: (i/2) as f32, y: (i%3) as f32, z: ((i * 2) % 5) as f32 }),
                ..Default::default()
            });
            let s = Scene {world: new_world};
            let sh = assets.add(s);
            data.0[i] = sh;
        }
        data
    }
}

impl FromWorld for DynamicSceneData {
    fn from_world(world: &mut World) -> Self {
        let scene_handle = world.resource::<SceneData>();
        let scenes = world.resource::<Assets<Scene>>();
        let scene = scenes.get(&scene_handle.0[0]).unwrap();
        let type_registry = world.resource::<bevy::reflect::TypeRegistry>();
        let from_scene = DynamicScene::from_scene(scene, type_registry);
        let dyn_scene_handle = world.resource_mut::<Assets<DynamicScene>>()
            .add(from_scene);
        DynamicSceneData(dyn_scene_handle)
    }
}

fn save_scene(
    input: Res<Input<KeyCode>>,
    scene: Res<DynamicSceneData>,
    scenes: Res<Assets<DynamicScene>>,
    type_registry: Res<bevy::reflect::TypeRegistry>,
) {
    if !input.just_pressed(KeyCode::PageDown) {return;}
    let string = if let Some(scene) = scenes.get(&scene.0) {
        scene.serialize_ron(&type_registry).unwrap()
    } else {return;};
    let _ = std::fs::write("./assets/scene.scn.ron", string);
}

fn load_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let _ = asset_server.watch_for_changes();
    commands.spawn_bundle(DynamicSceneBundle {
        scene: asset_server.load("scene.scn.ron"),
        ..Default::default()
    });
}

fn spawn_scene(
    input: Res<Input<KeyCode>>,
    mut scene_spawner: ResMut<SceneSpawner>,
    scenes: Res<SceneData>,
    mut instance: Local<[Vec<InstanceId>;10]>
) {
    for key in input.get_just_pressed() {
        let pressed = match key {
            KeyCode::Numpad0 => {
                0
            },
            KeyCode::Numpad1 => {
                1
            },
            KeyCode::Numpad2 => {
                2
            },
            KeyCode::Numpad3 => {
                3
            },
            KeyCode::Numpad4 => {
                4
            },
            KeyCode::Numpad5 => {
                5
            },
            KeyCode::Numpad6 => {
                6
            },
            KeyCode::Numpad7 => {
                7
            },
            KeyCode::Numpad8 => {
                8
            },
            KeyCode::Numpad9 => {
                9
            },
            _ => {10}
        };
        if pressed < 10 {
            if input.pressed(KeyCode::S) {
                instance[pressed].push(scene_spawner.spawn(scenes.0[pressed].clone()));
            } else if input.pressed(KeyCode::R) {
                for id in instance[pressed].iter() {
                    scene_spawner.despawn_instance(*id);
                }
                instance[pressed].clear();
            }
        }
    }
}

fn spawn_scene_bundle(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    loaded: Query<Entity,With<SceneInstance>>,
    scenes: Res<SceneData>,
) {
    if input.just_pressed(KeyCode::E) {
        for scene in loaded.iter() {
            commands.entity(scene).despawn_recursive();
        }
        return;
    }
    for key in input.get_just_pressed() {
        let pressed = match key {
            KeyCode::Numpad0 => {
                0
            },
            KeyCode::Numpad1 => {
                1
            },
            KeyCode::Numpad2 => {
                2
            },
            KeyCode::Numpad3 => {
                3
            },
            KeyCode::Numpad4 => {
                4
            },
            KeyCode::Numpad5 => {
                5
            },
            KeyCode::Numpad6 => {
                6
            },
            KeyCode::Numpad7 => {
                7
            },
            KeyCode::Numpad8 => {
                8
            },
            KeyCode::Numpad9 => {
                9
            },
            _ => {10}
        };
        if pressed < 10 {
            if input.pressed(KeyCode::A) {
                commands.spawn_bundle(
                    SceneBundle {
                        scene: scenes.0[pressed].clone(),
                        ..Default::default()
                    }
                );
            } else if input.pressed(KeyCode::D) {
                commands.spawn_bundle(SpatialBundle::default()).insert(scenes.0[pressed].clone());
            }
        }
    }
}

fn load_gltf(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(GltfHandle(asset_server.load("gltf.gltf")));
}

fn gltf_scene(
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut instance: Local<Option<[InstanceId;2]>>,
    gltf_assets: Res<Assets<bevy::gltf::Gltf>>,
    gltf_handle: Res<GltfHandle>,
) {
    if input.just_pressed(KeyCode::NumpadMultiply) {
        if instance.is_some() {return;}
        if let Some(gltf) = gltf_assets.get(&gltf_handle.0) {
            *instance = Some([
                scene_spawner.spawn(gltf.scenes[0].clone()),
                scene_spawner.spawn(asset_server.load("gltf.gltf#Scene1"))
            ]);
        }
    } else if input.just_pressed(KeyCode::NumpadDivide) {
        if let Some(v) = *instance {
            scene_spawner.despawn_instance(v[0]);
            scene_spawner.despawn_instance(v[1]);
            *instance = None;
        }
    }
}