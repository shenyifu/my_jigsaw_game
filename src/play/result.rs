use crate::GameState;
use crate::play::{OnPlayScreen, Success};
use bevy::asset::AssetServer;
use bevy::color::Color;
use bevy::prelude::*;

#[derive(Component)]
struct Result;
pub fn setup_result(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text::new("come on!"),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 33.0,
            ..default()
        },
        TextColor(Color::srgb(255., 0., 0.)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
        Result,
        OnPlayScreen,
    ));
    commands.add_observer(update_status);
}

fn update_status(
    _: Trigger<Success>,
    mut result: Query<(&mut Text, &mut TextColor), With<Result>>,
    mut state: ResMut<NextState<GameState>>,
) {
    for (mut text, mut color) in result.iter_mut() {
        color.0 = Color::srgb(0., 255., 0.);
        *text = Text::new("Well Done!");
    }

    state.set(GameState::Success);
}
