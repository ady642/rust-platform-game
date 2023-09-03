use bevy::math::Vec2;
use crate::{SCALE, WINDOW_BOTTOM_Y, WINDOW_LEFT_X};

pub fn build_point(x: f32, y: f32) -> Vec2 {
    Vec2::new(WINDOW_LEFT_X + x * SCALE , WINDOW_BOTTOM_Y + y * SCALE) // to put the origin in the bottom left corner
}
