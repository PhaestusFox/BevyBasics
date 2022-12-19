use super::InputExample;
use bevy::{input::gamepad::GamepadEventRaw, prelude::*};

pub struct GamepadExample;
impl Plugin for GamepadExample {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(InputExample::GamepadButton)
                .with_system(gamepad_button)
                .with_system(gamepad_button_axis),
        )
        .add_system_set(
            SystemSet::on_update(InputExample::GamepadEvent)
                .with_system(gamepad_event)
                .with_system(gamepad_event_raw),
        )
        .add_system_set(SystemSet::on_update(InputExample::GamepadAxis).with_system(gamepad_axis));
    }
}

fn gamepad_button(input: Res<Input<GamepadButton>>) {
    for button in input.get_just_pressed() {
        println!("You pressed : {:?}", button);
    }
}

fn gamepad_button_axis(input: Res<Axis<GamepadButton>>) {
    for button in [
        GamepadButtonType::LeftTrigger2,
        GamepadButtonType::RightTrigger2,
    ] {
        if let Some(val) = input.get(GamepadButton {
            gamepad: Gamepad { id: 0 },
            button_type: button,
        }) {
            if val > DEADZONE && val < 1.0 {
                println!("you have trigger at: {}", val);
            }
        }
    }
}

const DEADZONE: f32 = 0.1;
const AXISTYPES: [GamepadAxisType; 6] = [
    GamepadAxisType::LeftStickX,
    GamepadAxisType::LeftStickY,
    GamepadAxisType::LeftZ,
    GamepadAxisType::RightStickX,
    GamepadAxisType::RightStickY,
    GamepadAxisType::RightZ,
];

fn gamepad_axis(input: Res<Axis<GamepadAxis>>) {
    for axis in AXISTYPES.iter() {
        if let Some(x) = input.get(GamepadAxis {
            gamepad: Gamepad { id: 0 },
            axis_type: *axis,
        }) {
            if x.abs() > DEADZONE {
                println!("gamepad axis {:?}: {}", axis, x)
            }
        }
    }
}

fn gamepad_event(mut events: EventReader<GamepadEvent>) {
    for e in events.iter() {
        println!("gamepad event: {:?}", e);
    }
}

fn gamepad_event_raw(mut events: EventReader<GamepadEventRaw>) {
    for e in events.iter() {
        println!("Raw Event: {:?}", e);
    }
}

fn list_gamepads(gamepads: Res<Gamepads>, time: Res<Time>, mut delta: Local<f32>) {
    *delta += time.delta_seconds();
    if *delta < 1.0 {
        return;
    }
    let gamepads: Vec<Gamepad> = gamepads.iter().collect();
    if gamepads.len() == 0 {
        println!("there are no gamepads connected");
        return;
    }
    println!(
        "there are {} controlers connected\n they are:",
        gamepads.len()
    );
    for gamepad in gamepads {
        println!("{:?}", gamepad);
    }
}
