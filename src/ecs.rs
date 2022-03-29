use bevy::prelude::*;

pub struct ECSExample;

impl Plugin for ECSExample {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(my_system)
        .add_system(my_logic)
        .add_system(my_other_logic);
    }
}

#[derive(Component)]
struct MyCompontentUnit;

#[derive(Debug, Component)]
struct MyCompontentWithData(String);

#[derive(Debug, Component)]
struct MyCompontentWithName{
    name: String
}

struct Target(Entity);

fn my_system(
    mut commands: Commands,
) {
    println!("spawned entity {:?}", commands.spawn().insert(MyCompontentUnit).id());
    let entity = commands.spawn().insert(MyCompontentWithName{name: "target".to_string()}).id();
    println!("spawned entity {:?}", commands.spawn().insert(MyCompontentWithData("Hello".to_string())).id());
    println!("spawned entity {:?}", commands.spawn().insert(MyCompontentWithData("World".to_string())).id());
    commands.insert_resource(Target(entity));
}

fn my_logic(
    input: Res<Input<KeyCode>>,
    data: Query<&MyCompontentWithData>,
    targets: Query<&MyCompontentWithName>,
    target: Res<Target>,
){
    if input.just_pressed(KeyCode::P) {
        for data in data.iter() {
            println!("Data:{}",data.0);
        }
    }
    if input.just_pressed(KeyCode::T) {
        println!("the target is {:?}:{}",target.0 , targets.get(target.0).unwrap().name);
    }
}

fn my_other_logic(
    input: Res<Input<KeyCode>>,
    unit: Query<&MyCompontentUnit>,
){
    if input.just_pressed(KeyCode::P) {
        for _ in unit.iter() {
            println!("Unit");
        }
    }
}