mod animation;
mod camera;
mod physics;
mod sprite_manager;

mod entities {
    pub mod objects;
}

use std::ops::Deref;
use animation::AnimationPlugin;
use entities::objects::ObjectsPlugin;
use physics::PhysicsPlugin;

use crate::camera::CameraPlugin;
use crate::sprite_manager::SpriteManagerPlugin;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_rapier2d::parry::partitioning::IndexedData;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::pipeline::PhysicsHooks;
use bevy_rapier2d::rapier::prelude::ContactModificationContext;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;

const WINDOW_BOTTOM_Y: f32 = -WINDOW_HEIGHT / 2.0;
const WINDOW_LEFT_X: f32 = -WINDOW_WIDTH / 2.0;

const COLOR_BACKGROUND: Color = Color::rgb(0.29, 0.31, 0.41);

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
        .add_startup_systems(setup) //
        .add_systems(Update, display_contact_info)
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Platformer".to_string(),
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                    resizable: false,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0),
            RapierDebugRenderPlugin::default(),
            AnimationPlugin,
            SpriteManagerPlugin,
            PhysicsPlugin,
            CameraPlugin,
        ))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let world_image = asset_server.load("textures/world.png");

    let vertices = vec![
        Vec2::new(WINDOW_LEFT_X, WINDOW_BOTTOM_Y + 94.0),
        Vec2::new(WINDOW_LEFT_X + 3744.0, WINDOW_BOTTOM_Y + 94.0),
        Vec2::new(WINDOW_LEFT_X + 608.0, WINDOW_BOTTOM_Y + 190.0),
        Vec2::new(WINDOW_LEFT_X + 1116.0, WINDOW_BOTTOM_Y + 190.0),
    ];

    let indices = vec![
        [0u32, 1u32],
        [2u32, 3u32],
    ];

    commands
        .spawn(SpriteBundle {
            transform: Transform {
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::polyline(vertices, Option::from(indices)))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(CollisionGroups::new(Group::GROUP_2, Group::GROUP_1))
    ;

    commands.spawn(SpriteBundle {
        texture: world_image,
        transform: Transform {
            scale: Vec3::new(2.0, 2.0, 1.0),
            translation: Vec3::new(BG_WIDTH + WINDOW_LEFT_X, WINDOW_BOTTOM_Y + BG_HEIGHT, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(Camera2dBundle::default());
}

fn display_contact_info(
    mut character_controller: Query<&mut KinematicCharacterController>,
    mut character_controller_output: Query<&mut KinematicCharacterControllerOutput>,
) {
    let mut cc = character_controller.single_mut();
    for mut cco in character_controller_output.iter_mut() {
        for collision in cco.collisions.iter_mut() {
            println!("{:?}", collision.toi.status);
            if collision.toi.normal2.y == 1.0 {
                cc.filter_flags = QueryFilterFlags::EXCLUDE_FIXED;
            } else if collision.toi.status == TOIStatus::Converged {
                cc.filter_flags = QueryFilterFlags::default();
            }
        }
    }
}

