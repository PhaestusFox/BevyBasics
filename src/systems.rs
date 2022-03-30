use bevy::prelude::*;
use super::common::*;

pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(basic_system);
        app.add_startup_system(startup_system);
        app.add_system(exclusive_system.exclusive_system());
        app.add_system_to_stage(CoreStage::First, first_system);
        app.add_system_to_stage(CoreStage::Last, last_system);
        app.add_system(system_chain_one.chain(system_chain_two));
        app.add_startup_system(test_system);
    }
}

fn startup_system(){
    println!("running startup system once");
}

fn basic_system(
    input: Res<Input<KeyCode>>,
){
    if input.just_pressed(KeyCode::Space) {
        println!("Space was pressed");
    }
}

fn exclusive_system(world: &mut World) {
    let input = world.get_resource::<Input<KeyCode>>().unwrap();
    if input.just_pressed(KeyCode::Space) {
        println!("got input directily from world");
    }
}

fn first_system(timer: Res<SpamTime>) {
    if timer.finished() {
        println!("running first system");
    }
}

fn last_system(timer: Res<SpamTime>) {
    if timer.finished(){
        println!("running last system");
    }
}

fn system_chain_one(mut loacl: Local<f32>, input: Res<Input<KeyCode>>) -> f32 {
    let mut change = 0.0;
    for key in input.get_just_pressed(){
        match key {
            KeyCode::Numpad1 => change += 1.0,
            KeyCode::Numpad2 => change += 2.0,
            KeyCode::Numpad3 => change += 3.0,
            KeyCode::Numpad4 => change += 4.0,
            KeyCode::Numpad5 => change += 5.0,
            KeyCode::Numpad6 => change += 6.0,
            KeyCode::Numpad7 => change += 7.0,
            KeyCode::Numpad8 => change += 8.0,
            KeyCode::Numpad9 => change += 9.0,
            _ => {}
        }
    }
    if input.pressed(KeyCode::NumpadSubtract) {
        change = -change;
    }
    *loacl += change;
    *loacl
}

fn system_chain_two(input: In<f32>, time: Res<Time>, mut loacl: Local<f32>) {
    if input.0 == *loacl {
        return;
    }
    println!("time: {} with input {}", time.seconds_since_startup(), input.0);
    *loacl = input.0;
}

fn test_system(
    mut commands: Commands,
    res: Res<Time>,
    res_mut: ResMut<Assets<Mesh>>,
    op_res: Option<Res<AssetServer>>,
    query: Query<&mut Transform, Changed<Interaction>>,
    mut event_r: EventReader<AssetEvent<StandardMaterial>>,
    event_w: EventWriter<GamepadEvent>,
){
    commands.spawn().despawn();
    let _ = res;
    let _ = res_mut;
    let _ = op_res;
    let _ = query;
    let _ = event_w;
    for event in event_r.iter(){
        println!("got event {:?}", event);
    }
}