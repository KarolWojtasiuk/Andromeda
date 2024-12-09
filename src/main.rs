#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::prelude::*;
use engine::camera::{GameCamera, GameCameraTarget};
use engine::character::Speed;
use engine::character::npc::Npc;
use engine::character::player::Player;
use engine::item::storage::InsertItemCommand;
use engine::item::{Item, ItemDescription};
use engine::{GameInfo, create_app};
use rand::Rng;

mod engine;

fn main() -> AppExit {
    let mut app = create_app(GameInfo {
        name: env!("CARGO_PKG_NAME"),
        version: Some(env!("CARGO_PKG_VERSION")),
    });

    app.add_systems(Startup, setup);
    app.run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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

    let capsule = meshes.add(Capsule3d::new(0.5, 1.0));
    let player = commands
        .spawn((
            Player,
            GameCameraTarget,
            Mesh3d(capsule.clone()),
            MeshMaterial3d(materials.add(Color::linear_rgb(1.0, 0.8, 0.0))),
        ))
        .id();

    commands.spawn((
        Item,
        Name::new("Chestplate"),
        ItemDescription("Heavy steel chestplate".to_string()),
        Transform::default(),
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.1, 0.5))),
        MeshMaterial3d(materials.add(Color::linear_rgb(0.5, 0.5, 1.0))),
    ));

    let sword = commands
        .spawn((
            Item,
            Name::new("Sword"),
            ItemDescription("Long steel sword".to_string()),
            Transform::default(),
            Mesh3d(meshes.add(Cuboid::new(0.4, 0.1, 1.25))),
            MeshMaterial3d(materials.add(Color::linear_rgb(0.3, 0.3, 0.3))),
        ))
        .id();
    commands.queue(InsertItemCommand {
        storage: player,
        item: sword,
    });

    let red = materials.add(Color::linear_rgb(1.0, 0.0, 0.0));
    for x in -10..10 {
        for z in -10..10 {
            commands.spawn((
                Npc::default(),
                Speed(rand::thread_rng().gen_range(3.0..10.0)),
                Transform::from_xyz(x as f32 * 5.0, 0.0, 75.0 + z as f32 * 5.0),
                Mesh3d(capsule.clone()),
                MeshMaterial3d(red.clone()),
            ));
        }
    }
}
