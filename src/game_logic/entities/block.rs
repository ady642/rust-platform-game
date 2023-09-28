use std::time::Duration;
use bevy::prelude::*;
use bevy_rapier2d::control::KinematicCharacterControllerOutput;
use bevy_rapier2d::prelude::{Collider, RigidBody};
use crate::game_logic::entities::champi::{Champi, ChampiFactory};
use crate::rendering::animation::Animation;
use crate::rendering::sprite_manager::{SPRITE_TILE_HEIGHT, SPRITE_TILE_WIDTH};

#[derive(Component)]
pub struct Opened(f32);

#[derive(Component)]
pub struct Block {
    id: i32,
    pub opened: bool,
}

#[derive(Bundle)]
pub struct BlockFactory {
    block: Block,
    sprite_bundle: SpriteSheetBundle,
    body: RigidBody,
    collider: Collider,
    animation: Animation
}

impl BlockFactory {
    pub fn new(id: i32, texture_atlas: Handle<TextureAtlas>, x: f32, y: f32) -> Self {
        const CYCLE_DELAY: Duration = Duration::from_millis(500);

        const SPRITE_IDX_ANIM: &[usize] = &[0, 1, 2, 3];
        Self {
            sprite_bundle: SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(0),
                texture_atlas,
                transform: Transform {
                    scale: Vec3::new(
                        2.0,
                        2.0,
                        1.0,
                    ),
                    translation: Vec3::new(x, y, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            body: RigidBody::Fixed,
            collider: Collider::cuboid(
                SPRITE_TILE_WIDTH / 2.0,
                SPRITE_TILE_HEIGHT / 2.0,
            ),
            block: Block {
                id,
                opened: false
            },
            animation: Animation::new(SPRITE_IDX_ANIM, CYCLE_DELAY),
        }
    }
}

pub fn detect_collision_from_below_on_block(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Block), Without<Opened>>,
    mut character_controller_outputs: Query<&mut KinematicCharacterControllerOutput>,
    mut champi_query: Query<&mut Champi>,
) {
    if champi_query.is_empty() {
        return;
    }

    for(mut champi) in champi_query.iter_mut() {
        for (entity, mut block) in query.iter_mut() {
            for mut output in character_controller_outputs.iter_mut() {
                for collision in &output.collisions {
                    println!("collision entity: {:?}", collision.entity);
                    println!("entity: {:?}", entity);
                    println!("collision toi: {:?}", collision.toi);
                    if collision.toi.normal1.y == -1.0 {
                        commands.entity(entity).insert(Opened(0.0));
                        if block.id == champi.block_id {
                            champi.visible = true;
                            champi.upcoming = true;
                        }
                    }
                }
            }
        }
    }
}
