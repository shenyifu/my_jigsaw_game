use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_sprite)
        .run();
}

#[derive(Component)]
enum Direction {
    Left,
    Right,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_image(asset_server.load("resources/hat.png")),
        Transform::from_xyz(0., 0., 0.),
        Direction::Right,
    ));
}

fn move_sprite(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut direction, mut transform) in sprite_position.iter_mut() {
        match *direction {
            Direction::Left => transform.translation.x -= 150. * time.delta_secs(),
            Direction::Right => transform.translation.x += 150. * time.delta_secs(),
        }
        
        if transform.translation.x > 200. { 
            *direction = Direction::Left;
        }
        else if transform.translation.x < -200. {
            *direction = Direction::Right;
        }
    }
}
