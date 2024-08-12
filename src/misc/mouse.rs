use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

use crate::camera::MainCamera;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Mouse>()
            .add_systems(Update, (
                    update_mouse,
                    draw_mouse
            ).chain());
    }
}

#[derive(Resource, Debug, Default)]
pub struct Mouse {
    posn: Vec2,
}

impl Mouse {
    pub fn get_posn(&self) -> Vec2 {
        self.posn
    }
}

fn update_mouse(
    mut mouse: ResMut<Mouse>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // there is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .and_then(|ray| {
            if let Some(distance)
                = ray.intersect_plane(Vec3::splat(0.), InfinitePlane3d::new(Vec3::Z)) {
                    return Some(ray.get_point(distance).truncate())
            } else {
                return None
            }
        })
    {
        mouse.posn = world_position;
    }
}

fn draw_mouse(
    mouse: Res<Mouse>,
    mut gizmos: Gizmos,
) {
    gizmos.circle(mouse.get_posn().extend(0.) + Vec3::Z * 0.01, Dir3::Z, 0.2, Color::WHITE);
}
