use bevy::prelude::*;

pub use crate::components::{Speed, BBDirection, ComponentExample as BoxBall, WorldSize};
pub use crate::resources::{DefaultFont, ColorSet, ColorWheel};

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct UiCamera;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpamTime(Timer::from_seconds(2., true)));
        app.add_system_to_stage(CoreStage::First, timer_updata);
        app.register_type::<super::components::Speed>();
        app.register_type::<super::components::BBDirection>();
        app.add_event::<Events>();
        app.add_system(reset_system);
    }
}

fn reset_system(
   mut events: EventWriter<Events>,
   input: Res<Input<KeyCode>>,
){
    if input.just_pressed(KeyCode::Space) {
        println!("send reset");
        events.send(Events::Reset);
    }
}

pub struct SpamTime(Timer);

impl SpamTime {
    pub fn finished(&self) -> bool {
        self.0.finished()
    }
}

fn timer_updata(
    time: Res<Time>,
    mut timer: ResMut<SpamTime>,
){
    timer.0.tick(time.delta());
}

pub enum Events {
    Reset,
}