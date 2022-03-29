use bevy::prelude::*;
use bevy_editor_pls::*;

mod ecs;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(EditorPlugin)
    .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
    .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
    .add_startup_system(startup_system)
    .add_system(cube_move)
    .add_plugin(ecs::ECSExample)
    .run()
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
    commands.spawn_bundle(cam);

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