use bevy::prelude::*;
use bevy_basics::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_inspector_egui::prelude::WorldInspectorPlugin::default())
        .add_startup_system(spawn_cam)
        .add_startup_system(spawn_world)
        .add_system(change_state)
        .add_system(add_move)
        .add_system(update_ui)
        .add_system(set_ui_arrow.after(update_ui))
        .add_system(set_ui_letter.after(update_ui))
        .add_system(update_select)
        .add_system(run)
        .run()
}

#[derive(Debug, Component)]
struct Cube;

const MOVES: usize = 3;

struct Directions {
    z: UiImage,
    y: UiImage,
    x: UiImage,
}

const COLORS: [Color; 6] = [
    Color::RED,
    Color::BLUE,
    Color::WHITE,
    Color::GREEN,
    Color::ORANGE,
    Color::YELLOW,
];

fn spawn_world(
    mut commands: Commands,
    mut colors: ResMut<Assets<StandardMaterial>>,
    mut mesh: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let mesh = mesh.add(
        shape::Quad {
            size: Vec2::ONE * 2.0,
            flip: false,
        }
        .into(),
    );
    let top = commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::Y).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                -90f32.to_radians(),
                0.,
                0.,
            )),
            material: colors.add(COLORS[5].into()),
            mesh: mesh.clone(),
            ..Default::default()
        })
        .id();
    let bottom = commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(-Vec3::Y).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                90f32.to_radians(),
                0.,
                0.,
            )),
            material: colors.add(COLORS[2].into()),
            mesh: mesh.clone(),
            ..Default::default()
        })
        .id();
    let left = commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(-Vec3::X).with_rotation(Quat::from_euler(
                EulerRot::YXZ,
                -90f32.to_radians(),
                0.,
                0.,
            )),
            material: colors.add(COLORS[0].into()),
            mesh: mesh.clone(),
            ..Default::default()
        })
        .id();
    let right = commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::X).with_rotation(Quat::from_euler(
                EulerRot::YXZ,
                90f32.to_radians(),
                0.,
                0.,
            )),
            material: colors.add(COLORS[4].into()),
            mesh: mesh.clone(),
            ..Default::default()
        })
        .id();
    let back = commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(-Vec3::Z).with_rotation(Quat::from_euler(
                EulerRot::YXZ,
                -180f32.to_radians(),
                0.,
                0.,
            )),
            material: colors.add(COLORS[1].into()),
            mesh: mesh.clone(),
            ..Default::default()
        })
        .id();
    let frunt = commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::Z).with_rotation(Quat::from_euler(
                EulerRot::ZXY,
                90f32.to_radians(),
                0.,
                0.,
            )),
            material: colors.add(COLORS[3].into()),
            mesh: mesh.clone(),
            ..Default::default()
        })
        .id();
    commands
        .spawn_bundle(SpatialBundle::default())
        .insert(Cube)
        .push_children(&[top, bottom, left, right, frunt, back]);
    let arrow_image: UiImage = asset_server.load("arrow.png").into();
    let directions = Directions {
        x: asset_server.load("X.png").into(),
        y: asset_server.load("Y.png").into(),
        z: asset_server.load("Z.png").into(),
    };
    let ui = commands
    .spawn_bundle(NodeBundle{
        style: Style {
            size: Size {
                width: Val::Px((77 * MOVES) as f32),
                height: Val::Px(52.0),
            },
            margin: UiRect{
                left: Val::Auto,
                right: Val::Auto,
                bottom: Val::Px(10.),
                top: Val::Undefined
            },
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Name::new("UI Bar"))
    .add_children(|p| {
            let mut ui = UIElements([Entity::from_bits(u64::MAX); MOVES]);
            for i in 0..ui.0.len() {
                ui.0[i] = p.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size { width: Val::Px(75.0), height: Val::Px(50.) },
                        padding: UiRect::all(Val::Px(1.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Name::new(format!("Slot {}", i)))
                .with_children(|p| {
                    p
                        .spawn_bundle(ImageBundle {
                            image: arrow_image.clone(),
                            style: Style {
                                size: Size { width: Val::Px(25.), height: Val::Px(50.) },
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                    p
                        .spawn_bundle(ImageBundle {
                            image: arrow_image.clone(),
                            style: Style {
                                size: Size { width: Val::Px(50.), height: Val::Px(50.) },
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                }).id();
            }
            ui
        });
    commands.insert_resource(Order::default());
    commands.insert_resource(ui);
    commands.insert_resource(directions);
    commands.insert_resource(Selected(0));
}

fn change_state(
    input: Res<Input<KeyCode>>,
    mut transforms: Query<&mut Transform, With<Cube>>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::Space) {
        for mut transform in transforms.iter_mut() {
            *transform = Transform::identity();
        }
        commands.remove_resource::<Run>();
        return;
    }
    if input.just_pressed(KeyCode::Return) {
        commands.insert_resource(Run);
    }
}
#[derive(Component, Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}
#[derive(Component, Clone, Copy)]
struct Arrow(bool);

struct Order([(Axis, Arrow); MOVES]);
impl Order {
    fn as_vec3(&self, index: usize) -> Vec3 {
        match self.0[index].0 {
            Axis::X => if self.0[index].1.0 {Vec3::NEG_X} else {Vec3::X},
            Axis::Y => if self.0[index].1.0 {Vec3::NEG_Y} else {Vec3::Y},
            Axis::Z => if self.0[index].1.0 {Vec3::NEG_Z} else {Vec3::Z},
        }
    }
}
impl Default for Order {
    fn default() -> Self {
        Order([(Axis::X, Arrow(true));MOVES])
    }
}
struct Run;

fn add_move(
    mut selected: ResMut<Selected>,
    mut order: ResMut<Order>,
    input: Res<Input<KeyCode>>, 
    runing: Option<Res<Run>>
) {
    if runing.is_some() {return;}
    for key in input.get_just_pressed() {
        match key {
            KeyCode::A |
            KeyCode::Left => {
                if selected.0 == 0 {
                    selected.0 = MOVES - 1;
                } else {
                    selected.0 -= 1;
                }
            },
            KeyCode::D |
            KeyCode::Right => {
                selected.0 = (selected.0 + 1) % MOVES;
            },
            KeyCode::W |
            KeyCode::Up => {
                set_arrow(selected.0, &mut order, true);
            },
            KeyCode::S |
            KeyCode::Down => {
                set_arrow(selected.0, &mut order, false);
            },
            KeyCode::Z => {
                set_axis(selected.0, &mut order, Axis::Z);
            },
            KeyCode::X => {
                set_axis(selected.0, &mut order, Axis::X);
            },
            KeyCode::Y |
            KeyCode::C => {
                set_axis(selected.0, &mut order, Axis::Y);
            },
            _ => {}
        }
    }
}

#[inline]
fn set_axis(index: usize, order: &mut Order, to: Axis) {
    order.0[index].0 = to;
}

#[inline]
fn set_arrow(index: usize, order: &mut Order, to: bool) {
    order.0[index].1.0 = to;
}

#[derive(Clone, Copy)]
struct UIElements([Entity; MOVES]);

fn update_ui(
    elements: Res<UIElements>,
    order: Res<Order>,
    mut commands: Commands,
) {
    if !order.is_changed() {return;}
    for i in 0..MOVES {
        commands.entity(elements.0[i]).insert_bundle(order.0[i]);
    }
}

fn set_ui_arrow(
    query: Query<(&Arrow, &Children), Changed<Arrow>>,
    mut transforms : Query<&mut Transform>,
) {
    for (state, c) in query.iter() {
        if state.0 {
                if let Ok(mut trans) = transforms.get_mut(c[0]) {
                    trans.rotation = Quat::IDENTITY;
                }
            } else  {
                if let Ok(mut trans) = transforms.get_mut(c[0]) {
                    trans.rotation = Quat::from_rotation_z(std::f32::consts::PI);
                }
        }
    }
}

fn set_ui_letter(
    query: Query<(&Axis, &Children), Changed<Axis>>,
    mut ui_images: Query<&mut UiImage>,
    directions: Res<Directions>,
) {
    for (state, c) in query.iter() {
        match state {
                Axis::X => {
                if let Ok(mut image) = ui_images.get_mut(c[1]) {
                    *image = directions.x.clone();
                }
            },
            Axis::Y => {
                if let Ok(mut image) = ui_images.get_mut(c[1]) {
                    *image = directions.y.clone();
                }
            },
            Axis::Z => {
                if let Ok(mut image) = ui_images.get_mut(c[1]) {
                    *image = directions.z.clone();
                }
            },
        }
    }
}

#[derive(Debug, DerefMut, Deref)]
struct Selected(usize);

#[derive(Component)]
struct Select;

fn update_select(
    selected: Res<Selected>,
    ui: Res<UIElements>,
    mut query: Query<&mut UiColor>,
    mut old: Local<usize>,
) {
    if selected.is_changed() {
        if let Ok(mut color) = query.get_mut(ui.0[*old]) {
            color.0 = Color::WHITE;
        }
        if let Ok(mut color) = query.get_mut(ui.0[selected.0]) {
            color.0 = Color::GREEN;
        }
        *old = selected.0;
    }
}

fn run(
    mut query: Query<&mut Transform, With<Cube>>,
    order: Res<Order>,
    mut progress: Local<f32>,
    mut step: Local<usize>,
    run: Option<Res<Run>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    if run.is_none() {return;}
    let run = run.unwrap();
    if run.is_added() {
        *progress = 0.0;
        *step = 0;
    }
    if *step == MOVES {
        commands.remove_resource::<Run>();
        return;
    }
    let mut trans = query.single_mut();
    if *progress + time.delta_seconds() > 1.0 {
        let max = *progress + time.delta_seconds();
        let over = max - 1.;
        let under = time.delta_seconds() - over;
        trans.rotate_axis(order.as_vec3(*step), under * std::f32::consts::PI / 2.);
        *step += 1;
        if *step == MOVES {return;};
        trans.rotate_axis(order.as_vec3(*step), over * std::f32::consts::PI / 2.);
        *progress = over;
    } else {
        trans.rotate_axis(order.as_vec3(*step), time.delta_seconds() * std::f32::consts::PI / 2.);
        *progress += time.delta_seconds();
    }
}