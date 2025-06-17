mod config;
mod play;

use bevy::prelude::*;
use config::total_pieces::TotalPieces;
use std::cmp::PartialEq;

// 3 * 2
const PAINT_BOARD_HEIGHT: f32 = 640.;
const PAINT_BOARD_WIDTH: f32 = 960.;
// todo get image from https://picsum.photos/id/1/1920/1280.jpg

#[derive(States, Default, Clone, Eq, Debug, Hash, PartialEq)]
enum GameState {
    #[default]
    Config,
    Play,
    Success,
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .init_state::<GameState>()
        .add_plugins((config::config_plugin, play::play_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
