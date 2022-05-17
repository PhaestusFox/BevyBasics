use bevy::{prelude::*, ecs::event::Events};

fn event_update_every_x_frames<T>(frames: usize) -> impl FnMut(Events<T>, Local<usize>) {
    move |mut events, mut frame| {
        frame += 1;
        if frames >= frame {
            events.update();
            frame = 0;
        }
    }
}

fn update_every_other_frame<T>(mut events: Events<T>, mut update: Local<bool>) {
    *update = !*update;
    if *update {
        events.update();
    }
}

fn update_every_sec<T>(mut events: Events<T>, time: Res<Time>, mut last: Local<f32>) {
    last += time.delta_seconds;
    if *last > 1.0 {
        events.update();
        last %= 1.0;
    }
}

struct Popup {tital: String, icon: String, message: String}

fn some_system(mut popup: EventWriter<Popup>, input: Res<Input<KeyCode>>){
    if input.just_pressed(KeyCode::Space) {
        popup.send(Popup { tital: "KeyPress".to_string(), icon: "keys/space.png", message: "Space was pressed" })
    }
    if input.just_pressed(KeyCode::PageUp) {
        popup.send(Popup { tital: "KeyPress".to_string(), icon: "keys/PUp.png", message: "Page Up was pressed" })
    }
    if input.just_pressed(KeyCode::PageDown) {
        popup.send(Popup { tital: "KeyPress".to_string(), icon: "keys/PDown.png", message: "Page Down was pressed" })
    }
}

fn pupup_system(
    mut commands: Commands,
    mut events: EventReader<Popup>,
){
    for popup in events.iter() {
        spawn_popup(&mut Commands, popup);
    }
}