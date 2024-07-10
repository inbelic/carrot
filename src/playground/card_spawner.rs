use bevy::{
    prelude::*,
    input::common_conditions::*,
};

use crate::card::{factory, zone};

pub struct CSPlugin;

impl Plugin for CSPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,
            spawn_card.run_if(input_just_pressed(KeyCode::Space))
        );
    }
}

fn spawn_card(
    mut ev_cc: EventWriter<factory::CreateCard>,
) {
    ev_cc.send(factory::CreateCard {
        zone: zone::Zone::Hand,
    });
}
