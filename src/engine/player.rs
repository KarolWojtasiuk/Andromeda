use bevy::prelude::*;

use super::input::GameplayInput;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
    }
}

#[derive(Component, Default, Reflect, Debug)]
#[require(Transform, Name(|| Name::new("Player")))]
pub struct Player;

fn move_player(
    mut player: Single<&mut Transform, With<Player>>,
    input: Res<GameplayInput>,
    time: Res<Time>,
) {
    if input.movement == Vec2::ZERO {
        return;
    }

    let direction = Vec3::new(input.movement.x, 0.0, -input.movement.y);
    let speed = if input.sprint { 10.0 } else { 5.0 };
    player.translation += direction * speed * time.delta_secs();
}
