use crate::config::level::Levels;
use crate::config::total_pieces::TotalPieces;
use crate::{
    BUTTON_DEFAULT_BACKGROUND, BUTTON_SELECTED_BACKGROUND, GameState, TEXT_COLOR, despawn_screen,
};
use bevy::prelude::*;
use strum::IntoEnumIterator;

pub(crate) mod level;
pub mod total_pieces;

pub fn config_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Config), setup_config)
        .add_systems(OnExit(GameState::Config), despawn_screen::<OnConfigScreen>)
        .add_systems(
            Update,
            render_piece_color.run_if(resource_changed::<TotalPieces>),
        )
        .insert_resource(TotalPieces::P24);
}

#[derive(Component)]
struct PieceButton {
    total_piece: TotalPieces,
}

fn start_game(_: Trigger<Pointer<Click>>, mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Play);
}

#[derive(Component)]
struct OnConfigScreen;

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

    let piece_parent = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..default()
            },
            OnConfigScreen,
        ))
        .id();

    commands.entity(parent).add_child(piece_parent);

    for total_piece in TotalPieces::iter() {
        let child = commands
            .spawn((
                Button,
                button_node.clone(),
                PieceButton { total_piece },
                BackgroundColor(BUTTON_DEFAULT_BACKGROUND),
                children![(Text::new(total_piece.to_string()), TextColor(TEXT_COLOR),)],
                OnConfigScreen,
            ))
            .observe(total_piece_button_click)
            .id();
        commands.entity(piece_parent).add_child(child);
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
    commands.insert_resource(Levels::default())
}

fn total_piece_button_click(
    click: Trigger<Pointer<Click>>,
    query: Query<&PieceButton>,
    mut total_pieces: ResMut<TotalPieces>,
) {
    let piece_button = query.get(click.target);
    if let Ok(piece_button) = piece_button {
        *total_pieces = piece_button.total_piece;
    }
}

fn render_piece_color(
    total_pieces: Res<TotalPieces>,
    query: Query<(&PieceButton, &mut BackgroundColor)>,
) {
    for (piece_button, mut background) in query {
        if piece_button.total_piece == *total_pieces {
            *background = BackgroundColor(BUTTON_SELECTED_BACKGROUND)
        } else {
            *background = BackgroundColor(BUTTON_DEFAULT_BACKGROUND)
        }
    }
}
