use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::{WINDOW_BOTTOM_Y, WINDOW_LEFT_X, Direction};

pub struct SpriteManagerPlugin;

const SPRITESHEET_COLS: usize = 7;
const SPRITESHEET_ROWS: usize = 8;
const SPRITE_TILE_WIDTH: f32 = 128.0;
const SPRITE_TILE_HEIGHT: f32 = 256.0;
const SPRITE_IDX_STAND: usize = 28;

const SPRITE_RENDER_WIDTH: f32 = 64.0;
const SPRITE_RENDER_HEIGHT: f32 = 128.0;


impl Plugin for SpriteManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,setup);
    }
}

fn setup(
    mut commands: Commands,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    server: Res<AssetServer>
) {

    let image_handle: Handle<Image> = server.load("spritesheets/spritesheet_players.png");
    let texture_atlas = TextureAtlas::from_grid(
        image_handle,
        Vec2::new(SPRITE_TILE_WIDTH, SPRITE_TILE_HEIGHT),
        SPRITESHEET_COLS,
        SPRITESHEET_ROWS,
        None,
        None,
    );
    let atlas_handle = atlases.add(texture_atlas);

    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(SPRITE_IDX_STAND),
            texture_atlas: atlas_handle,
            transform: Transform {
                translation: Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 300.0, 0.0),
                scale: Vec3::new( // scale added
                                  SPRITE_RENDER_WIDTH / SPRITE_TILE_WIDTH,
                                  SPRITE_RENDER_HEIGHT / SPRITE_TILE_HEIGHT,
                                  1.0,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(
            SPRITE_TILE_WIDTH / 2.0,
            SPRITE_TILE_HEIGHT / 2.0)
        )
        .insert(KinematicCharacterController::default())
        .insert(Direction::Right); // default direction


}
