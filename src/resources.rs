use bevy::prelude::*;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DefaultFont>();

        app.insert_resource(CustomColors([
            Color::GOLD,
            Color::INDIGO,
            Color::LIME_GREEN,
            Color::BISQUE,
            Color::AQUAMARINE,
            Color::MAROON,
            Color::SEA_GREEN,
            Color::SALMON,
            Color::YELLOW_GREEN,
            Color::ALICE_BLUE,
        ]));

        app.init_resource::<ColorWheel>();

        app.add_system(add_new_custom);
    }
}

#[derive(Resource)]
pub struct CustomColors([Color; 10]);

#[derive(Resource)]
pub struct ColorWheel(Vec<Handle<StandardMaterial>>);

#[derive(Resource)]
pub struct DefaultFont(pub Handle<Font>);

#[allow(dead_code)]
pub struct FancyFont(pub Handle<Font>);

impl FromWorld for DefaultFont {
    fn from_world(world: &mut World) -> Self {
        DefaultFont(
            world
                .get_resource::<AssetServer>()
                .unwrap()
                .load("./Raleway-Bold.ttf"),
        )
    }
}

impl FromWorld for ColorWheel {
    fn from_world(world: &mut World) -> Self {
        let mut matts = Vec::with_capacity(29);
        let custom_colors = world.remove_resource::<CustomColors>();
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        matts.push(materials.add(StandardMaterial {
            base_color: Color::rgb(0., 0., 0.),
            ..Default::default()
        }));
        matts.push(materials.add(StandardMaterial {
            base_color: Color::rgb(0.5, 0.5, 0.5),
            ..Default::default()
        }));
        matts.push(materials.add(StandardMaterial {
            base_color: Color::rgb(1., 1., 1.),
            ..Default::default()
        }));
        for i in 0..4 {
            matts.push(materials.add(StandardMaterial {
                base_color: Color::rgb(1. - (i as f32 * 0.25), 0., 0.),
                ..Default::default()
            }));
            matts.push(materials.add(StandardMaterial {
                base_color: Color::rgb(0., 1. - (i as f32 * 0.25), 0.),
                ..Default::default()
            }));
            matts.push(materials.add(StandardMaterial {
                base_color: Color::rgb(0., 0., 1. - (i as f32 * 0.25)),
                ..Default::default()
            }));
            matts.push(materials.add(StandardMaterial {
                base_color: Color::rgb(1., 1. - (i as f32 * 0.25), 0.),
                ..Default::default()
            }));
            matts.push(materials.add(StandardMaterial {
                base_color: Color::rgb(1. - (i as f32 * 0.25), 1., 0.),
                ..Default::default()
            }));
            matts.push(materials.add(StandardMaterial {
                base_color: Color::rgb(1. - (i as f32 * 0.25), 0., 1.),
                ..Default::default()
            }));
            matts.push(materials.add(StandardMaterial {
                base_color: Color::rgb(1., 0., 1. - (i as f32 * 0.25)),
                ..Default::default()
            }));
            matts.push(materials.add(StandardMaterial {
                base_color: Color::rgb(0., 1., 1. - (i as f32 * 0.25)),
                ..Default::default()
            }));
            matts.push(materials.add(StandardMaterial {
                base_color: Color::rgb(0., 1. - (i as f32 * 0.25), 1.),
                ..Default::default()
            }));
        }

        if let Some(colors) = custom_colors {
            for i in 0..colors.0.len() {
                matts.push(materials.add(StandardMaterial {
                    base_color: colors.0[i],
                    ..Default::default()
                }));
            }
            world.remove_resource::<CustomColors>();
        }

        ColorWheel(matts)
    }
}

impl ColorWheel {
    pub fn get_color<'a>(&self, set: ColorSet, input: usize) -> Handle<StandardMaterial> {
        use rand::Rng;
        let index = match set {
            ColorSet::Red => input * 3,
            ColorSet::Green => input * 3 + 1,
            ColorSet::Blue => input * 3 + 2,
            ColorSet::Custom(set) => 39 + input + set as usize * 10,
            ColorSet::Random => rand::thread_rng().gen_range(0..self.0.len()),
        };
        let index = if index >= self.0.len() { 0 } else { index };
        self.0[index].clone()
    }
}

pub enum ColorSet {
    Red,
    Green,
    Blue,
    Custom(u8),
    Random,
}

fn add_new_custom(
    mut commands: Commands,
    mut color_weel: ResMut<ColorWheel>,
    new_custom: Option<Res<CustomColors>>,
    mut matts: ResMut<Assets<StandardMaterial>>,
) {
    if new_custom.is_none() {
        return;
    }
    for color in new_custom.unwrap().0.iter() {
        color_weel.0.push(matts.add(StandardMaterial {
            base_color: *color,
            ..Default::default()
        }));
    }

    commands.remove_resource::<CustomColors>();
}
