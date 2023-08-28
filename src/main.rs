mod animation;
mod sprite_manager;
mod physics;
mod camera;

mod entities { pub mod objects; }

use entities::objects::ObjectsPlugin;
use physics::PhysicsPlugin;
use animation::AnimationPlugin;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_rapier2d::prelude::*;
use crate::sprite_manager::SpriteManagerPlugin;
use crate::camera::CameraPlugin;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;

const WINDOW_BOTTOM_Y: f32 = -WINDOW_HEIGHT / 2.0;
const WINDOW_LEFT_X: f32 = -WINDOW_WIDTH / 2.0;

const COLOR_BACKGROUND: Color = Color::rgb(0.29, 0.31, 0.41);

const FLOOR_THICKNESS: f32 = 10.0;

const BG_WIDTH: f32 = 5120.0;
const BG_HEIGHT: f32 = 432.0;

#[derive(Component)]
enum Direction {
    Right,
    Left,
}

#[derive(Component)]
struct Jump(f32);

fn main() {
    App::new()
        .insert_resource(ClearColor(COLOR_BACKGROUND)) // resource added
        .add_systems(Startup,setup) // new system added
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Platformer".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0),
            ObjectsPlugin,
            AnimationPlugin,
            SpriteManagerPlugin,
            PhysicsPlugin,
            CameraPlugin
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<&KinematicCharacterControllerOutput>
) {
    let background_image = asset_server.load("textures/bg.png");

    commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, WINDOW_BOTTOM_Y  + (FLOOR_THICKNESS / 2.0), 0.0),
                scale: Vec3::new(BG_WIDTH, FLOOR_THICKNESS, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5, 0.5));

    commands
        .spawn(SpriteBundle {
            texture: background_image,
            transform: Transform {
                translation: Vec3::new(
                    BG_WIDTH / 2.0 + WINDOW_LEFT_X,
                    WINDOW_BOTTOM_Y + BG_HEIGHT / 2.0,
                    0.0),
                ..Default::default()
            },
            ..Default::default()
        });

    commands.spawn(Camera2dBundle::default());
}
