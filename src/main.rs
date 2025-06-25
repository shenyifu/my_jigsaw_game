mod config;
mod play;
mod success;

use bevy::prelude::*;
use std::cmp::PartialEq;

const BUTTON_DEFAULT_BACKGROUND: Color = Color::srgb(255., 255., 255.);
const BUTTON_SELECTED_BACKGROUND: Color = Color::srgb(0., 255., 0.);

const TEXT_COLOR: Color = Color::srgb(0., 0., 0.);

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
        .add_plugins((
            config::config_plugin,
            play::play_plugin,
            success::success_plugin,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
