use bevy::{
    prelude::*,
};

mod card;
mod input;
mod misc;
mod playground;

use card::{factory, primitives, zone};
use input::drag;
use misc::{camera, mouse};
use playground::card_spawner;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((factory::FactoryPlugin, primitives::CardPlugin, zone::ZonePlugin))
        .add_plugins(drag::DragPlugin)
        .add_plugins((camera::CameraPlugin, mouse::MousePlugin))
        .add_plugins(card_spawner::CSPlugin)
        .run();
}
