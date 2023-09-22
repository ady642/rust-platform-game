use std::time::Duration;
use bevy::asset::Handle;
use bevy::prelude::*;
use bevy::prelude::Visibility::Hidden;
use bevy::sprite::SpriteSheetBundle;
use bevy_rapier2d::prelude::*;
use crate::{WINDOW_BOTTOM_Y};
use crate::rendering::sprite_manager::{SPRITE_TILE_HEIGHT, SPRITE_TILE_WIDTH};

#[derive(Component)]
pub struct Champi {
    pub visible: bool,
    pub upcoming: bool,
    pub upcoming_height: f32,
}

#[derive(Bundle)]
pub struct ChampiFactory {
    champi: Champi,
    sprite_bundle: SpriteSheetBundle,
    body: RigidBody,
    collider: Collider,
    gravity: GravityScale,
}

impl ChampiFactory {
    pub fn new(texture_atlas: Handle<TextureAtlas>, x: f32, y: f32) -> Self {
        const CYCLE_DELAY: Duration = Duration::from_millis(500);

        const SPRITE_IDX_ANIM: &[usize] = &[0, 1, 2, 3];
        Self {
            sprite_bundle: SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(5),
                texture_atlas,
                visibility: Hidden,
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
            gravity: GravityScale(2.0),
            champi: Champi {
                visible: false,
                upcoming: false,
                upcoming_height: y + SPRITE_TILE_HEIGHT * 2.0,
            },
        }
    }
}

pub fn apply_translation_to_champi(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Champi, &mut Visibility)>,
) {
    for (entity, mut transform, mut champi, mut visibility) in query.iter_mut() {
        if champi.visible {
            *visibility = Visibility::Visible;

            if champi.upcoming {
                transform.translation.y += 0.5;

                println!("{} {}", transform.translation.y, champi.upcoming_height);

                if transform.translation.y >= champi.upcoming_height {
                    champi.upcoming = false;
                    commands.entity(entity).remove::<RigidBody>();
                    commands.entity(entity).insert(RigidBody::Dynamic);
                }
            } else {
                transform.translation.x += 1.0;
                transform.rotation.w = 0.0
            }
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
