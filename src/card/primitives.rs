use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CardDims>()
            .add_systems(Update, move_card);
    }
}

#[derive(Component, Debug)]
pub struct Card {
    id: u16,
}

#[derive(Component, Debug)]
pub struct Target(pub Vec2);

#[derive(Bundle)]
struct CardBundle {
    card: Card,
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    target: Target,
}

const CARD_WIDTH: f32 = 63.;
const CARD_RATIO: f32 = 1.4;
#[derive(Resource, Debug)]
pub struct CardDims {
    width: f32,
    height: f32,
}

impl CardDims {
    pub fn get_dims(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }

    pub fn scale_dims(&mut self, scale: f32) {
        self.width *= scale;
        self.height = CARD_RATIO * self.width;
    }
}

impl Default for CardDims {
    fn default() -> Self {
        CardDims { width: CARD_WIDTH, height: CARD_WIDTH * CARD_RATIO }
    }
}

fn interpolate(from: Vec2, to: Vec2) -> Vec2 {
    let interpolation_factor = 0.1;
    let x = from.x + (to.x - from.x) * interpolation_factor;
    let y = from.y + (to.y - from.y) * interpolation_factor;
    Vec2::new(x, y)
}

fn move_card(
    mut query: Query<(&mut Transform, &Target), With<Card>>,
) {
    for (mut transform, target) in query.iter_mut() {
        transform.translation = interpolate(
            transform.translation.truncate(),
            target.0,
        ).extend(transform.translation.z)
    }
}
