pub struct PhysicsPlugin;

use crate::{BG_WIDTH};
use bevy::prelude::*;
use crate::game_logic::entities::block::{detect_collision_from_below_on_block};
use crate::game_logic::entities::champi::{apply_translation_to_champi};
use crate::game_logic::entities::mario::{apply_movement_animation, fall, jump, movement, rise, update_direction};
use crate::utils::build_point;

const SPRITE_TILE_WIDTH: f32 = 16.0;
const SPRITE_TILE_HEIGHT: f32 = 32.0;

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
                detect_collision_from_below_on_block,
                apply_translation_to_champi
            ),
        );
    }
}

pub fn world_to_vec() -> (Vec<Vec2>, Vec<[u32; 2]>) {
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
