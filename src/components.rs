use bevy::prelude::*;
use super::common::*;
use rand::Rng;

pub struct ComponentPlugin;

impl Plugin for ComponentPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldSize(Vec3::ONE * 10.));
        app.add_startup_system(spawn_balls)
        .add_system(move_balls)
        .add_system(speed_switch);
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct ComponentExample;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Speed(pub f32);

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Direction(pub Vec3);

fn spawn_balls(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    world_size: Res<WorldSize>,
){
    let mut rng = rand::thread_rng();
    let mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: 1.,
        subdivisions: 2
    }));
    for _ in 0..10 {
        let speed = rng.gen_range(5.0..=50.0);
        let direction = Vec3::new(rng.gen_range(-1.0..1.), rng.gen_range(-1.0..1.), rng.gen_range(-1.0..1.));
        let size = rng.gen_range(0.5..=1.5);
        let material = materials.add(StandardMaterial {
            base_color: Color::rgb(speed/50., size - 0.5, rng.gen_range(0.0..=1.0)),
            ..Default::default()
        });
        commands.spawn_bundle(PbrBundle{
            mesh: mesh.clone(),
            material,
            transform: Transform::from_translation(
                Vec3::new(rng.gen_range(-world_size.0.x..world_size.0.x), rng.gen_range(-world_size.0.y..world_size.0.y), rng.gen_range(-world_size.0.z..world_size.0.z)))
                .with_scale(Vec3::ONE * size),
            ..Default::default()
        })
        .insert(Speed(speed))
        .insert(Direction(direction))
        .insert(ComponentExample);
    }
}

fn move_balls(
    mut balls: Query<(&mut Transform, &Speed, &mut Direction), With<ComponentExample>>,
    time: Res<Time>,
    world_size: Res<WorldSize>,
){
    let dt = time.delta_seconds();
    for (mut transform, speed, mut direction) in balls.iter_mut() {
        transform.translation += direction.0 * speed.0 * dt;
        if transform.translation.x.abs() >= world_size.0.x {
            direction.0.x = -direction.0.x;
        }
        if transform.translation.y.abs() >= world_size.0.y {
            direction.0.y = -direction.0.y;
        }
        if transform.translation.z.abs() >= world_size.0.z {
            direction.0.z = -direction.0.z;
        }
        transform.translation =  transform.translation.clamp(-world_size.0 * 0.99, world_size.0 * 0.99);
    }
}

fn speed_switch(
    switch: Res<SpamTime>,
    mut balls: Query<&mut Speed, With<ComponentExample>>,
    mut fast: Local<bool>,
) {
    if !switch.finished() {return;}
    for mut ball in balls.iter_mut(){
        if *fast {
            ball.0 /= 2.;
        } else {
            ball.0 *= 2.;
        }
    }
    *fast = !*fast;
}

//End///////////////////////////////////////////////
pub struct WorldSize(pub Vec3);