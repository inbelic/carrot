use bevy::{
    prelude::*,
    ecs::schedule::common_conditions::on_event,
};
use std::collections::HashMap;

use crate::card::primitives::Card;

pub struct ZonePlugin;

impl Plugin for ZonePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ZoneUpdate>()
           .add_systems(Update,
               update_zone_info.run_if(on_event::<ZoneUpdate>()))
           .add_systems(Update,
               update_card_indices.run_if(on_event::<ZoneUpdate>()));
    }
}

#[derive(Bundle)]
pub struct ZoneBundle {
    pub zone: Zone,
    pub center: ZoneCenter,
    pub size: ZoneIndex,
}

#[derive(Clone, Component, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Zone {
    Deck,
    Hand,
    Play,
}

#[derive(Component, Debug)]
pub struct ZoneCenter(pub Vec2);

#[derive(Component, Clone, Debug)]
pub struct ZoneIndex(pub u8);

#[derive(Debug, Event)]
pub struct ZoneUpdate {
    pub zone: Zone,
    pub joining: bool,
    pub index: u8,
}

fn update_zone_info(
    mut ev_zone_update: EventReader<ZoneUpdate>,
    mut query: Query<(&Zone, &mut ZoneIndex), With<ZoneCenter>>,
) {
    let mut zone_map = HashMap::<Zone, i8>::new();
    ev_zone_update.read().for_each(|update| {
        let inc = (update.joining as i8) * 2 - 1;
        zone_map.entry(update.zone).and_modify(|x| *x += inc).or_insert(inc);
    });
    for (zone, mut size) in query.iter_mut() {
        let new_size = size.0 as i8 + zone_map.get(zone).unwrap_or(&0);
        *size = ZoneIndex(new_size as u8);
    }
}

fn update_card_indices(
    mut ev_zone_update: EventReader<ZoneUpdate>,
    mut query: Query<(&Zone, &mut ZoneIndex), With<Card>>,
) {
    for update in ev_zone_update.read() {
        for (zone, mut z_index) in query.iter_mut() {
            if update.joining && *zone == update.zone && update.index < z_index.0 {
                z_index.0 -= 1;
            } else if !update.joining && *zone == update.zone && z_index.0 < update.index {
                z_index.0 += 1;
            }
        }
    }
}

pub fn zone_index_to_posn(
    center: &ZoneCenter,
    size: &ZoneIndex,
    index: &ZoneIndex,
    card_dims: &Vec2,
) -> Vec2 {
    let x = center.0.x - (size.0 as f32 / 2. - 0.5) * card_dims.x;
    Vec2::new(x + card_dims.x * index.0 as f32, center.0.y)
}
