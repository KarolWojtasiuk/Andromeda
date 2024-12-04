use super::input::GameplayInput;
use crate::engine::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
    }
}

#[derive(Component, Default, Debug)]
#[require(Transform)]
pub struct Player;

fn move_player(
    mut player: Single<&mut Transform, With<Player>>,
    input: Res<GameplayInput>,
    time: Res<Time>,
) {
    if input.movement == Vec2::ZERO {
        return;
    }

    let direction = Vec3::new(input.movement.x, input.movement.y, 0.0);
    let speed = if input.sprint { 500.0 } else { 300.0 };
    player.translation += direction * speed * time.delta_secs();
}
