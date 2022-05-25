use bevy::prelude::*;
use bevy_editor_pls::*;

mod common;

mod ecs;
mod systems;
mod components;
mod entitys;
mod resources;
#[allow(dead_code)]
mod query;
#[allow(dead_code)]
mod local;
#[allow(dead_code)]
mod events;
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(EditorPlugin)
    .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
    .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
    .add_startup_system(startup_system)
    .add_system(cube_move)
    .add_plugin(common::CommonPlugin)
    .add_plugin(ecs::ECSExample)
    .add_plugin(systems::SystemPlugin)
    .add_plugin(components::ComponentPlugin)
    .add_plugin(entitys::EntityPlugin)
    .add_plugin(resources::ResourcePlugin)
    .add_system(keycode_test)
    .run();


    let mut input: Axis<GamepadAxis> = Axis::default();

    let val = input.get(GamepadAxis(Gamepad(0), GamepadAxisType::LeftStickX));

    
    input.set(GamepadAxis(Gamepad(0), GamepadAxisType::LeftStickX), 0.5);
    
    input.remove(GamepadAxis(Gamepad(0), GamepadAxisType::LeftStickX));

    if val.is_some() {
        println!("{:?}", val);
    }
}

use bevy::input::keyboard::KeyboardInput;
fn keycode_test(input: Res<Input<KeyCode>>, mut events: EventReader<KeyboardInput>) {
    for key in input.get_pressed() {
        println!("{:?}", key);
    }
    for event in events.iter() {
        println!("{:?}", event);
    }
}


#[derive(Component)]
struct Cube;

fn startup_system(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut materials_asstes: ResMut<Assets<StandardMaterial>>,
){
    let mut cam = PerspectiveCameraBundle::new_3d();
    cam.transform.translation = Vec3::ONE * 5.0;
    cam.transform.look_at(Vec3::ZERO, Vec3::Y);
    commands.spawn_bundle(cam)
    .insert(common::MainCamera);

    commands.spawn_bundle(PbrBundle{
        mesh: mesh_assets.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials_asstes.add(StandardMaterial {
            base_color: Color::rgb(1.0, 0.5, 0.5),
            ..Default::default()
        }),
        ..Default::default()
    }).insert(Cube);
}

fn cube_move(
    input: Res<Input<KeyCode>>,
    mut cubes: Query<&mut Transform, With<Cube>>
){
    let mut change = Vec3::ZERO;
    if input.just_pressed(KeyCode::W) {
        change.z -= 1.0;
    }
    if input.just_pressed(KeyCode::S) {
        change.z += 1.0;
    }
    if input.just_pressed(KeyCode::A) {
        change.x -= 1.0;
    }
    if input.just_pressed(KeyCode::D) {
        change.x += 1.0;
    }
    if input.just_pressed(KeyCode::PageDown) {
        change.y -= 1.0;
    }
    if input.just_pressed(KeyCode::PageUp) {
        change.y += 1.0;
    }
    for mut cube in cubes.iter_mut() {
        cube.translation += change;
    }
}