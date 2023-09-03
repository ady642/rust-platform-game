pub struct PhysicsPlugin;

use crate::animation::Animation;
use crate::{BG_WIDTH, Direction, Jump, SCALE, WINDOW_BOTTOM_Y, WINDOW_LEFT_X};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;
use bevy::ecs::system::lifetimeless::SCommands;

const PLAYER_VELOCITY_X: f32 = 400.0;
const PLAYER_VELOCITY_Y: f32 = 850.0;
const MAX_JUMP_HEIGHT: f32 = 230.0;
const SPRITE_IDX_WALKING: &[usize] = &[0, 4, 3];
const CYCLE_DELAY: Duration = Duration::from_millis(70);

const SPRITE_TILE_WIDTH: f32 = 16.0;
const SPRITE_TILE_HEIGHT: f32 = 32.0;

#[derive(Component)]
pub struct Player {
    pub entity: Entity,
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
            Update,
            (
                movement,
                jump,
                rise,
                fall,
                apply_movement_animation,
                update_direction,
            ),
        );
    }
}

fn movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController>,
) {
    let mut player = query.single_mut();

    let mut movement = 0.0;

    if input.pressed(KeyCode::Right) {
        movement += time.delta_seconds() * PLAYER_VELOCITY_X;
    }

    if input.pressed(KeyCode::Left) {
        movement += time.delta_seconds() * PLAYER_VELOCITY_X * -1.0;
    }

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(movement, vec.y)), // update if it already exists
        None => player.translation = Some(Vec2::new(movement, 0.0)),
    }
}

fn jump(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut query_character_controller: Query<&mut KinematicCharacterController>,
    query: Query<
        (Entity, &KinematicCharacterControllerOutput),
        (With<KinematicCharacterController>, Without<Jump>),
    >
) {
    if query.is_empty() {
        return;
    }
    let mut character_controller = query_character_controller.single_mut();


    let (player, output) = query.single();

    if input.pressed(KeyCode::Up) && output.grounded {
        commands.entity(player).insert(Jump(0.0));
        character_controller.filter_flags = QueryFilterFlags::all();
    }
}

fn rise(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut KinematicCharacterController, &mut Jump)>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut player, mut jump) = query.single_mut();

    let mut movement = time.delta().as_secs_f32() * PLAYER_VELOCITY_Y;

    if movement + jump.0 >= MAX_JUMP_HEIGHT {
        movement = MAX_JUMP_HEIGHT - jump.0;
        commands.entity(entity).remove::<Jump>();
        player.filter_flags = QueryFilterFlags::default();
    }

    jump.0 += movement;

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
        None => player.translation = Some(Vec2::new(0.0, movement)),
    }
}

fn fall(time: Res<Time>, mut query: Query<&mut KinematicCharacterController, Without<Jump>>) {
    if query.is_empty() {
        return;
    }

    let mut player = query.single_mut();

    // I am using two-thirds of the Y-velocity since I want the character to fall slower than it rises
    let movement = time.delta().as_secs_f32() * (PLAYER_VELOCITY_Y / 1.5) * -1.0;

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
        None => player.translation = Some(Vec2::new(0.0, movement)),
    }
}

fn apply_movement_animation(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput), Without<Animation>>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = query.single();
    if output.desired_translation.x != 0.0 && output.grounded {
        commands
            .entity(player)
            .insert(Animation::new(SPRITE_IDX_WALKING, CYCLE_DELAY));
    }
}

fn update_direction(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput)>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = query.single();

    if output.desired_translation.x > 0.0 {
        commands.entity(player).insert(Direction::Right);
    } else if output.desired_translation.x < 0.0 {
        commands.entity(player).insert(Direction::Left);
    }
}

pub fn world_to_vec() -> (Vec<Vec2>, Vec<[u32; 2]>) {
    fn build_point(x: f32, y: f32) -> Vec2 {
        Vec2::new(WINDOW_LEFT_X + x * SCALE , WINDOW_BOTTOM_Y + y * SCALE) // to put the origin in the bottom left corner
    }

    let platform_3 = vec![ // Create factories to create each element (pipe, platform, etc)
                           build_point(2992.0, 96.0),
                           build_point(3071.0, 96.0),
    ];

    let platform_4 = [
        build_point(3024.0, 144.0),
        build_point(3150.0, 144.0),
    ];

    let platform_5 = [
        build_point(3120.0, 96.0),
        build_point(3215.0, 96.0),
    ];

    let platform_6 = [
        build_point(3392.0, 128.0),
        build_point(3551.0, 128.0),
    ];

    let mut vertices = vec![
        build_point(0.0, 47.0),
        build_point(1872.0, 47.0),
        build_point(304.0, 95.0),
        build_point(558.0, 95.0),
        build_point(704.0, 112.0),
        build_point(815.0, 112.0),
        build_point(1872.0, 64.0),
        build_point(1952.0, 64.0),
        build_point(1952.0, 80.0),
        build_point(2015.0, 80.0),
        build_point(2015.0, 96.0),
        build_point(2064.0, 96.0),
        build_point(2064.0, 113.0),
        build_point(2287.0, 113.0),
        build_point(2287.0, 47.0),
        build_point(3488.0, 47.0),
        build_point(3488.0, 79.0),
        build_point(3647.0, 79.0),
        build_point(3647.0, 47.0),
        build_point(BG_WIDTH, 47.0),
    ];

    vertices.extend(platform_3);
    vertices.extend(platform_4);
    vertices.extend(platform_5);
    vertices.extend(platform_6);

    let indices = vec![
        [0u32, 1u32],
        [2u32, 3u32],
        [4u32, 5u32],
        [6u32, 7u32],
        [8u32, 9u32],
        [10u32, 11u32],
        [12u32, 13u32],
        [13u32, 14u32],
        [14u32, 15u32],
        [16u32, 17u32],
        [18u32, 19u32],
        [20u32, 21u32],
        [22u32, 23u32],
        [24u32, 25u32],
        [26u32, 27u32]
    ];

    return (vertices, indices);
}
