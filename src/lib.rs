use bevy::prelude::*;

mod common;
pub use prelude::*;
pub mod prelude {
    pub use super::{
        common::*, components::ComponentPlugin, ecs::ECSExample, entitys::EntityPlugin,
        reflect::ReflectExample, resources::ResourcePlugin, scenes::ScenesExample, spawn_cam,
        systems::SystemPlugin, transform::TransformExample, user_input::InputExample,
        visibility::VisibilityExample,
    };
}

mod components;
mod ecs;
mod entitys;
#[allow(dead_code)]
mod events;
#[allow(dead_code)]
mod local;
#[allow(dead_code)]
mod query;
#[allow(dead_code)]
mod reflect;
mod resources;
mod scenes;
mod systems;
mod transform;
#[allow(dead_code)]
mod user_input;
mod visibility;
pub fn spawn_cam(mut commands: Commands) {
    let trans = Transform::from_xyz(5., 5., 5.);
    commands
        .spawn(Camera3dBundle {
            transform: trans.looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(common::MainCamera);
    commands.spawn(SpotLightBundle {
        transform: trans.looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
