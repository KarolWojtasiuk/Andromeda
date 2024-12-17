mod characters;
mod items;

use bevy::prelude::*;
use characters::{GameCharacterId, create_character_registry};
use items::{GameItemId, create_item_registry};
use noise::{NoiseFn, Perlin};
use rand::Rng;

use super::engine::camera::GameCamera;
use super::engine::item::storage::InsertItemCommand;
use super::engine::{GameInfo, create_app};
use crate::engine;
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
    app.add_systems(Update, regenerate_world);
    app.run()
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    character_registry: Res<PrototypeRegistry<GameCharacterId>>,
    item_registry: Res<PrototypeRegistry<GameItemId>>,
) {
    commands.spawn(GameCamera {
        offset: Vec3::new(0.0, 1.0, 0.5),
        direction: Dir3::from_xyz(0.0, 1.0, 3.0).unwrap(),
        min_distance: 3.0,
        max_distance: 1000.0,
        distance: 250.0,
        ..default()
    });
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY * 2.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(3.0, 10.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    const SIZE: UVec2 = UVec2::new(1000, 1000);
    const CELL_SIZE: f32 = 2.0;
    commands.spawn((
        Transform::from_xyz(
            -(CELL_SIZE * SIZE.x as f32 / 2.0),
            0.0,
            -(CELL_SIZE * SIZE.y as f32 / 2.0),
        ),
        GameWorld,
        MeshMaterial3d(materials.add(Color::WHITE)),
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
}

#[derive(Component)]
struct GameWorld;

fn regenerate_world(
    keys: Res<ButtonInput<KeyCode>>,
    world: Single<(Entity, &mut Transform, Has<Mesh3d>), With<GameWorld>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    const SIZE: UVec2 = UVec2::new(500, 500);
    const CELL_SIZE: f32 = 10.0;

    if !world.2 || keys.pressed(KeyCode::Space) {
        commands
            .entity(world.0)
            .insert(Mesh3d(meshes.add(generate_world(SIZE, CELL_SIZE))))
            .insert(Transform::from_xyz(
                -(CELL_SIZE * SIZE.x as f32 / 2.0),
                0.0,
                -(CELL_SIZE * SIZE.y as f32 / 2.0),
            ));
    }
}

fn generate_world(size: UVec2, cell_size: f32) -> Mesh {
    let mut rng = rand::thread_rng();

    let scale = rng.gen_range(4.0..5.0);
    let offset = Vec2::new(rng.gen_range(0.0..1000.0), rng.gen_range(0.0..100.0));
    let height_scale = rng.gen_range(25.0..75.0);

    let height_perlin = Perlin::new(rand::random());
    let height_noise = |x, y| {
        height_perlin.get([
            ((x + offset.x) * scale) as f64,
            ((y + offset.y) * scale) as f64,
        ]) as f32
            * height_scale
    };

    let color_perlin = Perlin::new(rand::random());
    let color_noise = |x, y| {
        Color::hsl(
            (color_perlin.get([x as f64, y as f64]) as f32 + 1.0) * 180.0,
            0.8,
            0.4,
        )
    };

    engine::world_generator::generate_world(size, cell_size, height_noise, color_noise)
}
