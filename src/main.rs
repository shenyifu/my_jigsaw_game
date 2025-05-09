use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    let shape = meshes.add(Circle::new(50.0));

    let color = Color::hsl(360. as f32, 0.95, 0.7);

    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(
            0.0,
            0.0,
            0.0,
        ),
    ));
}
