use bevy::{prelude::*, ecs::event::Events};

fn event_update_every_x_frames<T: 'static + Send + Sync>(frames: usize) -> impl FnMut(ResMut<Events<T>>, Local<usize>) {
    move |mut events: ResMut<Events<T>>, mut frame: Local<usize>| {
        *frame += 1;
        if frames >= *frame {
            events.update();
            *frame = 0;
        }
    }
}

fn update_every_other_frame<T:'static + Send + Sync>(mut events: ResMut<Events<T>>, mut update: Local<bool>) {
    *update = !*update;
    if *update {
        events.update();
    }
}

fn update_every_sec<T:'static + Send + Sync>(mut events: ResMut<Events<T>>, time: Res<Time>, mut last: Local<f32>) {
    *last += time.delta_seconds();
    if *last > 1.0 {
        events.update();
        *last %= 1.0;
    }
}

struct Popup {tital: String, icon: String, message: String}

fn some_system(mut popup: EventWriter<Popup>, input: Res<Input<KeyCode>>){
    if input.just_pressed(KeyCode::Space) {
        popup.send(Popup { tital: "KeyPress".to_string(), icon: "keys/space.png".to_string(), message: "Space was pressed".to_string() })
    }
    if input.just_pressed(KeyCode::PageUp) {
        popup.send(Popup { tital: "KeyPress".to_string(), icon: "keys/PUp.png".to_string(), message: "Page Up was pressed".to_string() })
    }
    if input.just_pressed(KeyCode::PageDown) {
        popup.send(Popup { tital: "KeyPress".to_string(), icon: "keys/PDown.png".to_string(), message: "Page Down was pressed".to_string() })
    }
}

fn pupup_system(
    mut commands: Commands,
    mut events: EventReader<Popup>,
){
    for popup in events.iter() {
        spawn_popup(&mut commands, popup);
    }
}

fn spawn_popup(_commands: &mut Commands, _popup: &Popup) {
    todo!()
}