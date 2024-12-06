use bevy::prelude::*;

use super::{Character, Speed};
use crate::engine::input::GameplayInput;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>();
        app.add_systems(Update, move_player);
    }
}

#[derive(Component, Default, Reflect, Debug)]
#[require(Name(|| Name::new("Player")), Character)]
pub struct Player;

fn move_player(
    mut player: Single<(&mut Transform, &Speed), With<Player>>,
    input: Res<GameplayInput>,
    time: Res<Time>,
) {
    if input.movement == Vec2::ZERO {
        return;
    }

    let direction = Vec3::new(input.movement.x, 0.0, -input.movement.y);
    let speed = {
        let speed = if input.sprint {
            player.1.0 * 2.0
        } else {
            player.1.0
        };
        speed.clamp(0.0, 100.0)
    };
    player.0.translation += direction * speed * time.delta_secs();
}
