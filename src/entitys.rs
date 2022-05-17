use bevy::prelude::*;

use crate::common::*;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ball_resource_init)
        .add_startup_system(spawn_ball_ui)
        .add_system(spawn_ball)
        .add_system(homing_ball_move)
        .add_system(inside_world_check)
        .add_system(change_speed)
        .add_system(change_color)
        .add_system(change_shape)
        .add_system(ball_ui_update)
        .add_system(move_cam);
    }
}
#[derive(Component)]
struct HomingBall;

pub struct CurrentBall(Entity);

pub struct CurrentBallUI{
    color: Entity,
    speed: Entity,
}

fn spawn_ball(
    mut commands: Commands,
    input: Res<Input<MouseButton>>,
    camera: Query<&Transform, With<MainCamera>>,
    mesh: Res<BallMesh>,
    matt: Res<ColorWheel>,
) {
    if !input.just_pressed(MouseButton::Left) { return; }
    let start = camera.single().clone();
    let direction = start.looking_at(Vec3::ZERO, Vec3::Y).forward();
    let ball = commands.spawn_bundle(PbrBundle{
        mesh: mesh.0[0].clone(),
        material: matt.get_color(ColorSet::Random, 0),
        transform: start,
        ..Default::default()
    })
    .insert(BBDirection(direction))
    .insert(Speed(1.))
    .insert(HomingBall)
    .id();
    commands.insert_resource(CurrentBall(ball));
}

fn homing_ball_move(
    mut homing_balls: Query<(&mut Transform, &Speed, &BBDirection), With<HomingBall>>,
    time: Res<Time>,
){
    for (mut transform, speed, direction) in &mut homing_balls.iter_mut() {
        transform.translation += direction.0 * speed.0 * time.delta_seconds();
    }
}

fn inside_world_check(
    mut commands: Commands,
    homing_balls: Query<(Entity, &Transform), With<HomingBall>>,
    world_size: Res<WorldSize>,
){
    for (ball, transform) in homing_balls.iter() {
        if transform.translation.x < world_size.0.x && transform.translation.x > -world_size.0.x &&
            transform.translation.y < world_size.0.y && transform.translation.y > -world_size.0.y &&
            transform.translation.z < world_size.0.z && transform.translation.z > -world_size.0.z {
            commands.entity(ball)
            .remove::<HomingBall>()
            .insert(BoxBall);
        }
    }
}

fn change_speed(
    mut speed: Query<&mut Speed>,
    current_ball: Option<Res<CurrentBall>>,
    input: Res<Input<KeyCode>>,
){
    if current_ball.is_none() { return; }
    let ball = current_ball.unwrap().0;
    let mut speed = speed.get_mut(ball).expect("Current ball has Speed");
    if input.just_pressed(KeyCode::NumpadAdd) {
        speed.0 += 1.0;
    }
    if input.just_pressed(KeyCode::NumpadSubtract) {
        speed.0 -= 1.0;
    }
    speed.0 = speed.0.clamp(-1.0, 100.0);
}

fn change_color(
    mut color: Query<&mut Handle<StandardMaterial>>,
    current_ball: Option<Res<CurrentBall>>,
    input: Res<Input<KeyCode>>,
    colors: Res<ColorWheel>,
){
    if current_ball.is_none() { return; }
    let ball = current_ball.unwrap().0;
    let mut set_color = None;
    if input.just_pressed(KeyCode::R) {
        set_color = Some(get_color_index(&colors, ColorSet::Red, input.get_pressed()));
    }
    else if input.just_pressed(KeyCode::G) {
        set_color = Some(get_color_index(&colors, ColorSet::Blue, input.get_pressed()));
    }
    else if input.just_pressed(KeyCode::B) {
        set_color = Some(get_color_index(&colors, ColorSet::Green, input.get_pressed()));
    }
    else if input.just_pressed(KeyCode::C) {
        set_color = Some(get_color_index(&colors, ColorSet::Custom(0), input.get_pressed()));
    }
    else if input.just_pressed(KeyCode::NumpadDecimal) {
        set_color = Some(colors.get_color(ColorSet::Random, 0));
    }
    if let Some(set_color) = set_color {
        *color.get_mut(ball).expect("Current ball has Color") = set_color;
    }
}

fn change_shape(
    mut next_shape: Local<usize>,
    current_ball: Option<Res<CurrentBall>>,
    input: Res<Input<KeyCode>>,
    shapes: Res<BallMesh>,
    mut shape: Query<&mut Handle<Mesh>>,
){
    if current_ball.is_none() { return; }
    let ball = current_ball.unwrap().0;
    if input.just_pressed(KeyCode::NumpadMultiply) {
        *shape.get_mut(ball).expect("Current ball has Mesh") = shapes.0[*next_shape].clone();
        *next_shape = (*next_shape + 1) % shapes.0.len();
    }
}

fn spawn_ball_ui(
    mut commands: Commands,
    font: Res<DefaultFont>
) {
    let mut color = Entity::from_raw(0);
    let mut speed = Entity::from_raw(0);
    commands.spawn_bundle(NodeBundle{
        style: Style {
            size: Size::new(Val::Px(300.), Val::Px(100.)),
            ..Default::default()
        },
        color: Color::WHITE.into(),
        ..Default::default()
    }).with_children(|p| {
        color = p.spawn_bundle(TextBundle{
            style: Style {
                size: Size::new(Val::Px(300.), Val::Px(30.)),
                ..Default::default()
            },
            text: Text {
                sections: vec![TextSection{
                    style: TextStyle { font: font.0.clone(), font_size: 24., color: Color::BLACK },
                    value: "Color: ".to_string()
                },
                TextSection{
                    style: TextStyle { font: font.0.clone(), font_size: 24., color: Color::BLACK },
                    value: "@".to_string()
                }],
                alignment: TextAlignment { vertical: VerticalAlign::Top, horizontal: HorizontalAlign::Left } },
            ..Default::default()}).id();
        speed = p.spawn_bundle(TextBundle{
            style: Style {
                size: Size::new(Val::Px(300.), Val::Px(30.)),
                ..Default::default()
            },
            text: Text {
                sections: vec![TextSection{
                    style: TextStyle { font: font.0.clone(), font_size: 24., color: Color::BLACK },
                    value: "Speed: ".to_string()
                },
                TextSection{
                    style: TextStyle { font: font.0.clone(), font_size: 24., color: Color::BLACK },
                    value: "0".to_string()
                }],
                alignment: TextAlignment { vertical: VerticalAlign::Top, horizontal: HorizontalAlign::Left } },
            ..Default::default()}).id();
    });
    commands.insert_resource(CurrentBallUI{speed, color});
}

fn ball_ui_update(
    ui: Res<CurrentBallUI>,
    mut text: Query<&mut Text>,
    current_ball: Option<Res<CurrentBall>>,
    colors: Query<&Handle<StandardMaterial>>,
    matts: Res<Assets<StandardMaterial>>,
    speeds: Query<&Speed>,
) {
    if current_ball.is_none() {return;}
    let ball = current_ball.unwrap().0;
    let mut color = text.get_mut(ui.color).unwrap();
    let c_handel = colors.get(ball).unwrap();
    color.sections[1].style.color = matts.get(c_handel).unwrap().base_color;
    let mut speed = text.get_mut(ui.speed).unwrap();
    speed.sections[1].value = format!("{}", speeds.get(ball).unwrap().0);
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////
struct BallMesh(Vec<Handle<Mesh>>);

fn ball_resource_init(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
) {
    let mut prism = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleStrip);
    prism.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 1.0, 0.0]; 4]);
    prism.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![
        [0.0, 0.5, 0.0],
        [0.0, -0.5, 0.5],
        [-0.5, -0.5, -0.5],
        [0.5, -0.5, -0.5],
    ]);
    prism.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; 4]);
    prism.set_indices(Some(bevy::render::mesh::Indices::U32(vec![0,1,2,3,0,1])));
    commands.insert_resource(BallMesh(vec![
        mesh.add(shape::Icosphere{radius: 1., subdivisions: 2}.into()),
        mesh.add(shape::Cube{size: 1.}.into()),
        mesh.add(prism),
    ]));
}

fn get_color_index<'a>(wheel: &ColorWheel, set: ColorSet, input: impl ExactSizeIterator<Item=&'a KeyCode>) -> Handle<StandardMaterial> {
    let mut index = 0;
    for key in input {
        match key {
            KeyCode::Numpad0 => {index = 0;},
            KeyCode::Numpad1 => {index = 1;},
            KeyCode::Numpad2 => {index = 2;},
            KeyCode::Numpad3 => {index = 3;},
            KeyCode::Numpad4 => {index = 4;},
            KeyCode::Numpad5 => {index = 5;},
            KeyCode::Numpad6 => {index = 6;},
            KeyCode::Numpad7 => {index = 7;},
            KeyCode::Numpad8 => {index = 8;},
            KeyCode::Numpad9 => {index = 9;},
            _ => {}
        }
    }
    wheel.get_color(set, index)
}

fn move_cam(
    mut place: Local<i8>,
    input: Res<Input<KeyCode>>,
    mut cam: Query<&mut Transform, With<MainCamera>>,
){
    if input.just_pressed(KeyCode::Q) {
        *place += 1;

    } else if input.just_pressed(KeyCode::Z) {
        *place -= 1;
    } else {return;}
    if *place < 0 {
        *place += 18;
    }
    *place %= 18;
    cam.single_mut().translation = cam_offset(*place);
    cam.single_mut().look_at(Vec3::ZERO, Vec3::Y);
}

fn cam_offset(place: i8) -> Vec3 {
    match place {
        0 => Vec3::X * 51. + Vec3::Y,
        1 => Vec3::X * 57. + Vec3::Z *-25.,
        2 => Vec3::Z * 62.4 + Vec3::Y,
        3 => Vec3::X * -55. + Vec3::Y,
        4 => Vec3::Z * -52. + Vec3::X * 25.,
        5 => Vec3::Z * -50. + Vec3::Y,
        6 => Vec3::X * 50.25 + Vec3::Z * 40.,
        7 => Vec3::X * 50.1 + Vec3::Z * -20.,
        8 => Vec3::X * 53. + Vec3::Z * 55.,
        9 => Vec3::X * 52. + Vec3::Z * -51.,
        10 => Vec3::X * 25. + Vec3::Z * 52.,
        11 => Vec3::X * -25. + Vec3::Z * -50.3,
        12 => Vec3::X * -50.5 + Vec3::Z * 30.2,
        13 => Vec3::X * -52. + Vec3::Z * -20.1,
        14 => Vec3::X * -51. + Vec3::Z * 50.3,
        15 => Vec3::X * -56. + Vec3::Z * -50.9,
        16 => Vec3::X * -54. + Vec3::Z * 35.1,
        17 => Vec3::X * -58. + Vec3::Z * -42.,
        _ => Vec3::ZERO,
    }
}