use super::InputExample;
use bevy::prelude::*;

pub struct TouchExample;
impl Plugin for TouchExample {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(InputExample::TouchEvent).with_system(touch_event))
            .add_system_set(
                SystemSet::on_update(InputExample::TouchInput).with_system(touch_input),
            );
    }
}

fn touch_event(mut touchs: EventReader<TouchInput>) {
    for touch in touchs.iter() {
        if touch.phase == bevy::input::touch::TouchPhase::Started {
            println!("touch: {:?}", touch);
        }
    }
}

fn touch_input(touchs: Res<Touches>) {
    for touch in touchs.iter() {
        println!("touch: {:?}", touch);
    }
}
