pub struct CameraPlugin;
use bevy::prelude::*;
use bevy_rapier2d::control::KinematicCharacterControllerOutput;
use bevy_rapier2d::prelude::KinematicCharacterController;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sync_player_camera);
    }
}

pub fn sync_player_camera(
    mut player: Query<&mut KinematicCharacterControllerOutput>,
    mut camera: Query<(&mut Camera, &mut Transform)>,
) {
    let Ok(player) = player.get_single() else { return };
    let Ok((mut camera, mut camera_transform)) = camera.get_single_mut() else { return };

    println!("Player x position: {:?}", player.effective_translation.x);
    println!("Camera x position: {:?}", camera_transform.translation.x );
    camera_transform.translation.x += player.effective_translation.x;
}
