use crate::common::Events;
use bevy::prelude::*;
fn entity_move(
    mut cubes: Query<&mut Transform, With<TransformExample>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut rot: Local<Vec3>,
    mut events: EventReader<Events>,
) {
    let mut reset = false;
    for event in events.iter() {
        match event {
            Events::Reset => {
                reset = true;
            }
        }
    }
    let mut delta = Vec3::ZERO;
    let mut scail = Vec3::ZERO;
    for key in input.get_pressed() {
        match key {
            KeyCode::A => {
                if input.pressed(KeyCode::R) {
                    rot.x -= 1.0 * time.delta_seconds();
                } else if input.pressed(KeyCode::T) {
                    scail.x -= 1.;
                } else {
                    delta.x -= 1.;
                }
            }
            KeyCode::D => {
                if input.pressed(KeyCode::R) {
                    rot.x += 1.0 * time.delta_seconds();
                } else if input.pressed(KeyCode::T) {
                    scail.x += 1.;
                } else {
                    delta.x += 1.;
                }
            }
            KeyCode::W => {
                if input.pressed(KeyCode::R) {
                    rot.y += 1.0 * time.delta_seconds();
                } else if input.pressed(KeyCode::T) {
                    scail.y += 1.;
                } else {
                    delta.y += 1.;
                }
            }
            KeyCode::S => {
                if input.pressed(KeyCode::R) {
                    rot.y -= 1.0 * time.delta_seconds();
                } else if input.pressed(KeyCode::T) {
                    scail.y -= 1.;
                } else {
                    delta.y -= 1.;
                }
            }
            KeyCode::Q => {
                if input.pressed(KeyCode::R) {
                    rot.z -= 1.0 * time.delta_seconds();
                } else if input.pressed(KeyCode::T) {
                    scail.z -= 1.;
                } else {
                    delta.z -= 1.;
                }
            }
            KeyCode::E => {
                if input.pressed(KeyCode::R) {
                    rot.z += 1.0 * time.delta_seconds();
                } else if input.pressed(KeyCode::T) {
                    scail.z += 1.;
                } else {
                    delta.z += 1.;
                }
            }
            _ => {}
        }
    }
    for mut cube in cubes.iter_mut() {
        if reset {
            *cube = Transform::from_translation(Vec3::Y);
            *rot = Vec3::ZERO;
            continue;
        }
        cube.translation += delta * time.delta_seconds();
        cube.scale += scail * time.delta_seconds();
        cube.rotation = Quat::from_euler(EulerRot::XYZ, rot.x, rot.y, rot.z);
    }
}

fn spawn_cube(
    mut commands: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshs.add(shape::Cube { size: 1. }.into()),
            material: materials.add(Color::ORANGE_RED.into()),
            transform: Transform::from_xyz(0., 1., 0.),
            ..Default::default()
        })
        .insert(TransformExample)
        .with_children(|p| {
            p.spawn(PbrBundle {
                mesh: meshs.add(shape::Cube { size: 1. }.into()),
                material: materials.add(Color::ALICE_BLUE.into()),
                transform: Transform::from_xyz(0., 1., 0.),
                ..Default::default()
            })
            .insert(TransformExample);
        });
}

//setup /////////////////////////////////////////////////
#[derive(Component)]
pub struct TransformExample;

impl Plugin for TransformExample {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_cube).add_system(entity_move);
    }
}

#[allow(dead_code, unused)]
fn crate_transforms() {
    let matrix = Transform::IDENTITY.compute_matrix();

    let transform = Transform::from_matrix(matrix);
    let transform = Transform::from_scale(Vec3::ONE);
    let transform = Transform::from_rotation(Quat::IDENTITY);
    let mut transform = Transform::from_translation(Vec3::ZERO);

    transform.with_rotation(Quat::IDENTITY);
    transform.with_scale(Vec3::ONE);
    transform.with_translation(Vec3::ZERO);

    fn get_transforms(transforms: Query<&Transform>) {
        //do stuff
    }
    let global_transform = GlobalTransform::IDENTITY;

    transform.rotate(Quat::IDENTITY);
    let x = transform.local_x();
    let up = transform.up();
    transform.look_at(Vec3::ZERO, Vec3::Y);
    let matrix = transform.compute_matrix();
    let affine = transform.compute_affine();

    let t = global_transform.translation();
    let up = global_transform.up();
    let left = global_transform.left();
    let (s, r, t) = global_transform.to_scale_rotation_translation();
}
