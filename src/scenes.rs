use bevy::{
    prelude::*,
    scene::{InstanceId, SceneInstance},
};

pub struct ScenesExample;

impl Plugin for ScenesExample {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_scene);
        app.init_resource::<SceneData>();
        app.init_resource::<DynamicSceneData>();
        app.add_system(spawn_scene_bundle);
        app.add_startup_system(load_gltf);
        app.add_system(gltf_scene);
        app.add_system(save_scene);
        app.add_startup_system(load_scene);
        app.add_system(simple_save_scene)
            .add_system(advanced_save_scene);
        app.add_system(spawn_scene_cube)
        .add_system(move_scene_cube);
    }
}

#[derive(Resource)]
struct SceneData([Handle<Scene>; 10]);
#[derive(Resource)]
struct DynamicSceneData(Handle<DynamicScene>);
#[derive(Resource)]
struct GltfHandle(Handle<bevy::gltf::Gltf>);

impl FromWorld for SceneData {
    fn from_world(world: &mut World) -> Self {
        let mesh_handle = world
            .resource_mut::<Assets<Mesh>>()
            .add(shape::Box::new(1., 1., 1.).into());
        let mut assets = world.resource_mut::<Assets<Scene>>();
        let mut data = SceneData(Default::default());
        for i in 0..10 {
            let mut new_world = World::new();
            new_world.spawn(PbrBundle {
                mesh: mesh_handle.clone(),
                transform: Transform::from_translation(Vec3 {
                    x: (i / 2) as f32,
                    y: (i % 3) as f32,
                    z: ((i * 2) % 5) as f32,
                }),
                ..Default::default()
            });
            let s = Scene { world: new_world };
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
        let type_registry = world.resource::<bevy::prelude::AppTypeRegistry>();
        let from_scene = DynamicScene::from_scene(scene, type_registry);
        let dyn_scene_handle = world.resource_mut::<Assets<DynamicScene>>().add(from_scene);
        DynamicSceneData(dyn_scene_handle)
    }
}

fn save_scene(
    input: Res<Input<KeyCode>>,
    scene: Res<DynamicSceneData>,
    scenes: Res<Assets<DynamicScene>>,
    type_registry: Res<AppTypeRegistry>,
) {
    if !input.just_pressed(KeyCode::PageDown) {
        return;
    }
    info!("saving scene");
    let string = if let Some(scene) = scenes.get(&scene.0) {
        scene.serialize_ron(&type_registry).unwrap()
    } else {
        error!("failed to get scene");
        return;
    };
    std::fs::write("./assets/scene.scn.ron", string).unwrap();
}

fn load_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let id = commands
        .spawn(DynamicSceneBundle {
            scene: asset_server.load("part_scene.scn.ron"),
            ..Default::default()
        })
        .id();
    info!("loading scene on {:?}", id);
}

fn spawn_scene(
    input: Res<Input<KeyCode>>,
    mut scene_spawner: ResMut<SceneSpawner>,
    scenes: Res<SceneData>,
    mut instance: Local<[Vec<InstanceId>; 10]>,
) {
    for key in input.get_just_pressed() {
        let pressed = match key {
            KeyCode::Numpad0 => 0,
            KeyCode::Numpad1 => 1,
            KeyCode::Numpad2 => 2,
            KeyCode::Numpad3 => 3,
            KeyCode::Numpad4 => 4,
            KeyCode::Numpad5 => 5,
            KeyCode::Numpad6 => 6,
            KeyCode::Numpad7 => 7,
            KeyCode::Numpad8 => 8,
            KeyCode::Numpad9 => 9,
            _ => 10,
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
    loaded: Query<Entity, With<SceneInstance>>,
    scenes: Res<SceneData>,
) {
    if input.just_pressed(KeyCode::C) {
        for scene in loaded.iter() {
            commands.entity(scene).despawn_recursive();
        }
        return;
    }
    for key in input.get_just_pressed() {
        let pressed = match key {
            KeyCode::Numpad0 => 0,
            KeyCode::Numpad1 => 1,
            KeyCode::Numpad2 => 2,
            KeyCode::Numpad3 => 3,
            KeyCode::Numpad4 => 4,
            KeyCode::Numpad5 => 5,
            KeyCode::Numpad6 => 6,
            KeyCode::Numpad7 => 7,
            KeyCode::Numpad8 => 8,
            KeyCode::Numpad9 => 9,
            _ => 10,
        };
        if pressed < 10 {
            if input.pressed(KeyCode::A) {
                commands.spawn_bundle(SceneBundle {
                    scene: scenes.0[pressed].clone(),
                    ..Default::default()
                });
            } else if input.pressed(KeyCode::D) {
                commands
                    .spawn_bundle(SpatialBundle::default())
                    .insert(scenes.0[pressed].clone());
            }
        }
    }
}

fn load_gltf(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GltfHandle(asset_server.load("gltf.gltf")));
}

fn gltf_scene(
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut instance: Local<Option<[InstanceId; 2]>>,
    gltf_assets: Res<Assets<bevy::gltf::Gltf>>,
    gltf_handle: Res<GltfHandle>,
) {
    if input.just_pressed(KeyCode::NumpadMultiply) {
        if instance.is_some() {
            return;
        }
        if let Some(gltf) = gltf_assets.get(&gltf_handle.0) {
            *instance = Some([
                scene_spawner.spawn(gltf.scenes[0].clone()),
                scene_spawner.spawn(asset_server.load("gltf.gltf#Scene1")),
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

//Video 2///////////////////////////////////////////////////////////////////////////
use bevy::reflect::TypeRegistry;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
#[derive(Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
struct SceneItem;

#[derive(Component)]
struct SceneCube;

fn spawn_scene_cube(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<SceneCube>>,
) {
    if !input.just_pressed(KeyCode::Key0) {
        return;
    }
    commands
        .spawn_bundle(PbrBundle {
            mesh: asset_server.load("gltf.gltf#Mesh0/Primitive0"),
            ..Default::default()
        })
        .insert(SceneItem)
        .insert(SceneCube);
    for cube in query.iter() {
        commands.entity(cube).remove::<SceneCube>();
    }
}

fn move_scene_cube(
    mut transforms: Query<&mut Transform, With<SceneCube>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut delta = Vec3::ZERO;
    let dt = time.delta_seconds();
    for key in input.get_pressed() {
        match key {
            KeyCode::A | KeyCode::Left => {delta.x -= 1.0},
            KeyCode::D | KeyCode::Right => {delta.x += 1.0},
            KeyCode::Z => {delta.y -= 1.0},
            KeyCode::Q => {delta.y += 1.0},
            KeyCode::W | KeyCode::Up => {delta.z -= 1.0},
            KeyCode::S | KeyCode::Down => {delta.z += 1.0},
            _ => {},
        }
    }
    if delta.is_finite() {
        for mut transform in transforms.iter_mut() {
            transform.translation += delta * dt * 10.;
        }
    }
}

fn advanced_save_scene(input: Res<Input<KeyCode>>, world: &World, query: Query<Entity, With<SceneItem>>,
    type_registry: Res<AppTypeRegistry>) {
    // return if the user did not just press PageUp
    if !input.just_pressed(KeyCode::PageUp) {
        return;
    }
    info!("saving advanced scene");
    // collect entites that are part of the scene into a hashset
    let query: HashSet<Entity> = query.iter().collect();
    // crate a dynamic scene for the world and type registry only including entitys from the entity set
    let dynamic_scene = from_world(world, &type_registry, query);
    // save the scene to disk
    let string = dynamic_scene.serialize_ron(&type_registry).unwrap();
    let _ = std::fs::write("./assets/advanced_scene.scn.ron", string).unwrap();
}

fn simple_save_scene(
    input: Res<Input<KeyCode>>,
    world: &World,
    type_registry: Res<AppTypeRegistry>,
) {
    // return if the user did not just press PageUp
    if !input.just_pressed(KeyCode::End) {
        return;
    }
    info!("saving simple scene");
    // crate scene from world
    let dynamic_scene = DynamicScene::from_world(world, &type_registry);
    // save the scene to disk
    let string = dynamic_scene.serialize_ron(&type_registry).unwrap();
    let _ = std::fs::write("./assets/simple_scene.scn.ron", string).unwrap();
}

/// Create a new dynamic scene from a given world and etity set;
fn from_world(
    world: &World,
    type_registry: &TypeRegistry,
    entitys: HashSet<Entity>,
) -> DynamicScene {
    let mut scene = DynamicSceneBuilder::from_world(world);
    scene.extract_entities(entitys.into_iter());
    scene.build()
}

fn test(
    mut assets: ResMut<Assets<Mesh>>,
) {
    let id = bevy::asset::HandleId::new(<Mesh as bevy::reflect::TypeUuid>::TYPE_UUID, 42069u64);
    let handle = assets.set(id, shape::Box::new(1., 1., 1.).into());
}

// fn spawn_scene(
//     asset_server: Res<AssetServer>,
//     mut commands: Commands,
// ) {
//     commands.spawn_bundle(DynamicSceneBundle{
//         scene: asset_server.load("scene.scn.ron"),
//         ..Default::default()
//     });
// }