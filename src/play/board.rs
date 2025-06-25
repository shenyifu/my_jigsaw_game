use crate::config::total_pieces::TotalPieces;
use crate::play::{CorrectIndex, OnPlayScreen, PreUnder, get_correct_position};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::prelude::*;

const PAINT_BOARD_COLOR: Color = Color::srgb(255., 255., 255.);
const PAINT_PRE_SELECT_COLOR: Color = Color::srgb(0., 255., 0.);

#[derive(Component)]
pub struct Board {
    pub(crate) index: CorrectIndex,
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
            Board { index },
            OnPlayScreen,
        ));
    }
}

pub fn draw_board_color(
    mut materials: ResMut<Assets<ColorMaterial>>,
    pre_under: Query<&PreUnder>,
    mut correct_positions: Query<(&MeshMaterial2d<ColorMaterial>, Entity), With<Board>>,
) {
    for (mesh_material, entity) in correct_positions.iter_mut() {
        materials.get_mut(mesh_material.id()).unwrap().color = if pre_under.get(entity).is_ok() {
            PAINT_PRE_SELECT_COLOR
        } else {
            PAINT_BOARD_COLOR
        }
    }
}
