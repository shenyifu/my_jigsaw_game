use crate::config::total_pieces::TotalPieces;
use crate::play::{Piece, get_correct_position};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::prelude::{
    ColorMaterial, Commands, Component, Mesh, Mesh2d, MeshMaterial2d, Query, Rectangle, Res,
    ResMut, Without,
};

const PAINT_BOARD_COLOR: Color = Color::srgb(255., 255., 255.);
const PAINT_PRE_SELECT_COLOR: Color = Color::srgb(0., 255., 0.);

#[derive(PartialEq)]
pub enum BoardStatus {
    Init,
    PreSelect,
    Used,
}

#[derive(Component)]
pub struct Board {
    pub(crate) status: BoardStatus,
    pub(crate) index: usize,
}

pub fn setup_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    total_pieces: Res<TotalPieces>,
) {
    for index in 0..total_pieces.get_value() {
        let index = index as usize;
        let correct_position = get_correct_position(index, &total_pieces);
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(
                total_pieces.get_side_length(),
                total_pieces.get_side_length(),
            ))),
            MeshMaterial2d(materials.add(PAINT_BOARD_COLOR)),
            correct_position,
            Board {
                status: BoardStatus::Init,
                index,
            },
        ));
    }
}

pub fn draw_board_color(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut correct_positions: Query<(&Board, &MeshMaterial2d<ColorMaterial>), Without<Piece>>,
) {
    for (board, mesh_material) in correct_positions.iter_mut() {
        materials.get_mut(mesh_material.id()).unwrap().color = match board.status {
            BoardStatus::Used => PAINT_BOARD_COLOR,
            BoardStatus::Init => PAINT_BOARD_COLOR,
            BoardStatus::PreSelect => PAINT_PRE_SELECT_COLOR,
        }
    }
}
