use crate::PAINT_BOARD_HEIGHT;
use bevy::prelude::{Component, Resource};
use std::fmt::Display;
use strum::EnumIter;

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy, EnumIter)]
pub enum TotalPieces {
    P6,
    P24,
    P54,
    P96,
}

impl TotalPieces {
    pub fn get_height_count(&self) -> u8 {
        2 * self.get_factor()
    }

    pub fn get_width_count(&self) -> u8 {
        3 * self.get_factor()
    }

    fn get_factor(&self) -> u8 {
        match self {
            TotalPieces::P6 => 1,
            TotalPieces::P24 => 2,
            TotalPieces::P54 => 3,
            TotalPieces::P96 => 4,
        }
    }

    fn get_value(&self) -> u8 {
        match self {
            TotalPieces::P6 => 6,
            TotalPieces::P24 => 24,
            TotalPieces::P54 => 54,
            TotalPieces::P96 => 96,
        }
    }

    pub fn get_side_length(&self) -> f32 {
        PAINT_BOARD_HEIGHT / (self.get_height_count() as f32)
    }

    pub fn get_radius(&self) -> f32 {
        (self.get_side_length() * self.get_side_length()) / 2.
    }

    pub fn get_radius_half(&self) -> f32 {
        self.get_radius() / 4.
    }
}

impl Display for TotalPieces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_value())
    }
}
