use bevy::{
    prelude::*,
};

mod misc;
mod card;

use misc::mouse;
use card::{factory, primitives, zone};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(mouse::MousePlugin)
        .add_plugins((factory::FactoryPlugin, primitives::CardPlugin, zone::ZonePlugin))
        .run();
}
