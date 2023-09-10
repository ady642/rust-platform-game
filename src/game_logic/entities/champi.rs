use bevy::prelude::{Commands, Component, Entity, Query, Transform};
use bevy_rapier2d::prelude::*;
use crate::{WINDOW_BOTTOM_Y};

#[derive(Component)]
pub struct Champi {
    pub visible: bool,
    pub upcoming: bool
}


pub fn apply_translation_to_champi(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Champi)>,
) {
    for (entity, mut transform, mut champi) in query.iter_mut() {
        if champi.visible {
            if champi.upcoming {
                transform.translation.y += 0.5;
                if transform.translation.y >= WINDOW_BOTTOM_Y + 250.0 {
                    champi.upcoming = false;
                    commands.entity(entity).remove::<RigidBody>();
                    commands.entity(entity).insert(RigidBody::Dynamic);
                }
            } else {
                transform.translation.x += 1.0;
                transform.rotation.w = 0.0
            }
        }
    }
}
