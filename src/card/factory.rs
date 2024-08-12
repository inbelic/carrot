use bevy::{
    prelude::*,
    ecs::schedule::common_conditions::on_event,
};

use crate::card::primitives::*;
use crate::card::zone::*;

pub struct FactoryPlugin;

impl Plugin for FactoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreateCard>()
           .add_systems(Startup, factory_init)
           .add_systems(Update, create_card.run_if(on_event::<CreateCard>()));
    }
}

// Moved to a startup system as we can't know if the dims will be inited before
fn factory_init(mut commands: Commands) {
    commands.init_resource::<FactoryState>();

    commands.spawn(ZoneBundle {
        zone: Zone::Hand,
        center: ZoneCenter(Vec2::new(0., -20.)),
        size: ZoneIndex(0),
        spacing: ZoneSpacing(1.),
        dir: ZoneDir::Horiz,
    });
    commands.spawn(ZoneBundle {
        zone: Zone::Play,
        center: ZoneCenter(Vec2::new(0., 0.)),
        size: ZoneIndex(0),
        spacing: ZoneSpacing(1.),
        dir: ZoneDir::Horiz,
    });
    commands.spawn(ZoneBundle {
        zone: Zone::Deck,
        center: ZoneCenter(Vec2::new(50., 0.)),
        size: ZoneIndex(0),
        spacing: ZoneSpacing(1.),
        dir: ZoneDir::Vert,
    });
}

#[derive(Debug, Event)]
pub struct CreateCard {
    pub zone: Zone,
}

#[derive(Resource, Debug)]
struct FactoryState {
    card_id: u16,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl FromWorld for FactoryState {
    fn from_world(world: &mut World) -> Self {
        let dims = world.get_resource::<CardDims>().unwrap().get_dims();

        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh = meshes.add(Cuboid::new(dims.x, dims.y, 0.1));

        let mut materials = world.get_resource_mut::<Assets<StandardMaterial>>().unwrap();
        let color = Color::srgb_u8(124, 144, 255);
        let mat = materials.add(color);

        FactoryState {
            card_id: 0,
            mesh: mesh,
            material: mat,
        }
    }
}

fn create_card(
    mut state: ResMut<FactoryState>,
    dims: Res<CardDims>,
    mut commands: Commands,
    mut ev_cc: EventReader<CreateCard>,
    mut ev_zu: EventWriter<ZoneUpdate>,
    query: Query<(&Zone, &ZoneCenter, &ZoneIndex, &ZoneSpacing, &ZoneDir)>,
) {
    for ev in ev_cc.read() {
        let Some((_zone, center, size, spacing, dir)) =
            query.iter().find(|(zone, _, _, _, _)| **zone == ev.zone)
        else {
            panic!("missing zone")
        };
        let posn = zone_index_to_posn(
            center, &ZoneIndex(size.0 + 1), size, spacing, dir, &dims.get_dims()
        );
        let e = commands.spawn(CardBundle {
            card: Card { id: state.card_id },
            mesh: PbrBundle {
                mesh: state.mesh.clone(),
                material: state.material.clone(),
                transform: Transform::from_translation(posn.extend(0.)),
                     ..default()
             },
             target: Target(posn),
         }).insert(ev.zone).insert(size.clone())
        .id();

        ev_zu.send(ZoneUpdate {
            entity: e,
            zone: ev.zone,
            joining: true,
            index: size.0,
        });
        state.card_id += 1;
     }
}
