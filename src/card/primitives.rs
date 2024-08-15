use bevy::{
    prelude::*,
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
    pub id: u16,
}

#[derive(Component, Debug)]
pub struct Target(pub Vec3);

#[derive(Bundle)]
pub struct CardBundle {
    pub card: Card,
    pub mesh: PbrBundle,
    pub target: Target,
}

const CARD_WIDTH: f32 = 10.;
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

fn move_card(
    mut query: Query<(&mut Transform, &Target), With<Card>>,
) {
    for (mut transform, target) in query.iter_mut() {
        let x_diff = target.0.x - transform.translation.x;
        let y_diff = transform.translation.y - target.0.y;

        let rot_target = if x_diff.abs() + y_diff.abs() < 0.01 {
            Quat::IDENTITY
        } else {
            let x_rot = Quat::from_rotation_x(y_diff * 0.02);
            let y_rot = Quat::from_rotation_y(x_diff * 0.02);
            x_rot.mul_quat(y_rot)
        };
        transform.rotation = transform.rotation.slerp(rot_target, 0.1);
        transform.translation = transform.translation.lerp(target.0, 0.1);
    }
}
