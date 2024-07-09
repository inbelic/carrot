use bevy::{
    prelude::*,
};

mod misc;

use misc::mouse;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, mouse::MousePlugin))
        .run();
}
