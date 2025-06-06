use crate::config::total_pieces::TotalPieces;
use crate::{
    GameState, OnConfigScreen, PieceButton, despawn_screen, start_game, total_piece_button_click,
};
use bevy::prelude::*;
use strum::IntoEnumIterator;

pub mod total_pieces;

pub fn config_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Config), setup_config)
        .add_systems(OnExit(GameState::Config), despawn_screen::<OnConfigScreen>);
}

fn setup_config(mut commands: Commands) {
    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let parent = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            OnConfigScreen,
        ))
        .id();

    for total_piece in TotalPieces::iter() {
        let child = commands
            .spawn((
                Button,
                button_node.clone(),
                PieceButton { total_piece },
                children![Text::new(total_piece.to_string())],
                OnConfigScreen,
            ))
            .observe(total_piece_button_click)
            .id();
        commands.entity(parent).add_child(child);
    }

    let start_game = commands
        .spawn((
            Button,
            button_node.clone(),
            children![Text::new("start")],
            OnConfigScreen,
        ))
        .observe(start_game)
        .id();
    commands.entity(parent).add_child(start_game);
}
