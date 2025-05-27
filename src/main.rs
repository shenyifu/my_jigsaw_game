use bevy::input::common_conditions::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::cmp::PartialEq;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_sprite)
        .add_systems(
            Update,
            click_chose.run_if(input_just_pressed(MouseButton::Left)),
        )
        .run();
}

#[derive(Component, Eq, PartialEq)]
enum MoveStatus {
    Init,
    MoveSprite,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_image(asset_server.load("resources/the-mortar.png")),
        Transform::from_xyz(0., 0., 0.),
        MoveStatus::Init,
    ));
}

fn move_sprite(
    mut sprite_position: Query<(&mut MoveStatus, &mut Transform)>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.single().unwrap();
    let window = q_window.single().unwrap();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| Some(camera.viewport_to_world(camera_transform, cursor).unwrap()))
        .map(|ray| ray.origin.truncate())
    {
        for (move_status, mut transform) in sprite_position.iter_mut() {
            if *move_status == MoveStatus::MoveSprite {
                transform.translation.x = world_position.x;
                transform.translation.y = world_position.y;
            }
        }
    }
}

fn click_chose(mut sprite_position: Query<&mut MoveStatus>) {
    for mut move_status in sprite_position.iter_mut() {
        match *move_status {
            MoveStatus::Init => *move_status = MoveStatus::MoveSprite,
            MoveStatus::MoveSprite => *move_status = MoveStatus::Init,
        }
    }
}
