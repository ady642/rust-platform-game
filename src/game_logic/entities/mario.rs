use std::time::Duration;
use bevy::ecs::bundle::DynamicBundle;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::game_logic::entities::champi::Champi;
use crate::rendering::animation::Animation;
use crate::rendering::sprite_manager::{SPRITE_OFFSET_X, SPRITE_PADDING_X, SPRITE_PADDING_Y, SPRITESHEET_COLS, SPRITESHEET_ROWS};

pub const COLLISION_GROUPS_DEFAULT: CollisionGroups = CollisionGroups::new(
    Group::GROUP_2,
    Group::ALL.difference(Group::GROUP_3),
);

#[derive(Component)]
pub enum Direction {
    Right,
    Left,
}

#[derive(Component)]
pub struct Jump(f32);

#[derive(Component)]
pub struct Big(f32);

const PLAYER_VELOCITY_X: f32 = 400.0;
const PLAYER_VELOCITY_Y: f32 = 850.0;
const MAX_JUMP_HEIGHT: f32 = 230.0;
const SPRITE_IDX_WALKING: &[usize] = &[0, 4, 3];
const CYCLE_DELAY: Duration = Duration::from_millis(70);

const SPRITE_MARIO_WIDTH: f32 = 13.0;
const SPRITE_MARIO_HEIGHT: f32 = 23.0;
const SPRITE_BIG_MARIO_HEIGHT: f32 = 30.0;


#[derive(Bundle)]
pub struct Mario {
    sprite_bundle: SpriteSheetBundle,
    body: RigidBody,
    collider: Collider,
    direction: Direction,
}

impl Mario {
    pub fn new(texture_atlas: Handle<TextureAtlas>, x: f32, y: f32) -> Self {
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
            body: RigidBody::KinematicPositionBased,
            collider: Collider::cuboid(
                SPRITE_MARIO_WIDTH / 2.0,
                SPRITE_MARIO_HEIGHT / 2.0,
            ),
            direction: Direction::Right
        }
    }
}


pub fn movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController>,
) {
    for mut player in query.iter_mut() {
        let mut movement = 0.0;

        if input.pressed(KeyCode::Right) {
            movement += time.delta_seconds() * PLAYER_VELOCITY_X;
        }

        if input.pressed(KeyCode::Left) {
            movement += time.delta_seconds() * PLAYER_VELOCITY_X * -1.0;
        }

        match player.translation {
            Some(vec) => player.translation = Some(Vec2::new(movement, vec.y)), // update if it already exists
            None => player.translation = Some(Vec2::new(movement, 0.0)),
        }
    }
}

pub fn jump(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut query_character_controller: Query<&mut KinematicCharacterController>,
    query: Query<
        (Entity, &KinematicCharacterControllerOutput),
        (With<KinematicCharacterController>, Without<Jump>),
    >
) {
    if query.is_empty() {
        return;
    }
    for mut character_controller in query_character_controller.iter_mut() {
        for (player, output) in query.iter() {
            if input.pressed(KeyCode::Up) && output.grounded {
                commands.entity(player).insert(Jump(0.0));
                character_controller.filter_groups = Option::from(CollisionGroups::new(
                    Group::GROUP_2,
                    Group::ALL - Group::GROUP_1 - Group::GROUP_3,
                ));
            }
        }
    }
}

pub fn rise(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut KinematicCharacterController, &mut Jump)>,
) {
    if query.is_empty() {
        return;
    }

    for (entity, mut player, mut jump) in query.iter_mut() {
        let mut movement = time.delta().as_secs_f32() * PLAYER_VELOCITY_Y;

        if movement + jump.0 >= MAX_JUMP_HEIGHT {
            movement = MAX_JUMP_HEIGHT - jump.0;
            commands.entity(entity).remove::<Jump>();
            player.filter_groups = Option::from(COLLISION_GROUPS_DEFAULT);
        }

        jump.0 += movement;

        match player.translation {
            Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
            None => player.translation = Some(Vec2::new(0.0, movement)),
        }
    }
}

pub fn fall(time: Res<Time>, mut query: Query<&mut KinematicCharacterController, Without<Jump>>) {
    if query.is_empty() {
        return;
    }

    for mut player in query.iter_mut() {
        // I am using two-thirds of the Y-velocity since I want the character to fall slower than it rises
        let movement = time.delta().as_secs_f32() * (PLAYER_VELOCITY_Y / 1.5) * -1.0;

        match player.translation {
            Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
            None => player.translation = Some(Vec2::new(0.0, movement)),
        }
    }
}

pub fn apply_movement_animation(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput), Without<Animation>>,
) {
    if query.is_empty() {
        return;
    }

    for (player, output) in query.iter() {
        if output.desired_translation.x != 0.0 && output.grounded {
            commands
                .entity(player)
                .insert(Animation::new(SPRITE_IDX_WALKING, CYCLE_DELAY));
        }
    }
}

pub fn update_direction(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput)>,
) {
    if query.is_empty() {
        return;
    }

    for (player, output) in query.iter() {
        if output.desired_translation.x > 0.0 {
            commands.entity(player).insert(Direction::Right);
        } else if output.desired_translation.x < 0.0 {
            commands.entity(player).insert(Direction::Left);
        }
    }
}

pub fn detect_collision_with_champi(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput)>,
    mut query_champi: Query<(Entity, &mut Transform, &mut Champi)>,
) {
    if query.is_empty() {
        return;
    }

    for (mario_entity, output) in query.iter() {
        if output.collisions.is_empty() {
            return;
        }

        for event in output.collisions.iter() {
            for (champi_entity, transform_champi,mut champi) in query_champi.iter_mut() {
                if champi_entity == event.entity {
                    champi.visible = false;
                    commands.entity(champi_entity).remove::<Collider>();
                    commands.entity(mario_entity).insert(Big(0.0));
                }
            }
        }
    }
}

pub fn add_big_mario(
    mut query: Query<(&mut Handle<TextureAtlas>), With<Big>>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    server: Res<AssetServer>,
) {
    for (mut mario) in query.iter_mut() {
        let image_handle: Handle<Image> = server.load("spritesheets/spritesheet_Mario.png");
        let texture_atlas = TextureAtlas::from_grid(
            image_handle,
            Vec2::new(SPRITE_MARIO_WIDTH, SPRITE_BIG_MARIO_HEIGHT),
            SPRITESHEET_COLS,
            SPRITESHEET_ROWS,
            Option::from(Vec2::new(SPRITE_PADDING_X, SPRITE_PADDING_Y)),
            Option::from(Vec2::new(SPRITE_OFFSET_X, 584.0)),
        );
        let atlas_handle = atlases.add(texture_atlas);

        *mario = atlas_handle;
    }
}
