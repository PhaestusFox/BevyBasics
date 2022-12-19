use bevy::prelude::*;
use bevy_basics::prelude::*;
use bevy_inspector_egui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(VisibilityExample(0))
        .add_startup_system(spawn_cam)
        .add_plugin(WorldInspectorPlugin::default())
        .run()
}

fn toggle_visibility(
    mut visibility: Query<(&mut Visibility, &VisibilityExample)>,
    input: Res<Input<KeyCode>>,
) {
    let mut set: std::collections::HashSet<u8> = std::collections::HashSet::default();
    for key in input.get_just_pressed() {
        match key {
            KeyCode::Numpad0 => {
                set.insert(0);
            }
            KeyCode::Numpad1 => {
                set.insert(1);
            }
            KeyCode::Numpad2 => {
                set.insert(2);
            }
            KeyCode::Numpad3 => {
                set.insert(3);
            }
            KeyCode::Numpad4 => {
                set.insert(4);
            }
            KeyCode::Numpad5 => {
                set.insert(5);
            }
            KeyCode::Numpad6 => {
                set.insert(6);
            }
            KeyCode::Numpad7 => {
                set.insert(7);
            }
            KeyCode::Numpad8 => {
                set.insert(8);
            }
            KeyCode::Numpad9 => {
                set.insert(9);
            }
            _ => {}
        }
    }
    for (mut vis, id) in visibility.iter_mut() {
        if set.contains(&id.0) {
            vis.is_visible = !vis.is_visible;
        }
    }
}

//setup///////////////////////////////
#[derive(Component, Reflect)]
pub struct VisibilityExample(pub u8);

impl Plugin for VisibilityExample {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_cube)
            .add_system(toggle_visibility);
    }
}

const CUBE_COLORS: [Color; 10] = [
    Color::RED,
    Color::GREEN,
    Color::BLUE,
    Color::BLACK,
    Color::CRIMSON,
    Color::CYAN,
    Color::GOLD,
    Color::VIOLET,
    Color::WHITE,
    Color::ORANGE,
];

fn spawn_cube(
    mut commands: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut last = Entity::from_raw(0);
    for i in 0..10 {
        let e = commands
            .spawn_bundle(PbrBundle {
                transform: Transform::from_xyz(0., if i == 0 { -2.5 } else { 0.5 }, 0.),
                mesh: meshs.add(shape::Cube { size: 0.5 }.into()),
                material: materials.add(CUBE_COLORS[i].into()),
                ..Default::default()
            })
            .insert(VisibilityExample(i as u8))
            .id();
        if i == 0 {
            last = e;
            continue;
        }
        commands.entity(last).add_child(e);
        last = e;
    }
}
