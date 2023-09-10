#![feature(exact_size_is_empty)]

mod utils;

mod game_logic {
    pub mod entities {
        pub mod objects;
        pub mod champi;
        pub mod mario;
        pub mod block;
    }

    pub mod world {
        pub mod physics;
    }
}

mod rendering {
    pub mod camera;
    pub mod sprite_manager;
    pub mod animation;
}

use std::ops::Deref;
use rendering::animation::AnimationPlugin;
use game_logic::world::physics::PhysicsPlugin;

use crate::rendering::camera::CameraPlugin;
use crate::rendering::sprite_manager::SpriteManagerPlugin;
use bevy::prelude::*;
use bevy::reflect::List;
use bevy::window::WindowResolution;
use bevy_rapier2d::parry::partitioning::IndexedData;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::pipeline::PhysicsHooks;
use crate::game_logic::entities::objects::ObjectsPlugin;
use crate::game_logic::world::physics::world_to_vec;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;

const WINDOW_BOTTOM_Y: f32 = -WINDOW_HEIGHT / 2.0;
const WINDOW_LEFT_X: f32 = -WINDOW_WIDTH / 2.0;

const COLOR_BACKGROUND: Color = Color::rgb(0.29, 0.31, 0.41);

const BG_WIDTH: f32 = 5120.0;
const BG_HEIGHT: f32 = 432.0;

const SCALE: f32 = 2.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(COLOR_BACKGROUND)) // resource added
        .add_systems(Startup, setup) //
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Platformer".to_string(),
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                    resizable: true,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0),
            AnimationPlugin,
            SpriteManagerPlugin,
            PhysicsPlugin,
            CameraPlugin,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    let (vertices, indices) = world_to_vec();

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
        .insert(CollisionGroups::new(
            Group::GROUP_1,
            Group::ALL
        ))
    ;

    commands.spawn(Camera2dBundle::default());
}
