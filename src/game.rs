use crate::{Direction, WINDOW_BOTTOM_Y, WINDOW_LEFT_X};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct GamePlugin;

#[derive(Component)]
struct Player;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                setup,
            ));
    }
}

fn setup(
    mut commands: Commands,
    mut atlases: ResMut<Assets<TextureAtlas>>, // sprite
    server: Res<AssetServer>, // kine
) {

}
