use super::InputExample;
use bevy::prelude::*;

pub struct MouseExample;
impl Plugin for MouseExample {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(InputExample::MouseClick)
                .with_system(mouse_button)
                .with_system(mouse_button_raw),
        );
        app.add_system_set(
            SystemSet::on_update(InputExample::MouseMove).with_system(mouse_move_raw),
        );
        app.add_system_set(
            SystemSet::on_update(InputExample::MouseScroll).with_system(mouse_scroll),
        );
        app.add_system_set(
            SystemSet::on_update(InputExample::Cursor)
                .with_system(mouse_move)
                .with_system(cursor_click),
        );
    }
}

fn mouse_button(input: Res<Input<MouseButton>>) {
    for key in input.get_just_pressed() {
        println!("you have clicked: {:?}", key);
    }
}

use bevy::input::mouse::MouseButtonInput;
fn mouse_button_raw(mut input: EventReader<MouseButtonInput>) {
    for key in input.iter() {
        println!("you have clicked: {:?}", key);
    }
}

use bevy::input::mouse::MouseMotion;
fn mouse_move_raw(mut events: EventReader<MouseMotion>) {
    for e in events.iter() {
        println!("Mouse Move: {:?}", e)
    }
}

use bevy::input::mouse::MouseWheel;
fn mouse_scroll(mut events: EventReader<MouseWheel>) {
    for e in events.iter() {
        println!("Scroll is: {:?}", e);
    }
}

fn mouse_move(mut events: EventReader<CursorMoved>) {
    for e in events.iter() {
        println!("Cursor is at: {:?}", e);
    }
}

fn cursor_click(mut events: EventReader<CursorEntered>) {
    for e in events.iter() {
        println!("Enter window: {:?}", e);
    }
}
