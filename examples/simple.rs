use bevy::{core_pipeline::prepass::NormalPrepass, prelude::*};
use bevy_show_normals::{NormalCamera, ShowNormalPlugin};

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins((DefaultPlugins, ShowNormalPlugin))
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // set up the camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(2., 2., 3.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        NormalPrepass,
        NormalCamera,
    ));

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::rgb_u8(124, 144, 255)),
        ..default()
    });
}
