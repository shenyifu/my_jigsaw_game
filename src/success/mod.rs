use crate::{BUTTON_DEFAULT_BACKGROUND, GameState, TEXT_COLOR};
use bevy::app::App;
use bevy::prelude::*;

pub fn success_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Success), setup_success);
}

#[derive(Component)]
struct OnSuccessScreen;

fn setup_success(mut commands: Commands) {
    // left is current success picture

    //right is buttons

    // buttons:
    // try again
    // next in order
    // random next
    // exit

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

    commands.entity(parent).add_child(left_image);

    let right_buttons = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            OnSuccessScreen,
            children![
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(BUTTON_DEFAULT_BACKGROUND),
                    children![(Text::new("Play again".to_string()), TextColor(TEXT_COLOR),)],
                    OnSuccessScreen,
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(BUTTON_DEFAULT_BACKGROUND),
                    children![(
                        Text::new("Next in order".to_string()),
                        TextColor(TEXT_COLOR),
                    )],
                    OnSuccessScreen,
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(BUTTON_DEFAULT_BACKGROUND),
                    children![(Text::new("Next random".to_string()), TextColor(TEXT_COLOR),)],
                    OnSuccessScreen,
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(BUTTON_DEFAULT_BACKGROUND),
                    children![(Text::new("Exit".to_string()), TextColor(TEXT_COLOR),)],
                    OnSuccessScreen,
                )
            ],
        ))
        .id();
    commands.entity(parent).add_child(right_buttons);
}
