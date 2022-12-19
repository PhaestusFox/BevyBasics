use bevy::prelude::*;
use bevy_basics::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CommonPlugin)
        .add_plugin(TransformExample)
        .add_startup_system(spawn_cam)
        .run()
}
