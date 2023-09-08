use bevy::prelude::Component;
use crate::Direction;

#[derive(Component)]
pub struct Champi {
    pub color: String,
    pub direction: Direction,
    pub visible: bool,
    pub upcoming: bool
}
