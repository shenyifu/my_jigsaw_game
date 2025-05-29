use bevy::asset::RenderAssetUsages;
use bevy::input::common_conditions::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use std::cmp::PartialEq;
use std::path::Path;

const SPIRIT_HEIGHT_COUNT: u32 = 2;
const SPIRIT_WIDTH_COUNT: u32 = 3;
const SPIRIT_SIDE_LENGTH: f32 = PAINT_BOARD_HEIGHT / (SPIRIT_HEIGHT_COUNT as f32);

const SPIRIT_RADIUS: f32 =
    (SPIRIT_SIDE_LENGTH * SPIRIT_SIDE_LENGTH + SPIRIT_SIDE_LENGTH * SPIRIT_SIDE_LENGTH) / 4.;

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

#[derive(Component)]
struct CorrectPosition(Transform);

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
        SPIRIT_WIDTH_COUNT,
        SPIRIT_HEIGHT_COUNT,
    )
    .unwrap();
    
    for image in split_images {
        let img = Image::from_dynamic(image, true, RenderAssetUsages::RENDER_WORLD);
        let img_handle = images.add(img);

        let mut sprite = Sprite::from_image(img_handle);
        sprite.custom_size = Some(Vec2::new(SPIRIT_SIDE_LENGTH, SPIRIT_SIDE_LENGTH));
        commands.spawn((
            sprite,
            Transform::from_xyz(0., 0., 0.),
            MoveStatus::Init,
            CorrectPosition(Transform::from_xyz(0., 0., 0.)),
        ));
    }
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(PAINT_BOARD_WIDTH, PAINT_BOARD_HEIGHT))),
        MeshMaterial2d(materials.add(PAINT_BOARD_COLOR)),
        Transform::from_xyz(0., 0.0, 0.0),
    ));

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
    mut sprite_position: Query<(&mut MoveStatus, &mut Transform, &CorrectPosition)>,
    mut result: Query<(&mut Text, &mut TextColor)>,
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
        for (move_status, _, _) in sprite_position.iter_mut() {
            match *move_status {
                MoveStatus::Init => (),
                MoveStatus::MoveSprite => some_in_move = true,
            }
        }
        let mut check_all_correct = false;

        for (mut move_status, transform, _) in sprite_position.iter_mut() {
            if some_in_move {
                *move_status = MoveStatus::Init;
                // todo check if on paint board and near one correct position
                // if so please it to one correct position
                // check if all already correct
                // only check one now
                check_all_correct = true;
            } else if cursor_on_sprite(&world_position, &transform) {
                *move_status = MoveStatus::MoveSprite;
                break;
            }
        }

        if check_all_correct {
            if all_sprite_correct(&sprite_position) {
                // change result to correct
                for (mut text, mut color) in result.iter_mut() {
                    color.0 = Color::srgb(0., 255., 0.);
                    *text = Text::new("Well Done!");
                }
            }
        }
    }
}

fn all_sprite_correct(
    sprite_position: &Query<(&mut MoveStatus, &mut Transform, &CorrectPosition)>,
) -> bool {
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
