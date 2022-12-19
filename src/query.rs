use crate::common::*;
use bevy::prelude::*;

fn query_init(mut commands: Commands) {
    commands
        .spawn((Speed(0.),BBDirection(Vec3::ZERO),Name::new("Bob")));
}

fn single(_query: Query<&SomeComponent>) {}

fn multiple(_query: Query<(&SomeComponent, &SomeOtherComponent)>) {}

fn query(query: Query<&Name, Or<(With<Speed>, With<Transform>)>>) {
    for name in query.iter() {
        println!("{} has A Speed and A BBDirection", name.as_str());
    }
}

fn no_conflic(_query: Query<&Name, Or<(With<Speed>, With<Transform>)>>) {}

fn conflic(mut _query: Query<&mut Name, Or<(With<Speed>, With<Transform>)>>) {}

fn some_query(_query: Query<(&Transform, Option<&Name>)>) {}
#[derive(Component)]
struct SomeComponent;

#[derive(Component)]
struct SomeOtherComponent;

fn entity_ref(_query: Query<(Entity, &SomeComponent)>) {}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Size(f32);

fn query_filter(
    mut player_query: Query<(&mut Transform, &Size), With<Player>>,
    obj_query: Query<(&Transform, &Size), Without<Player>>,
) {
    let (mut player_tran, player_size) = player_query.single_mut();

    for (tran, size) in obj_query.iter() {
        if check_colision(&player_tran, player_size, tran, size) {
            move_player(&mut player_tran, player_size, tran, size);
        }
    }
}

fn check_colision(
    player_tran: &Transform,
    player_size: &Size,
    obj_tran: &Transform,
    obj_size: &Size,
) -> bool {
    let player_center = player_tran.translation;
    let player_half_size = player_size.0 * 0.5;
    let obj_center = obj_tran.translation;
    let obj_half_size = obj_size.0 * 0.5;
    if player_center.x + player_half_size < obj_center.x - obj_half_size {
        return false;
    }
    if player_center.x - player_half_size > obj_center.x + obj_half_size {
        return false;
    }
    if player_center.y + player_half_size < obj_center.y - obj_half_size {
        return false;
    }
    if player_center.y - player_half_size > obj_center.y + obj_half_size {
        return false;
    }
    if player_center.z + player_half_size < obj_center.z - obj_half_size {
        return false;
    }
    if player_center.z - player_half_size > obj_center.z + obj_half_size {
        return false;
    }
    true
}

fn move_player(
    player_tran: &mut Transform,
    player_size: &Size,
    obj_tran: &Transform,
    obj_size: &Size,
) {
    let player_center = player_tran.translation;
    let player_half_size = player_size.0 * 0.5;
    let obj_center = obj_tran.translation;
    let obj_half_size = obj_size.0 * 0.5;
    if player_center.x + player_half_size < obj_center.x - obj_half_size {
        player_tran.translation.x += obj_half_size + player_half_size;
    }
    if player_center.x - player_half_size > obj_center.x + obj_half_size {
        player_tran.translation.x -= obj_half_size + player_half_size;
    }
    if player_center.y + player_half_size < obj_center.y - obj_half_size {
        player_tran.translation.y += obj_half_size + player_half_size;
    }
    if player_center.y - player_half_size > obj_center.y + obj_half_size {
        player_tran.translation.y -= obj_half_size + player_half_size;
    }
    if player_center.z + player_half_size < obj_center.z - obj_half_size {
        player_tran.translation.z += obj_half_size + player_half_size;
    }
    if player_center.z - player_half_size > obj_center.z + obj_half_size {
        player_tran.translation.z -= obj_half_size + player_half_size;
    }
}
