use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;
use smart_default::SmartDefault;

use super::{Character, Speed};

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Npc>();
        app.add_systems(Update, move_npcs);
    }
}

#[derive(Component, SmartDefault, Reflect, Debug)]
#[require(Name(|| Name::new("NPC")), Character)]
pub enum Npc {
    #[default]
    Idle(#[default(Timer::new(Duration::from_secs(1), TimerMode::Once))] Timer),
    Moving(Vec3),
}

fn move_npcs(mut npcs: Query<(&mut Transform, &mut Npc, &Speed)>, time: Res<Time>) {
    // temporary ai implementation: move to random places xd

    let mut rng = rand::thread_rng();
    for (mut transform, mut npc, speed) in npcs.iter_mut() {
        match npc.as_mut() {
            Npc::Idle(timer) => {
                timer.tick(time.delta());
                if timer.finished() {
                    let direction = Vec3::new(
                        rng.gen_range(-1.0..1.0),
                        transform.translation.y,
                        rng.gen_range(-1.0..1.0),
                    )
                    .normalize();
                    let distance = rng.gen_range(1.0..25.0);

                    let target = transform.translation + direction * distance;
                    *npc = Npc::Moving(target);
                }
            }
            Npc::Moving(target) => {
                let direction = (*target - transform.translation).normalize_or_zero();
                transform.translation += direction * speed.0 * time.delta_secs();

                if transform.translation.distance(*target) < 0.1 {
                    *npc = Npc::Idle(Timer::new(
                        Duration::from_secs_f32(rng.gen_range(0.1..3.0)),
                        TimerMode::Once,
                    ))
                }
            }
        }
    }
}
