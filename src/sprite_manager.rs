use std::time::Duration;
use crate::animation::Animation;
use crate::{Direction, WINDOW_BOTTOM_Y, WINDOW_LEFT_X, SCALE, BG_WIDTH, BG_HEIGHT};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::entities::champi::Champi;

pub struct SpriteManagerPlugin;

const SPRITESHEET_COLS: usize = 5;
const SPRITESHEET_ROWS: usize = 2;
const SPRITE_MARIO_WIDTH: f32 = 13.0;
const SPRITE_MARIO_HEIGHT: f32 = 23.0;

const SPRITE_OFFSET_X: f32 = 25.0;
const SPRITE_OFFSET_Y: f32 = 49.0;
const SPRITE_PADDING_X: f32 = 39.0;
const SPRITE_PADDING_Y: f32 = 40.0;

const SPRITE_IDX_STAND: usize = 0;
const SPRITE_IDX_JUMP: usize = 6;
const SPRITE_IDX_BLOCK_OPENED: usize = 4;

const SPRITE_TILE_WIDTH: f32 = 15.0;
const SPRITE_TILE_HEIGHT: f32 = 15.0;
const SPRITE_TILE_PADDING: f32 = 9.0;
const SPRITE_TILE_PADDING_Y: f32 = 6.0;

impl Plugin for SpriteManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                setup,
                add_world_image,
                add_block_to_world,
                add_champi
            ))
            .add_systems(
            Update,
            (
                apply_jump_sprite,
                apply_idle_sprite,
                update_sprite_direction,
                apply_opened_block_sprite
            ),
        );
    }
}

#[derive(Component)] // TODO: Reflect
pub struct Block {
    pub opened: bool,
}

fn setup(
    mut commands: Commands,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    server: Res<AssetServer>,
) {
    let image_handle: Handle<Image> = server.load("spritesheets/spritesheet_Mario.png");
    let texture_atlas = TextureAtlas::from_grid(
        image_handle,
        Vec2::new(SPRITE_MARIO_WIDTH, SPRITE_MARIO_HEIGHT),
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
                    2.0,
                    2.0,
                    1.0,
                ),
                translation: Vec3::new(WINDOW_LEFT_X + 300.0, WINDOW_BOTTOM_Y + 300.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(
            SPRITE_MARIO_WIDTH / 2.0,
            SPRITE_MARIO_HEIGHT / 2.0,
        ))
        .insert(KinematicCharacterController{
            filter_groups: Option::from(CollisionGroups::new(
                Group::GROUP_2,
                Group::ALL - Group::GROUP_3
            )),
            ..Default::default()
        })
        .insert(Direction::Right)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ActiveHooks::MODIFY_SOLVER_CONTACTS);
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

fn add_world_image(mut commands: Commands, asset_server: Res<AssetServer>) {
    let world_image = asset_server.load("textures/world.png");

    commands.spawn(SpriteBundle {
        texture: world_image,
        transform: Transform {
            scale: Vec3::new(SCALE, SCALE, 1.0),
            translation: Vec3::new(BG_WIDTH + WINDOW_LEFT_X, WINDOW_BOTTOM_Y + BG_HEIGHT, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn add_block_to_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image_handle: Handle<Image> = asset_server.load("spritesheets/tiles.png");
    let texture_atlas = TextureAtlas::from_grid(
        image_handle,
        Vec2::new(SPRITE_TILE_WIDTH, SPRITE_TILE_HEIGHT),
        4,
        2,
        Option::from(Vec2::new(SPRITE_TILE_PADDING, SPRITE_TILE_PADDING_Y)),
        Option::from(Vec2::new(8.0, 248.0)),
    );
    let atlas_handle = atlases.add(texture_atlas);

    const CYCLE_DELAY: Duration = Duration::from_millis(500);

    const SPRITE_IDX_ANIM: &[usize] = &[0, 1, 2, 3];

    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(SPRITE_IDX_STAND),
            texture_atlas: atlas_handle,
            transform: Transform {
                scale: Vec3::new(
                    2.0,
                    2.0,
                    1.0,
                ),
                translation: Vec3::new(WINDOW_LEFT_X + 1216.0, WINDOW_BOTTOM_Y + 224.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(
            SPRITE_TILE_WIDTH / 2.0,
            SPRITE_TILE_HEIGHT / 2.0,
        ))
        .insert(Block{
            opened: false
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Animation::new(SPRITE_IDX_ANIM, CYCLE_DELAY));
}

fn apply_opened_block_sprite(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Block,
        &mut TextureAtlasSprite,
    )>,
) {
    if query.is_empty() {
        return;
    }

    let (block_entity, block, mut sprite) = query.single_mut();
    if block.opened {
        commands.entity(block_entity).remove::<Animation>();
        sprite.index = SPRITE_IDX_BLOCK_OPENED;
    }
}

fn add_champi(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image_handle: Handle<Image> = asset_server.load("spritesheets/tiles.png");
    let texture_atlas = TextureAtlas::from_grid(
        image_handle,
        Vec2::new(SPRITE_TILE_WIDTH, SPRITE_TILE_HEIGHT),
        4,
        2,
        Option::from(Vec2::new(SPRITE_TILE_PADDING, SPRITE_TILE_PADDING_Y)),
        Option::from(Vec2::new(8.0, 248.0)),
    );
    let atlas_handle = atlases.add(texture_atlas);

    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(5),
            texture_atlas: atlas_handle,
            transform: Transform {
                scale: Vec3::new(
                    2.0,
                    2.0,
                    1.0,
                ),
                translation: Vec3::new(WINDOW_LEFT_X + 1216.0, WINDOW_BOTTOM_Y + 230.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(GravityScale(2.0))
        .insert(Collider::cuboid(
            SPRITE_TILE_WIDTH / 2.0,
            SPRITE_TILE_HEIGHT / 2.0,
        ))
        .insert(Champi{
            color: "red".to_string(),
            direction: Direction::Right,
            visible: false,
            upcoming: false
        })
        .insert(CollisionGroups::new(
            Group::GROUP_3,
            Group::GROUP_1
        ))
    ;
}
