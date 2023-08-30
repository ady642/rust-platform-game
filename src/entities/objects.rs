use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const COLOR_OBJECT: Color = Color::rgb(0.13, 0.13, 0.23);

#[derive(Bundle)]
struct ObjectBundle {
    sprite_bundle: SpriteBundle,
    body: RigidBody,
    collider: Collider,
}

impl ObjectBundle {
    fn new(x: f32, scale: Vec3) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: COLOR_OBJECT,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(x, 432.0, 0.0),
                    scale,
                    ..Default::default()
                },
                ..Default::default()
            },
            body: RigidBody::Dynamic,
            collider: Collider::cuboid(0.5, 0.5),
        }
    }
}

pub struct ObjectsPlugin;

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(ObjectBundle::new(-100.0, Vec3::new(75.0, 200.0, 1.0)));
    commands.spawn(ObjectBundle::new(100.0, Vec3::new(50.0, 350.0, 1.0)));
    commands.spawn(ObjectBundle::new(350.0, Vec3::new(150.0, 250.0, 1.0)));
}
