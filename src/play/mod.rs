mod board;
mod piece;
mod result;

use crate::config::total_pieces::TotalPieces;
use crate::play::board::{draw_board_color, setup_board};
use crate::play::piece::{move_sprite, setup_piece};
use crate::play::result::setup_result;
use crate::{GameState, PAINT_BOARD_HEIGHT, PAINT_BOARD_WIDTH, despawn_screen};
use bevy::app::{App, Update};
use bevy::math::Vec2;
use bevy::prelude::*;

#[derive(Component)]
struct Moving(Vec2);

type CorrectIndex = usize;

#[derive(States, Default, Clone, Eq, Debug, Hash, PartialEq)]
enum MoveState {
    #[default]
    Init,
    Move,
}

#[derive(Component, Debug)]
#[relationship(relationship_target = Under)]
struct Above(Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = Above)]
struct Under(Entity);

#[derive(Component, Debug)]
#[relationship(relationship_target = PreUnder)]
struct PreAbove(Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = PreAbove)]
struct PreUnder(Entity);

#[derive(Event)]
pub struct Success;

#[derive(Component)]
struct OnPlayScreen;

pub fn play_plugin(app: &mut App) {
    app.init_state::<MoveState>()
        .add_systems(OnEnter(GameState::Play), setup_board)
        .add_systems(OnEnter(GameState::Play), setup_piece)
        .add_systems(OnEnter(GameState::Play), setup_result)
        .add_systems(OnExit(GameState::Play), despawn_screen::<OnPlayScreen>)
        .add_systems(Update, draw_board_color.run_if(in_state(GameState::Play)))
        .add_systems(Update, move_sprite.run_if(in_state(MoveState::Move)));
}

fn get_correct_position(index: usize, total_pieces: &TotalPieces) -> Transform {
    let width_index = index % total_pieces.get_width_count() as usize;
    let height_index = total_pieces.get_height_count() as usize
        - 1
        - (index / total_pieces.get_width_count() as usize);

    Transform::from_xyz(
        total_pieces.get_side_length() / 2. + width_index as f32 * total_pieces.get_side_length()
            - PAINT_BOARD_WIDTH / 2.,
        total_pieces.get_side_length() / 2. + height_index as f32 * total_pieces.get_side_length()
            - PAINT_BOARD_HEIGHT / 2.,
        0.0,
    )
}
