use super::InputExample;
use bevy::prelude::*;

pub struct KeyboardExample;
impl Plugin for KeyboardExample {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(InputExample::Keyboard).with_system(keycode_test));
    }
}

fn key_press(input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        //Jump
    }
}

fn input(input: Res<Input<KeyCode>>) {
    for key in input.get_pressed() {
        match key {
            KeyCode::A => {
                //left
            }
            KeyCode::W => {
                //forward
            }
            KeyCode::S => {
                //back
            }
            KeyCode::D => {
                //right
            }
            _ => {}
        }
    }
}

fn multi_key(input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Space)
        && input.any_pressed([KeyCode::A, KeyCode::D, KeyCode::W, KeyCode::S])
    {
        //moving jump
    }
}

use bevy::input::keyboard::KeyboardInput;
fn order_keys(mut keys: EventReader<KeyboardInput>) {
    for key in keys.iter() {
        if let Some(keycode) = key.key_code {
            //has a standard keycode
            let _ = keycode;
        } else {
            //no keycode use scan code here
        }
    }
}

fn keycode_test(input: Res<Input<KeyCode>>, mut events: EventReader<KeyboardInput>) {
    for key in input.get_pressed() {
        println!("{:?}", key);
    }
    for event in events.iter() {
        println!("{:?}", event);
    }
}

#[derive(Debug, Component)]
struct Player;
fn player_controler(mut player: Query<&mut Transform, With<Player>>, input: Res<Input<KeyCode>>) {
    let mut player = player.single_mut();
    if input.any_pressed([KeyCode::W, KeyCode::Up, KeyCode::Numpad8]) {
        player.translation.y += 1.
    }
    if input.any_pressed([KeyCode::S, KeyCode::Down, KeyCode::Numpad2]) {
        player.translation.y -= 1.
    }
    if input.any_pressed([KeyCode::A, KeyCode::Left, KeyCode::Numpad4]) {
        player.translation.x -= 1.
    }
    if input.any_pressed([KeyCode::D, KeyCode::Right, KeyCode::Numpad6]) {
        player.translation.x += 1.
    }
}
