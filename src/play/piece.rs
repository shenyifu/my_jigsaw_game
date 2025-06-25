use crate::config::level::Levels;
use crate::config::total_pieces::TotalPieces;
use crate::play::board::Board;
use crate::play::{
    Above, CorrectIndex, MoveState, Moving, OnPlayScreen, PreAbove, Success, Under,
    get_correct_position,
};
use bevy::asset::{Assets, RenderAssetUsages};
use bevy::image::Image;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use image::{DynamicImage, GenericImageView};
use rand::{Rng, thread_rng};
use std::path::Path;

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Piece {
    pub correct_index: CorrectIndex,
}

pub fn setup_piece(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    total_pieces: Res<TotalPieces>,
    level: Res<Levels>,
) {
    let split_images = split_image(
        level.current_level().get_path(),
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
        commands
            .spawn((
                Piece {
                    correct_index: index,
                },
                random_position(),
                sprite,
                Pickable::default(),
                OnPlayScreen,
            ))
            .observe(chose_one_piece)
            .observe(piece_picked)
            .observe(piece_unpicked);
    }
    commands.add_observer(check_piece_all_correct);
}

#[derive(Event)]
struct Pick;

#[derive(Event)]
struct Unpick;

#[derive(Event)]
struct PieceMatch;

fn chose_one_piece(
    click: Trigger<Pointer<Click>>,
    pieces: Query<(&Piece, &Transform)>,
    mut commands: Commands,
    state: Res<State<MoveState>>,
    mut next_state: ResMut<NextState<MoveState>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.single().unwrap();

    match state.get() {
        MoveState::Init => {
            let piece = pieces.get(click.target);
            if let Ok((_, transform)) = piece {
                let world_position = camera
                    .viewport_to_world(camera_transform, click.pointer_location.position)
                    .unwrap()
                    .origin
                    .truncate();

                commands.entity(click.target).insert(Moving(Vec2::new(
                    transform.translation.x - world_position.x,
                    transform.translation.y - world_position.y,
                )));
                commands.trigger_targets(Pick, click.target)
            }
            next_state.set(MoveState::Move);
        }
        MoveState::Move => {
            commands.entity(click.target).remove::<Moving>();
            commands.trigger_targets(Unpick, click.target);

            next_state.set(MoveState::Init);
        }
    }
}

fn piece_picked(pick: Trigger<Pick>, above: Query<&Above>, mut commands: Commands) {
    if let Ok(above) = above.get(pick.target()) {
        commands.entity(pick.target()).remove::<Above>();
        commands.entity(pick.target()).insert(PreAbove(above.0));
    }
}

fn piece_unpicked(
    unpick: Trigger<Unpick>,
    pre_above: Query<&PreAbove>,
    mut commands: Commands,
    mut pieces: Query<(&Piece, &mut Transform)>,
    boards: Query<(&Board, &Transform), Without<Piece>>,
) {
    if let Ok(pre_above) = pre_above.get(unpick.target()) {
        let (_, mut piece_transform) = pieces.get_mut(unpick.target()).unwrap();
        let (_, box_transform) = boards.get(pre_above.0).unwrap();
        piece_transform.translation.x = box_transform.translation.x;
        piece_transform.translation.y = box_transform.translation.y;

        commands.entity(unpick.target()).remove::<PreAbove>();
        commands.entity(unpick.target()).insert(Above(pre_above.0));
        commands.trigger(PieceMatch);
    }
}

fn check_piece_all_correct(
    _: Trigger<PieceMatch>,
    above: Query<(&Above, Entity)>,
    pieces: Query<&Piece>,
    boards: Query<&Board>,
    total_pieces: Res<TotalPieces>,
    mut commands: Commands,
) {
    if above.iter().len() != total_pieces.get_value() as usize {
        return;
    }

    for (above, entity) in above {
        let piece = pieces.get(entity).unwrap();
        let board = boards.get(above.0).unwrap();

        if piece.correct_index != board.index {
            return;
        }
    }
    commands.trigger(Success);
}

pub fn move_sprite(
    mut pieces: Query<(&mut Transform, &Moving, Entity), Without<Board>>,
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut correct_positions: Query<(&Transform, Entity), (With<Board>, Without<Under>)>,
    pre_above: Query<&PreAbove>,
    total_pieces: Res<TotalPieces>,
) {
    let (camera, camera_transform) = q_camera.single().unwrap();
    let window = q_window.single().unwrap();

    let world_position = window
        .cursor_position()
        .map(|cursor| camera.viewport_to_world(camera_transform, cursor).unwrap())
        .map(|ray| ray.origin.truncate());

    if world_position.is_none() {
        return;
    }

    let world_position = world_position.unwrap();

    for (mut current_position, moving, piece_entity) in pieces.iter_mut() {
        current_position.translation.x = world_position.x + moving.0.x;
        current_position.translation.y = world_position.y + moving.0.y;
        for (board_transform, board_entity) in correct_positions.iter_mut() {
            if close_correct_position(&current_position, board_transform, &total_pieces) {
                if pre_above.get(piece_entity).is_ok()
                    && pre_above.get(piece_entity).unwrap().0 != board_entity
                {
                    commands.entity(piece_entity).remove::<PreAbove>();
                    commands.entity(piece_entity).insert(PreAbove(board_entity));
                } else {
                    commands.entity(piece_entity).insert(PreAbove(board_entity));
                }
            } else {
                commands.entity(piece_entity).remove::<PreAbove>();
            }
        }
    }
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

fn split_image<P: AsRef<Path>>(
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

fn random_position() -> Transform {
    let mut rng = thread_rng();
    Transform::from_xyz(rng.gen_range(-800., 800.), rng.gen_range(-500., 500.), 0.)
}
