use bevy::asset::RenderAssetUsages;
use bevy::input::common_conditions::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use image::{DynamicImage, GenericImageView};
use std::cmp::PartialEq;
use std::path::Path;

const SPIRIT_HEIGHT_COUNT: u8 = 2;
const SPIRIT_WIDTH_COUNT: u8 = 3;
const SPIRIT_SIDE_LENGTH: f32 = PAINT_BOARD_HEIGHT / (SPIRIT_HEIGHT_COUNT as f32);

const SPIRIT_RADIUS: f32 =
    (SPIRIT_SIDE_LENGTH * SPIRIT_SIDE_LENGTH + SPIRIT_SIDE_LENGTH * SPIRIT_SIDE_LENGTH) / 4.;

const SPIRIT_RADIUS_HALF: f32 = SPIRIT_RADIUS / 4.;

// 3 * 2
const PAINT_BOARD_HEIGHT: f32 = 640.;
const PAINT_BOARD_WIDTH: f32 = 960.;

const PAINT_BOARD_COLOR: Color = Color::srgb(255., 255., 255.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_sprite)
        .insert_resource(DeltaPosition(Transform::default()))
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
#[derive(Resource)]
struct DeltaPosition(Transform);
#[derive(Resource)]
struct CorrectPositions(Vec<Transform>);

#[derive(Component)]
struct CorrectPosition;

enum CorrectPositionStatus {
    Init,
    Used,
}

#[derive(Component)]
#[require(Sprite, Transform)]
struct Piece {
    correct_position: Transform,
    move_status: MoveStatus,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn(Camera2d);

    let split_images = split_image(
        "assets/resources/flower.png",
        SPIRIT_WIDTH_COUNT as u32,
        SPIRIT_HEIGHT_COUNT as u32,
    )
    .unwrap();

    let mut all_correct_positions = vec![];

    for (index, image) in split_images.into_iter().enumerate() {
        let img = Image::from_dynamic(image, true, RenderAssetUsages::RENDER_WORLD);
        let img_handle = images.add(img);

        let mut sprite = Sprite::from_image(img_handle);
        sprite.custom_size = Some(Vec2::new(SPIRIT_SIDE_LENGTH, SPIRIT_SIDE_LENGTH));
        let correct_position = get_correct_position(index);
        all_correct_positions.push(correct_position);
        commands.spawn((
            Piece {
                correct_position,
                move_status: MoveStatus::Init,
            },
            correct_position,
            sprite,
        ));

        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(SPIRIT_SIDE_LENGTH, SPIRIT_SIDE_LENGTH))),
            MeshMaterial2d(materials.add(PAINT_BOARD_COLOR)),
            correct_position,
        ));
    }
    commands.insert_resource(CorrectPositions(all_correct_positions));

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
    ));
}

fn move_sprite(
    mut pieces: Query<(&mut Piece, &mut Transform)>,
    delta_position: Res<DeltaPosition>,
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
        for (mut piece, mut current_position) in pieces.iter_mut() {
            if piece.move_status == MoveStatus::MoveSprite {
                current_position.translation.x = world_position.x + delta_position.0.translation.x;
                current_position.translation.y = world_position.y + delta_position.0.translation.y;
            }
        }
    }
}

fn click_chose(
    mut pieces: Query<(&mut Piece, &mut Transform)>,
    mut result: Query<(&mut Text, &mut TextColor)>,
    mut delta_position: ResMut<DeltaPosition>,
    correct_positions: Res<CorrectPositions>,
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
        for (piece, _) in pieces.iter_mut() {
            match piece.move_status {
                MoveStatus::Init => (),
                MoveStatus::MoveSprite => some_in_move = true,
            }
        }

        for (mut piece, mut current_position) in pieces.iter_mut() {
            if some_in_move {
                if piece.move_status == MoveStatus::MoveSprite {
                    let close_position =
                        close_correct_position(&current_position, &correct_positions);

                    if let Some(close_position) = close_position {
                        current_position.translation.x = close_position.translation.x;
                        current_position.translation.y = close_position.translation.y;
                    }

                    piece.move_status = MoveStatus::Init;
                }
            } else if cursor_on_sprite(&world_position, &current_position) {
                piece.move_status = MoveStatus::MoveSprite;
                delta_position.0.translation.x = current_position.translation.x - world_position.x;
                delta_position.0.translation.y = current_position.translation.y - world_position.y;
                break;
            }
        }

        if some_in_move {
            if all_sprite_correct(&pieces) {
                // change result to correct
                for (mut text, mut color) in result.iter_mut() {
                    color.0 = Color::srgb(0., 255., 0.);
                    *text = Text::new("Well Done!");
                }
            }
        }
    }
}

fn all_sprite_correct(pieces: &Query<(&mut Piece, &mut Transform)>) -> bool {
    for (piece, transform) in pieces.iter() {
        if piece.correct_position.translation.x != transform.translation.x
            || piece.correct_position.translation.y != transform.translation.y
        {
            return false;
        }
    }
    true
}

fn cursor_on_sprite(world_position: &Vec2, transform: &Transform) -> bool {
    let delta_x = world_position.x - transform.translation.x;
    let delta_y = world_position.y - transform.translation.y;

    delta_x * delta_x + delta_y * delta_y < SPIRIT_RADIUS
}

pub fn split_image<P: AsRef<Path>>(
    image_path: P,
    width_count: u32,
    height_count: u32,
) -> Result<Vec<DynamicImage>, image::ImageError> {
    let img = image::open(image_path)?;

    let (width, height) = img.dimensions();

    let sub_width = width / width_count;
    let sub_height = height / height_count;

    let mut sub_images = Vec::new();

    for y in 0..height_count {
        for x in 0..width_count {
            let sub_img = img.crop_imm(x * sub_width, y * sub_height, sub_width, sub_height);

            sub_images.push(sub_img);
        }
    }

    Ok(sub_images)
}

fn close_correct_position(
    current: &Transform,
    correct_positions: &CorrectPositions,
) -> Option<Transform> {
    for position in correct_positions.0.iter() {
        let delta_x = current.translation.x - position.translation.x;
        let delta_y = current.translation.y - position.translation.y;
        if delta_x * delta_x + delta_y * delta_y < SPIRIT_RADIUS_HALF {
            return Some(position.clone());
        }
    }
    None
}

fn get_correct_position(index: usize) -> Transform {
    let width_index = index % SPIRIT_WIDTH_COUNT as usize;
    let height_index = SPIRIT_HEIGHT_COUNT as usize - 1 - (index / SPIRIT_WIDTH_COUNT as usize);

    Transform::from_xyz(
        SPIRIT_SIDE_LENGTH as f32 / 2. + width_index as f32 * SPIRIT_SIDE_LENGTH
            - PAINT_BOARD_WIDTH / 2.,
        SPIRIT_SIDE_LENGTH as f32 / 2. + height_index as f32 * SPIRIT_SIDE_LENGTH
            - PAINT_BOARD_HEIGHT / 2.,
        0.0,
    )
}
