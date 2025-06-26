use crate::config::level::Levels;
use crate::{BUTTON_DEFAULT_BACKGROUND, GameState, TEXT_COLOR, despawn_screen};
use bevy::app::App;
use bevy::prelude::*;

pub fn success_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Success), setup_success);
    app.add_systems(OnExit(GameState::Success),despawn_screen::<OnSuccessScreen>);
}

#[derive(Component)]
struct OnSuccessScreen;

fn setup_success(mut commands: Commands) {
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
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..default()
            },
            OnSuccessScreen,
        ))
        .id();

    let left_image = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..default()
            },
            OnSuccessScreen,
        ))
        .id();

    let right_part = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            OnSuccessScreen,
        ))
        .id();

    commands
        .entity(parent)
        .add_children(&[left_image, right_part]);

    let play_again = commands
        .spawn((
            Button,
            button_node.clone(),
            BackgroundColor(BUTTON_DEFAULT_BACKGROUND),
            children![(Text::new("Play again".to_string()), TextColor(TEXT_COLOR),)],
            OnSuccessScreen,
        ))
        .observe(play_again)
        .id();
    let play_in_order = commands
        .spawn((
            Button,
            button_node.clone(),
            BackgroundColor(BUTTON_DEFAULT_BACKGROUND),
            children![(
                Text::new("Next in order".to_string()),
                TextColor(TEXT_COLOR),
            )],
            OnSuccessScreen,
        ))
        .observe(play_in_order)
        .id();
    let play_random = commands
        .spawn((
            Button,
            button_node.clone(),
            BackgroundColor(BUTTON_DEFAULT_BACKGROUND),
            children![(Text::new("Next random".to_string()), TextColor(TEXT_COLOR),)],
            OnSuccessScreen,
        ))
        .observe(play_random)
        .id();
    let exit = commands
        .spawn((
            Button,
            button_node.clone(),
            BackgroundColor(BUTTON_DEFAULT_BACKGROUND),
            children![(Text::new("exit".to_string()), TextColor(TEXT_COLOR),)],
            OnSuccessScreen,
        ))
        .observe(exit)
        .id();

    commands
        .entity(right_part)
        .add_children(&[play_again, play_in_order, play_random, exit]);
}

fn play_again(_: Trigger<Pointer<Click>>, mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Play);
}

fn play_in_order(
    _: Trigger<Pointer<Click>>,
    mut level: ResMut<Levels>,
    mut state: ResMut<NextState<GameState>>,
) {
    level.next_level();
    state.set(GameState::Play);
}

fn play_random(
    _: Trigger<Pointer<Click>>,
    mut level: ResMut<Levels>,
    mut state: ResMut<NextState<GameState>>,
) {
    level.random_level();
    state.set(GameState::Play);
}

fn exit(_: Trigger<Pointer<Click>>, mut exit: EventWriter<AppExit>) {
    exit.write(AppExit::Success);
}
