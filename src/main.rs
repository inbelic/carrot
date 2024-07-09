use bevy::{
    prelude::*,
};

mod misc;
mod card;

use misc::mouse;
use card::{primitives, zone};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((mouse::MousePlugin, primitives::CardPlugin, zone::ZonePlugin))
        .run();
}
