use bevy::{
    prelude::*,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Component)]
pub struct MainCamera;

pub const CAMERA_HEIGHT: f32 = 100.;

fn setup(mut commands: Commands) {
    // camera
    commands.spawn((Camera3dBundle {
        transform:
            Transform::from_xyz(0.0, 0.0, 100.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }, MainCamera));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 15.0),
        ..default()
    });
}
