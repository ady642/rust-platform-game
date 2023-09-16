use std::time::Duration;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::game_logic::entities::champi::Champi;
use crate::rendering::animation::Animation;

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

const PLAYER_VELOCITY_X: f32 = 400.0;
const PLAYER_VELOCITY_Y: f32 = 850.0;
const MAX_JUMP_HEIGHT: f32 = 230.0;
const SPRITE_IDX_WALKING: &[usize] = &[0, 4, 3];
const CYCLE_DELAY: Duration = Duration::from_millis(70);



pub fn movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController>,
) {
    let mut player = query.single_mut();

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
    let mut character_controller = query_character_controller.single_mut();


    let (player, output) = query.single();

    if input.pressed(KeyCode::Up) && output.grounded {
        commands.entity(player).insert(Jump(0.0));
        character_controller.filter_groups = Option::from(CollisionGroups::new(
            Group::GROUP_2,
            Group::ALL - Group::GROUP_1 - Group::GROUP_3,
        ));
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

    let (entity, mut player, mut jump) = query.single_mut();

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

pub fn fall(time: Res<Time>, mut query: Query<&mut KinematicCharacterController, Without<Jump>>) {
    if query.is_empty() {
        return;
    }

    let mut player = query.single_mut();

    // I am using two-thirds of the Y-velocity since I want the character to fall slower than it rises
    let movement = time.delta().as_secs_f32() * (PLAYER_VELOCITY_Y / 1.5) * -1.0;

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
        None => player.translation = Some(Vec2::new(0.0, movement)),
    }
}

pub fn apply_movement_animation(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput), Without<Animation>>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = query.single();
    if output.desired_translation.x != 0.0 && output.grounded {
        commands
            .entity(player)
            .insert(Animation::new(SPRITE_IDX_WALKING, CYCLE_DELAY));
    }
}

pub fn update_direction(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput)>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = query.single();

    if output.desired_translation.x > 0.0 {
        commands.entity(player).insert(Direction::Right);
    } else if output.desired_translation.x < 0.0 {
        commands.entity(player).insert(Direction::Left);
    }
}

pub fn detect_collision_with_champi(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput)>,
    mut query_champi: Query<(Entity, &mut Champi)>,
) {
    if query.is_empty() {
        return;
    }

    let (_, output) = query.single();

    if output.collisions.is_empty() {
        return;
    }

    for event in output.collisions.iter() {
        for (champi_entity, mut champi) in query_champi.iter_mut() {
            if champi_entity == event.entity {
                champi.visible = false;
                commands.entity(champi_entity).remove::<Collider>();
                // then change mario to be bigger


            }
        }
    }
}
