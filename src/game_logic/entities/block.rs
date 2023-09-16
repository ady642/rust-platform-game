use bevy::prelude::*;
use bevy_rapier2d::control::KinematicCharacterControllerOutput;
use crate::game_logic::entities::champi::Champi;

#[derive(Component)]
pub struct Block {
    pub opened: bool,
}

pub fn detect_collision_from_below_on_block(
    mut query: Query<(Entity, &mut Block)>,
    mut character_controller_outputs: Query<&mut KinematicCharacterControllerOutput>,
    mut champi_query: Query<&mut Champi>,
) {
    let mut champi = champi_query.single_mut();

    for (entity, mut block) in query.iter_mut() {
        if block.opened {
            return
        }
        for mut output in character_controller_outputs.iter_mut() {
            for collision in &output.collisions {
                if collision.entity == entity && collision.toi.normal1.y == -1.0 {
                    block.opened = true;
                    champi.visible = true;
                    champi.upcoming = true
                }
            }
        }
    }
}
