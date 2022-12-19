use bevy::prelude::*;

fn test(mut id: u8) -> impl FnMut(Res<Input<KeyCode>>) {
    move |input| {
        println!("test system {:?}", id);
        if input.just_pressed(KeyCode::Space) {
            id += 1;
        }
    }
}

#[derive(Resource)]
pub struct Slave(u8);

pub fn hello(is_slave: Local<Slave>) {
    if is_slave.0 != 0 {
        println!("Hello from slave {}", is_slave.0);
    } else {
        println!("Hello from master");
    }
}

pub fn hello2(is_slave1: Local<Slave>, is_slave2: Local<Slave>) {
    if is_slave1.0 != 0 {
        println!("Hello from slave {}", is_slave1.0);
    } else {
        println!("Hello from master");
    }
    println!("Slaver {} is here too", is_slave2.0);
}

impl FromWorld for Slave {
    fn from_world(world: &mut World) -> Self {
        if let Some(mut is_slave) = world.get_resource_mut::<Slave>() {
            is_slave.0 += 1;
            Slave(is_slave.0)
        } else {
            world.insert_resource(Slave(0));
            Slave(0)
        }
    }
}

fn edit_loacl(mut slave: Local<Slave>) {
    *slave = Slave(slave.0 + 1);
}

fn first_run_example(mut not_first: Local<bool>) {
    if !(*not_first) {
        //first time the system runs
        *not_first = true;
    } else {
        //all other tiemes the system runs
    }
    //runs every time
}

#[derive(Default, Resource)]
struct LastState(u64);

fn change_example(mut last_state: Local<LastState>, this_state: Res<LastState>) {
    if this_state.0 == last_state.0 {
        return;
    }
    last_state.0 = this_state.0;
    //code that only runs of state change
}
