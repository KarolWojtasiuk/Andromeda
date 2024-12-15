mod characters;
mod items;

use bevy::prelude::*;
use characters::{GameCharacterId, create_character_registry};
use items::{GameItemId, create_item_registry};
use rand::Rng;

use super::engine::camera::GameCamera;
use super::engine::character::Speed;
use super::engine::item::storage::InsertItemCommand;
use super::engine::{GameInfo, create_app};
use crate::engine::prototype::PrototypeRegistry;

pub fn run() -> AppExit {
    let mut app = create_app(
        GameInfo {
            name: env!("CARGO_PKG_NAME"),
            version: Some(env!("CARGO_PKG_VERSION")),
        },
        create_character_registry,
        create_item_registry,
    );

    app.add_systems(Startup, setup);
    app.run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    character_registry: Res<PrototypeRegistry<GameCharacterId>>,
    item_registry: Res<PrototypeRegistry<GameItemId>>,
) {
    commands.spawn(GameCamera::default());
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY * 2.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(3.0, 10.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Transform::from_xyz(0.0, -1.0, 0.0),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(1000.0, 1000.0)))),
        MeshMaterial3d(materials.add(Color::linear_rgb(0.1, 0.3, 0.1))),
    ));

    let player = character_registry.spawn(GameCharacterId::Player, &mut commands);

    let sword = item_registry.spawn_at(GameItemId::LongSword, Transform::default(), &mut commands);
    commands.queue(InsertItemCommand {
        storage: player,
        item: sword,
    });

    item_registry.spawn_at(
        GameItemId::Chestplate,
        Transform::from_xyz(-10.0, 0.0, 2.5),
        &mut commands,
    );

    for x in -10..10 {
        for z in -10..10 {
            let enemy = character_registry.spawn_at(
                GameCharacterId::Enemy,
                Transform::from_xyz(x as f32 * 5.0, 0.0, 75.0 + z as f32 * 5.0),
                &mut commands,
            );

            commands
                .entity(enemy)
                .insert(Speed(rand::thread_rng().gen_range(3.0..10.0)));
        }
    }
}
