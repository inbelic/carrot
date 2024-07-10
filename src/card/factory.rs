use bevy::{
    prelude::*,
    ecs::schedule::common_conditions::on_event,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
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
        center: ZoneCenter(Vec2::new(0., -100.)),
        size: ZoneIndex(0),
    });
    commands.spawn(ZoneBundle {
        zone: Zone::Deck,
        center: ZoneCenter(Vec2::new(500., 0.)),
        size: ZoneIndex(0),
    });
    commands.spawn(ZoneBundle {
        zone: Zone::Play,
        center: ZoneCenter(Vec2::new(0., 0.)),
        size: ZoneIndex(0),
    });
}

#[derive(Debug, Event)]
pub struct CreateCard {
    pub zone: Zone,
}

#[derive(Resource, Debug)]
struct FactoryState {
    card_id: u16,
    mesh: Mesh2dHandle,
    material: Handle<ColorMaterial>,
}

impl FromWorld for FactoryState {
    fn from_world(world: &mut World) -> Self {
        // TODO: ensure card startup is first
        let dims = world.get_resource::<CardDims>().unwrap().get_dims();

        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh = Mesh2dHandle(meshes.add(Rectangle::new(dims.x, dims.y)));

        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let color = Color::srgb(139., 69., 19.);
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
    query: Query<(&Zone, &ZoneCenter, &ZoneIndex)>,
) {
    for ev in ev_cc.read() {
        let Some((_zone, center, size)) =
            query.iter().find(|(zone, _center, _size)| **zone == ev.zone)
        else {
            panic!("missing zone")
        };
        let posn = zone_index_to_posn(
            center, &ZoneIndex(size.0 + 1), size, &dims.get_dims()
        );
        commands.spawn(CardBundle {
            card: Card { id: state.card_id },
            mesh: MaterialMesh2dBundle {
                mesh: state.mesh.clone(),
                material: state.material.clone(),
                transform: Transform::from_translation(posn.extend(0.)),
                     ..default()
             },
             target: Target(posn),
         }).insert(ev.zone).insert(size.clone());

        ev_zu.send(ZoneUpdate {
            zone: ev.zone,
            joining: true,
            index: size.0,
        });
     }
}
