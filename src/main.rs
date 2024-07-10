use bevy::{
    prelude::*,
};

mod card;
mod misc;
mod playground;

use misc::mouse;
use card::{factory, primitives, zone};
use playground::card_spawner;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(mouse::MousePlugin)
        .add_plugins((factory::FactoryPlugin, primitives::CardPlugin, zone::ZonePlugin))
        .add_plugins(card_spawner::CSPlugin)
        .run();
}
