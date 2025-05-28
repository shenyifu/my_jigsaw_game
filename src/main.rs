use bevy::input::common_conditions::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::cmp::PartialEq;

const SPIRIT_HEIGHT: f32 = 640.;
const SPIRIT_WIDTH: f32 = 960.;

const SPIRIT_RADIUS: f32 = (SPIRIT_HEIGHT * SPIRIT_HEIGHT + SPIRIT_WIDTH * SPIRIT_WIDTH) / 4.;

// 3 * 2
const PAINT_BOARD_HEIGHT: f32 = 640.;
const PAINT_BOARD_WIDTH: f32 = 960.;

const PAINT_BOARD_COLOR: Color = Color::srgb(255., 255., 255.);

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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let mut mortar = Sprite::from_image(asset_server.load("resources/flower.png"));
    mortar.custom_size = Some(Vec2::new(SPIRIT_WIDTH, SPIRIT_HEIGHT));

    commands.spawn((mortar, Transform::from_xyz(0., 0., 0.), MoveStatus::Init));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(PAINT_BOARD_WIDTH, PAINT_BOARD_HEIGHT))),
        MeshMaterial2d(materials.add(PAINT_BOARD_COLOR)),
        Transform::from_xyz(0., 0.0, 0.0),
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

fn click_chose(
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
        let mut some_in_move = false;
        for (move_status, _) in sprite_position.iter_mut() {
            match *move_status {
                MoveStatus::Init => (),
                MoveStatus::MoveSprite => some_in_move = true,
            }
        }

        for (mut move_status, transform) in sprite_position.iter_mut() {
            if some_in_move {
                *move_status = MoveStatus::Init;
            } else if cursor_on_sprite(&world_position, &transform) {
                *move_status = MoveStatus::MoveSprite;
                break;
            }
        }
    }
}

fn cursor_on_sprite(world_position: &Vec2, transform: &Transform) -> bool {
    let delta_x = world_position.x - transform.translation.x;
    let delta_y = world_position.y - transform.translation.y;

    delta_x * delta_x + delta_y * delta_y < SPIRIT_RADIUS
}
