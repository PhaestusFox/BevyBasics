use bevy::prelude::*;

mod keyboard;
mod mouse;
mod gamepad;
mod touch;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputExample {
    Keyboard,
    MouseClick,
    MouseMove,
    MouseScroll,
    Cursor,
    GamepadButton,
    GamepadEvent,
    GamepadAxis,
    TouchEvent,
    TouchInput,
}

impl Plugin for InputExample {
    fn build(&self, app: &mut App) {
        app
        .add_state(*self)
        .add_system(change_mode)
        .add_plugin(keyboard::KeyboardExample)
        .add_plugin(mouse::MouseExample)
        .add_plugin(gamepad::GamepadExample)
        .add_plugin(touch::TouchExample);
    }
}

fn change_mode(
    input: Res<Input<KeyCode>>,
    mut state: ResMut<State<InputExample>>,
) {
    use InputExample::*;
    let up = if input.just_pressed(KeyCode::PageUp) {true} else if input.just_pressed(KeyCode::PageDown) {false} else {return;};
    let set = match state.current() {
        Keyboard => {if up {TouchEvent} else {MouseClick}},
        MouseClick => {if up {Keyboard} else {MouseMove}},
        MouseMove => {if up {MouseClick} else {MouseScroll}},
        MouseScroll => {if up {MouseMove} else {Cursor}},
        Cursor => {if up {MouseScroll} else {GamepadButton}},
        GamepadButton => {if up {Cursor} else {GamepadEvent}},
        GamepadEvent => {if up {GamepadButton} else {GamepadAxis}},
        GamepadAxis => {if up {GamepadEvent} else {TouchEvent}},
        TouchEvent => {if up {GamepadAxis} else {TouchInput}},
        TouchInput => {if up {TouchEvent} else {Keyboard}},
    };
    println!("Set mode to {:?}", set);
    state.set(set).unwrap();
}

