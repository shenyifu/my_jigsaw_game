use crate::config::total_pieces::TotalPieces;
use crate::{GameState, PAINT_BOARD_HEIGHT, PAINT_BOARD_WIDTH};
use bevy::app::{App, Update};
use bevy::asset::{AssetServer, Assets, RenderAssetUsages};
use bevy::color::Color;
use bevy::image::Image;
use bevy::input::common_conditions::input_just_pressed;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use image::{DynamicImage, GenericImageView};
use rand::{Rng, thread_rng};
use std::path::Path;

const PAINT_BOARD_COLOR: Color = Color::srgb(255., 255., 255.);
const PAINT_PRE_SELECT_COLOR: Color = Color::srgb(0., 255., 0.);

#[derive(Component)]
#[require(Transform)]
struct CorrectPosition {
    status: CorrectPositionStatus,
    index: usize,
}

#[derive(PartialEq)]
enum CorrectPositionStatus {
    Init,
    Used,
}

#[derive(Component)]
#[require(Sprite, Transform)]
struct Piece {
    correct_position: Transform,
    move_status: MoveStatus,
    used_correct_position: Option<usize>,
}

#[derive(Component, Eq, PartialEq)]
enum MoveStatus {
    Init,
    MoveSprite,
}

#[derive(Resource)]
struct DeltaPosition(Transform);

pub fn play_plugin(app: &mut App) {
    app.insert_resource(DeltaPosition(Transform::default()))
        .add_systems(OnEnter(GameState::Play), setup_game)
        .add_systems(Update, move_sprite.run_if(in_state(GameState::Play)))
        .add_systems(
            Update,
            click_chose
                .run_if(in_state(GameState::Play))
                .run_if(input_just_pressed(MouseButton::Left)),
        );
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    total_pieces: Res<TotalPieces>,
) {
    let split_images = split_image(
        "assets/resources/flower.png",
        total_pieces.get_width_count() as u32,
        total_pieces.get_height_count() as u32,
    )
    .unwrap();

    let mut all_correct_positions = vec![];

    for (index, image) in split_images.into_iter().enumerate() {
        let img = Image::from_dynamic(image, true, RenderAssetUsages::RENDER_WORLD);
        let img_handle = images.add(img);

        let mut sprite = Sprite::from_image(img_handle);
        sprite.custom_size = Some(Vec2::new(
            total_pieces.get_side_length(),
            total_pieces.get_side_length(),
        ));
        let correct_position = get_correct_position(index, &total_pieces);
        all_correct_positions.push(correct_position);
        commands.spawn((
            Piece {
                correct_position,
                move_status: MoveStatus::Init,
                used_correct_position: None,
            },
            random_position(),
            sprite,
        ));

        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(
                total_pieces.get_side_length(),
                total_pieces.get_side_length(),
            ))),
            MeshMaterial2d(materials.add(PAINT_BOARD_COLOR)),
            correct_position,
            CorrectPosition {
                status: CorrectPositionStatus::Init,
                index,
            },
        ));
    }

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
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut correct_positions: Query<
        (
            &mut CorrectPosition,
            &Transform,
            &mut MeshMaterial2d<ColorMaterial>,
        ),
        Without<Piece>,
    >,
    total_pieces: Res<TotalPieces>,
) {
    let (camera, camera_transform) = q_camera.single().unwrap();
    let window = q_window.single().unwrap();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| Some(camera.viewport_to_world(camera_transform, cursor).unwrap()))
        .map(|ray| ray.origin.truncate())
    {
        for (piece, mut current_position) in pieces.iter_mut() {
            if piece.move_status == MoveStatus::MoveSprite {
                current_position.translation.x = world_position.x + delta_position.0.translation.x;
                current_position.translation.y = world_position.y + delta_position.0.translation.y;
                // when close to correct position and not used, hint it

                for (correct_position, correct_position_transform, mut color_material) in
                    correct_positions.iter_mut()
                {
                    if close_correct_position(
                        &current_position,
                        &correct_position_transform,
                        &total_pieces,
                    ) && correct_position.status == CorrectPositionStatus::Init
                    {
                        materials.get_mut(color_material.id()).unwrap().color =
                            PAINT_PRE_SELECT_COLOR;
                    } else {
                        materials.get_mut(color_material.id()).unwrap().color = PAINT_BOARD_COLOR;
                    }
                }
            }
        }
    }
}

fn click_chose(
    mut pieces: Query<(&mut Piece, &mut Transform)>,
    mut result: Query<(&mut Text, &mut TextColor)>,
    mut delta_position: ResMut<DeltaPosition>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    total_pieces: Res<TotalPieces>,
    mut correct_positions: Query<
        (
            &mut CorrectPosition,
            &Transform,
            &mut MeshMaterial2d<ColorMaterial>,
        ),
        Without<Piece>,
    >,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (camera, camera_transform) = q_camera.single().unwrap();
    let window = q_window.single().unwrap();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| Some(camera.viewport_to_world(camera_transform, cursor).unwrap()))
        .map(|ray| ray.origin.truncate())
    {
        let mut move_piece = None;
        for (piece, transform) in pieces.iter_mut() {
            match piece.move_status {
                MoveStatus::Init => (),
                MoveStatus::MoveSprite => move_piece = Some((piece, transform)),
            }
        }

        if move_piece.is_some() {
            let (mut piece, mut current_position) = move_piece.unwrap();

            for (mut correct_position, correct_position_transform, color_material) in
                correct_positions.iter_mut()
            {
                if close_correct_position(
                    &current_position,
                    &correct_position_transform,
                    &total_pieces,
                ) && correct_position.status == CorrectPositionStatus::Init
                {
                    current_position.translation.x = correct_position_transform.translation.x;
                    current_position.translation.y = correct_position_transform.translation.y;
                    correct_position.status = CorrectPositionStatus::Used;
                    piece.used_correct_position = Some(correct_position.index);
                    materials.get_mut(color_material.id()).unwrap().color = PAINT_BOARD_COLOR;
                }

                piece.move_status = MoveStatus::Init;
            }
            if all_sprite_correct(&pieces) {
                // change result to correct
                for (mut text, mut color) in result.iter_mut() {
                    color.0 = Color::srgb(0., 255., 0.);
                    *text = Text::new("Well Done!");
                }
            }
        } else {
            for (mut piece, current_position) in pieces.iter_mut() {
                if cursor_on_sprite(&world_position, &current_position, &total_pieces) {
                    piece.move_status = MoveStatus::MoveSprite;
                    delta_position.0.translation.x =
                        current_position.translation.x - world_position.x;
                    delta_position.0.translation.y =
                        current_position.translation.y - world_position.y;
                    // if already on correct position, change status of correct position
                    if piece.used_correct_position.is_some() {
                        for (mut correct_position, _, _) in correct_positions.iter_mut() {
                            if correct_position.index == piece.used_correct_position.unwrap() {
                                correct_position.status = CorrectPositionStatus::Init;
                                break;
                            }
                        }
                        piece.used_correct_position = None;
                    }
                    break;
                }
            }
        }
    }
}

fn random_position() -> Transform {
    let mut rng = thread_rng();

    Transform::from_xyz(rng.gen_range(-800., 800.), rng.gen_range(-500., 500.), 0.)
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

fn cursor_on_sprite(
    world_position: &Vec2,
    transform: &Transform,
    total_pieces: &TotalPieces,
) -> bool {
    let delta_x = world_position.x - transform.translation.x;
    let delta_y = world_position.y - transform.translation.y;

    delta_x * delta_x + delta_y * delta_y < total_pieces.get_radius()
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
    correct_position: &Transform,
    total_pieces: &TotalPieces,
) -> bool {
    let delta_x = current.translation.x - correct_position.translation.x;
    let delta_y = current.translation.y - correct_position.translation.y;
    if delta_x * delta_x + delta_y * delta_y < total_pieces.get_radius_half() {
        return true;
    }
    false
}

fn get_correct_position(index: usize, total_pieces: &TotalPieces) -> Transform {
    let width_index = index % total_pieces.get_width_count() as usize;
    let height_index = total_pieces.get_height_count() as usize
        - 1
        - (index / total_pieces.get_width_count() as usize);

    Transform::from_xyz(
        total_pieces.get_side_length() / 2. + width_index as f32 * total_pieces.get_side_length()
            - PAINT_BOARD_WIDTH / 2.,
        total_pieces.get_side_length() / 2. + height_index as f32 * total_pieces.get_side_length()
            - PAINT_BOARD_HEIGHT / 2.,
        0.0,
    )
}
