use bevy::prelude::*;
//use bevy_editor_pls::*;
use bevy_basics::prelude::*;

fn main() {
    App::new()
    // default stuff //
    .add_plugins(DefaultPlugins)
    //.add_plugin(EditorPlugin)
    .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
    .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
    .add_plugin(CommonPlugin)
    .add_startup_system(spawn_cam)
    // end default stuff //
    
    // comment in the examples you want //
    .add_system(cube_move)
    .add_startup_system(startup_system)
    // .add_plugin(ECSExample)
    // .add_plugin(SystemPlugin)
    // .add_plugin(ComponentPlugin)
    // .add_plugin(EntityPlugin)
    // .add_plugin(ResourcePlugin)
    // .add_plugin(InputExample::TouchEvent)
    .add_plugin(TransformExample)
    .add_plugin(VisibilityExample(0))
    
    .run();
}

#[derive(Component)]
struct Cube;

pub fn spawn_cam(
    mut commands: Commands,
) {
    let trans = Transform::from_xyz(5., 5., 5.);
    commands.spawn_bundle(Camera3dBundle {
        transform: trans.looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    })
    .insert(MainCamera);
    commands.spawn_bundle(SpotLightBundle{
        transform: trans.looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

#[allow(dead_code)]
fn startup_system(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut materials_asstes: ResMut<Assets<StandardMaterial>>,
){
    commands.spawn_bundle(PbrBundle{
        mesh: mesh_assets.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials_asstes.add(StandardMaterial {
            base_color: Color::rgb(1.0, 0.5, 0.5),
            ..Default::default()
        }),
        ..Default::default()
    }).insert(Cube);
}

#[allow(dead_code)]
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