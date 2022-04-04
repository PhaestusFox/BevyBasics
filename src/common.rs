use bevy::prelude::*;

pub use crate::components::{Speed, Direction as BBDirection, ComponentExample as BoxBall, WorldSize};

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
        app.register_type::<super::components::Direction>();
        app.add_startup_system_to_stage(StartupStage::PreStartup, ui_init);
    }
}

pub struct DefaultFont(pub Handle<Font>);

fn ui_init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    commands.spawn_bundle(UiCameraBundle::default());
    commands.insert_resource(DefaultFont(asset_server.load("./Raleway-Bold.ttf")));
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