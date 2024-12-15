use bevy::prelude::*;

use super::{Character, Speed};
use crate::engine::input::GameplayInput;
use crate::engine::item::Item;
use crate::engine::item::storage::InsertItemCommand;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MeshPickingPlugin);

        app.register_type::<Player>();
        app.add_systems(Update, (move_player, pickup_items));
    }
}

#[derive(Component, Default, Clone, Reflect, Debug)]
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

fn pickup_items(
    mut commands: Commands,
    mut click_events: EventReader<Pointer<Down>>,
    items: Query<&GlobalTransform, (With<Item>, With<Transform>)>,
    player: Single<(Entity, &GlobalTransform), With<Player>>,
) {
    for click in click_events.read() {
        if let Ok(item_transform) = items.get(click.target) {
            if item_transform
                .translation()
                .distance(player.1.translation())
                <= 5.0
            {
                commands.queue(InsertItemCommand {
                    storage: player.0,
                    item: click.target,
                });
            }
        }
    }
}
