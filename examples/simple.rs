use bevy::{
    core_pipeline::prepass::{DepthPrepass, NormalPrepass},
    prelude::*,
};
use bevy_mod_edge_detection::{NormalCamera, ShowNormalPlugin};

fn main() {
    App::new()
        // MSAA currently doesn't work correctly with the plugin
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
            transform: Transform::from_xyz(3., 3., 5.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        // The edge detection effect requires the depth and normal prepass
        DepthPrepass,
        NormalPrepass,
        NormalCamera,
    ));

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::rgb_u8(124, 144, 255)),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
