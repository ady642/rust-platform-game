use crate::animation::Animation;
use crate::{Direction, WINDOW_BOTTOM_Y, WINDOW_LEFT_X};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct SpriteManagerPlugin;

const SPRITESHEET_COLS: usize = 5;
const SPRITESHEET_ROWS: usize = 2;
const SPRITE_TILE_WIDTH: f32 = 16.0;
const SPRITE_TILE_HEIGHT: f32 = 32.0;

const SPRITE_RENDER_WIDTH: f32 = 32.0;
const SPRITE_RENDER_HEIGHT: f32 = 64.0;

const SPRITE_OFFSET_X: f32 = 23.0;
const SPRITE_OFFSET_Y: f32 = 583.0;
const SPRITE_PADDING_X: f32 = 36.0;
const SPRITE_PADDING_Y: f32 = 36.0;

const SPRITE_IDX_STAND: usize = 0;
const SPRITE_IDX_JUMP: usize = 6;

impl Plugin for SpriteManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                apply_jump_sprite,
                apply_idle_sprite,
                update_sprite_direction,
            ),
        );
    }
}

fn setup(
    mut commands: Commands,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    server: Res<AssetServer>,
) {
    let image_handle: Handle<Image> = server.load("spritesheets/spritesheet_Mario.png");
    let texture_atlas = TextureAtlas::from_grid(
        image_handle,
        Vec2::new(SPRITE_TILE_WIDTH, SPRITE_TILE_HEIGHT),
        SPRITESHEET_COLS,
        SPRITESHEET_ROWS,
        Option::from(Vec2::new(SPRITE_PADDING_X, SPRITE_PADDING_Y)),
        Option::from(Vec2::new(SPRITE_OFFSET_X, SPRITE_OFFSET_Y)),
    );
    let atlas_handle = atlases.add(texture_atlas);

    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(SPRITE_IDX_STAND),
            texture_atlas: atlas_handle,
            transform: Transform {
                scale: Vec3::new(
                    SPRITE_RENDER_WIDTH / SPRITE_TILE_WIDTH,
                    SPRITE_RENDER_HEIGHT / SPRITE_TILE_HEIGHT,
                    1.0,
                ),
                translation: Vec3::new(WINDOW_LEFT_X + 300.0, WINDOW_BOTTOM_Y + 300.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(
            SPRITE_TILE_WIDTH / 2.0,
            SPRITE_TILE_HEIGHT / 2.0,
        ))
        .insert(KinematicCharacterController::default())
        .insert(Direction::Right); // default direction
}

fn apply_jump_sprite(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &KinematicCharacterControllerOutput,
        &mut TextureAtlasSprite,
    )>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output, mut sprite) = query.single_mut();
    if !output.grounded {
        commands.entity(player).remove::<Animation>();
        sprite.index = SPRITE_IDX_JUMP
    }
}

fn apply_idle_sprite(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &KinematicCharacterControllerOutput,
        &mut TextureAtlasSprite,
    )>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output, mut sprite) = query.single_mut();
    if output.desired_translation.x == 0.0 && output.grounded {
        commands.entity(player).remove::<Animation>();
        sprite.index = SPRITE_IDX_STAND
    }
}

fn update_sprite_direction(mut query: Query<(&mut TextureAtlasSprite, &Direction)>) {
    if query.is_empty() {
        return;
    }

    let (mut sprite, direction) = query.single_mut();

    match direction {
        Direction::Right => sprite.flip_x = true,
        Direction::Left => sprite.flip_x = false,
    }
}
