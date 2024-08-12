use bevy::{
    prelude::*,
    input::common_conditions::*,
    math::bounding::Aabb2d,
    render::primitives::Aabb,
};

use crate::card::zone::*;
use crate::card::primitives::{Card, CardDims, Target};
use crate::misc::mouse;

pub struct DragPlugin;

impl Plugin for DragPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Selected>()
           .add_systems(Update, (
                   select_card.run_if(input_just_pressed(MouseButton::Left))
                              .run_if(deselected),
                   deselect_card.run_if(input_just_released(MouseButton::Left))
                                .run_if(selected),
            ))
           .add_systems(Update, update_card_target.run_if(selected));
    }
}

#[derive(Resource, Debug, Default)]
struct Selected(Option<(Entity, Zone, ZoneIndex)>);

fn selected(selected: Res<Selected>) -> bool { selected.0.is_some() }
fn deselected(selected: Res<Selected>) -> bool { selected.0.is_none() }

fn select_card(
    mouse: Res<mouse::Mouse>,
    mut ev_zu: EventWriter<ZoneUpdate>,
    mut selected: ResMut<Selected>,
    query: Query<(Entity, &Transform, &Aabb, &Zone, &ZoneIndex), With<Card>>
) {
    for (e, transform, aabb, zone, z_idx) in query.iter() {
        let card_bounds = Aabb2d::new(
            transform.translation.truncate(), aabb.half_extents.truncate()
        );
        let mouse_posn = mouse.get_posn();
        if mouse_posn == card_bounds.closest_point(mouse_posn) {
            let mut replace = true;
            if let Some((_, _, cur_idx)) = selected.0 {
                replace = z_idx.0 < cur_idx.0;
            }

            if replace {
                selected.0 = Some((e, zone.clone(), *z_idx));
            }
        }
    }
    if let Some((e, zone, z_idx)) = selected.0 {
        ev_zu.send(ZoneUpdate {
            entity: e,
            zone: zone.clone(),
            joining: false,
            index: z_idx.0,
        });
    }
}

fn deselect_card(
    dims: Res<CardDims>,
    mouse: Res<mouse::Mouse>,
    zone_query: Query<(&Zone, &ZoneCenter, &ZoneIndex, &ZoneSpacing, &ZoneDir)>,
    mut ev_zu: EventWriter<ZoneUpdate>,
    mut selected: ResMut<Selected>,
) {
    let (e, old_zone, z_idx) = selected.0.unwrap();
    let mut new_zone = old_zone;
    let mut new_index = z_idx.0;
    let mouse_posn = mouse.get_posn();
    for (zone, center, size, spacing, dir) in zone_query.iter() {
        if within_zone(&mouse_posn, center, size, spacing, dir, &dims.get_dims()) {
            new_zone = zone.clone();
            new_index = size.0;
        }
    }
    ev_zu.send(ZoneUpdate {
        entity: e,
        zone: new_zone,
        joining: true,
        index: new_index,
    });
    selected.0 = None;
}

fn update_card_target(
    mouse: Res<mouse::Mouse>,
    selected: Res<Selected>,
    mut query: Query<&mut Target>,
) {
    let (entity, _zone, _z_idx) = selected.0.unwrap();
    if let Ok(mut target) = query.get_mut(entity) {
        target.0 = mouse.get_posn();
    }
}
