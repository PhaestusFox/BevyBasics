use bevy::prelude::*;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpamTime(Timer::from_seconds(2., true)));
        app.add_system_to_stage(CoreStage::First, timer_updata);
        app.register_type::<super::components::Speed>();
        app.register_type::<super::components::Direction>();
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