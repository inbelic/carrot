use bevy::{
    prelude::*,
    ecs::schedule::common_conditions::on_event,
    math::bounding::Aabb2d,
};
use std::collections::HashMap;
use std::collections::HashSet;

use crate::card::primitives::{Card, CardDims, Target};

pub struct ZonePlugin;

impl Plugin for ZonePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ZoneUpdate>()
           .add_systems(Update, (
                   update_zone_info,
                   update_card_indices,
                   rebase_updated_zones,
            ).chain().run_if(on_event::<ZoneUpdate>()));
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

#[derive(Component, Clone, Copy, Debug)]
pub struct ZoneIndex(pub u8);

#[derive(Debug, Event)]
pub struct ZoneUpdate {
    pub entity: Entity,
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
    mut query: Query<(Entity, &mut Zone, &mut ZoneIndex), With<Card>>,
) {
    for update in ev_zone_update.read() {
        for (e, mut zone, mut z_index) in query.iter_mut() {
            if update.entity == e {
                *zone = update.zone;
                z_index.0 = update.index;
            } else if update.joining && *zone == update.zone && update.index <= z_index.0 {
                z_index.0 += 1;
            } else if !update.joining && *zone == update.zone && update.index < z_index.0 {
                z_index.0 -= 1;
            }
        }
    }
}

const CARD_SPACING: f32 = 5.;

pub fn zone_index_to_posn(
    center: &ZoneCenter,
    size: &ZoneIndex,
    index: &ZoneIndex,
    card_dims: &Vec2,
) -> Vec2 {
    let width = card_dims.x + CARD_SPACING;
    let steps = size.0 as f32 / 2. - 0.5;
    let x = center.0.x + (index.0 as f32 - steps) * width;
    Vec2::new(x, center.0.y)
}

fn rebase_updated_zones(
    dims: Res<CardDims>,
    mut ev_zone_update: EventReader<ZoneUpdate>,
    zone_query: Query<(&Zone, &ZoneCenter, &ZoneIndex)>,
    mut card_query: Query<(&mut Target, &Zone, &ZoneIndex), With<Card>>,
) {
    // zones denotes the list of zones that have been updated
    let zones = ev_zone_update.read().map(|x| x.zone).collect::<HashSet<_>>();
    for (mut card_target, card_zone, card_posn) in card_query.iter_mut() {
        if zones.contains(card_zone) {
            for (zone, center, size) in zone_query.iter() {
                if zone == card_zone {
                    card_target.0 = zone_index_to_posn(
                        center, size, card_posn, &dims.get_dims()
                    );
                }
            }
        }
    }
}

pub fn within_zone(
    posn: &Vec2,
    center: &ZoneCenter,
    size: &ZoneIndex,
    card_dims: &Vec2,
) -> bool {
    let width = card_dims.x + CARD_SPACING;
    let steps = size.0 as f32 / 2. - 0.5;
    let zone_bounds = Aabb2d::new(
        center.0, Vec2::new(width * steps.max(0.5), card_dims.y / 2.)
    );
    *posn == zone_bounds.closest_point(*posn)
}
