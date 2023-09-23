use std::time::Duration;
use crate::rendering::animation::Animation;
use crate::{WINDOW_BOTTOM_Y, WINDOW_LEFT_X, SCALE, BG_WIDTH, BG_HEIGHT};
use bevy::prelude::*;
use bevy::prelude::Visibility::{Hidden, Visible};
use bevy_rapier2d::prelude::*;
use crate::game_logic::entities::block::{Block, BlockFactory};
use crate::game_logic::entities::champi::{Champi, ChampiFactory};
use crate::game_logic::entities::mario::{COLLISION_GROUPS_DEFAULT, Direction, Mario};

pub struct SpriteManagerPlugin;

pub const SPRITESHEET_COLS: usize = 5;
pub const SPRITESHEET_ROWS: usize = 2;
const SPRITE_MARIO_WIDTH: f32 = 13.0;
const SPRITE_MARIO_HEIGHT: f32 = 23.0;

pub const SPRITE_OFFSET_X: f32 = 25.0;
pub const SPRITE_OFFSET_Y: f32 = 49.0;
pub const SPRITE_PADDING_X: f32 = 39.0;
pub const SPRITE_PADDING_Y: f32 = 40.0;

const SPRITE_IDX_STAND: usize = 0;
const SPRITE_IDX_JUMP: usize = 6;
const SPRITE_IDX_BLOCK_OPENED: usize = 4;

pub const SPRITE_TILE_WIDTH: f32 = 15.0;
pub const SPRITE_TILE_HEIGHT: f32 = 15.0;
const SPRITE_TILE_PADDING: f32 = 9.0;
const SPRITE_TILE_PADDING_Y: f32 = 6.0;

impl Plugin for SpriteManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                setup,
                add_world_image,
                add_block_to_world,
                add_champi,
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
        .spawn(Mario::new(atlas_handle, WINDOW_LEFT_X + 300.0, WINDOW_BOTTOM_Y + 300.0 ))
        .insert(KinematicCharacterController {
            filter_groups: Option::from(COLLISION_GROUPS_DEFAULT),
            ..Default::default()
        });
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

    for (player, output, mut sprite) in query.iter_mut() {
        if !output.grounded {
            commands.entity(player).remove::<Animation>();
            sprite.index = SPRITE_IDX_JUMP
        }
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

    for (player, output, mut sprite) in query.iter_mut() {
        if output.desired_translation.x == 0.0 && output.grounded {
            commands.entity(player).remove::<Animation>();
            sprite.index = SPRITE_IDX_STAND
        }
    }
}

fn update_sprite_direction(mut query: Query<(&mut TextureAtlasSprite, &Direction)>) {
    if query.is_empty() {
        return;
    }

    for (mut sprite, direction) in query.iter_mut() {
        match direction {
            Direction::Right => sprite.flip_x = true,
            Direction::Left => sprite.flip_x = false,
        }

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

    let atlas_handle1 = atlas_handle.clone();
    let atlas_handle2 = atlas_handle.clone();
    let atlas_handle3 = atlas_handle.clone();

    commands
        .spawn(BlockFactory::new(1, atlas_handle1, WINDOW_LEFT_X + 1216.0, WINDOW_BOTTOM_Y + 224.0 ));
    commands
        .spawn(BlockFactory::new(2, atlas_handle2, WINDOW_LEFT_X + 1712.0, WINDOW_BOTTOM_Y + 176.0 ));
    commands
        .spawn(BlockFactory::new(3, atlas_handle3, WINDOW_LEFT_X + 1744.0, WINDOW_BOTTOM_Y + 176.0 ));
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

    for (block_entity, block, mut sprite) in query.iter_mut() {
        if block.opened {
            commands.entity(block_entity).remove::<Animation>();
            sprite.index = SPRITE_IDX_BLOCK_OPENED;
        }
    }
}

fn add_champi(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image_handle: Handle<Image> = asset_server.get_handle("spritesheets/tiles.png");
    let texture_atlas = TextureAtlas::from_grid(
        image_handle,
        Vec2::new(SPRITE_TILE_WIDTH, SPRITE_TILE_HEIGHT),
        4,
        2,
        Option::from(Vec2::new(SPRITE_TILE_PADDING, SPRITE_TILE_PADDING_Y)),
        Option::from(Vec2::new(8.0, 248.0)),
    );


    let atlas_handle = atlases.add(texture_atlas);

    let atlas_handle1 = atlas_handle.clone();
    let atlas_handle2 = atlas_handle.clone();

    commands
        .spawn(ChampiFactory::new(1, atlas_handle, WINDOW_LEFT_X + 1216.0, WINDOW_BOTTOM_Y + 230.0));

    commands
        .spawn(ChampiFactory::new(2, atlas_handle1, WINDOW_LEFT_X + 1712.0, WINDOW_BOTTOM_Y + 176.0));

    commands
        .spawn(ChampiFactory::new(3, atlas_handle2, WINDOW_LEFT_X + 1744.0, WINDOW_BOTTOM_Y + 176.0));
}

