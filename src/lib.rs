use bevy::prelude::*;

mod common;

pub mod prelude {
    pub use super::{
        common::*,
        transform::TransformExample,
        visibility::VisibilityExample,
        spawn_cam,
        user_input::InputExample,
        entitys::EntityPlugin,
        systems::SystemPlugin,
        ecs::ECSExample,
        components::ComponentPlugin,
        resources::ResourcePlugin,
    };
}

mod ecs;
mod systems;
mod components;
mod entitys;
mod resources;
#[allow(dead_code)]
mod query;
#[allow(dead_code)]
mod local;
#[allow(dead_code)]
mod events;
#[allow(dead_code)]
mod user_input;
mod transform;
mod visibility;
pub fn spawn_cam(
    mut commands: Commands,
) {
    let trans = Transform::from_xyz(5., 5., 5.);
    commands.spawn_bundle(Camera3dBundle {
        transform: trans.looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    })
    .insert(common::MainCamera);
    commands.spawn_bundle(SpotLightBundle{
        transform: trans.looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}