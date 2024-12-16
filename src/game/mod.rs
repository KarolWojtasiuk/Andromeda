mod characters;
mod items;

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use characters::{GameCharacterId, create_character_registry};
use items::{GameItemId, create_item_registry};
use noise::NoiseFn;

use super::engine::camera::GameCamera;
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
        Mesh3d(meshes.add(generate_world(12534, 100, 100, 4.0, 10.0))),
        MeshMaterial3d(materials.add(Color::linear_rgb(0.4, 0.1, 0.9))),
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

    // for x in -10..10 {
    //     for z in -10..10 {
    //         let enemy = character_registry.spawn_at(
    //             GameCharacterId::Enemy,
    //             Transform::from_xyz(x as f32 * 5.0, 0.0, 75.0 + z as f32 * 5.0),
    //             &mut commands,
    //         );

    //         commands
    //             .entity(enemy)
    //             .insert(Speed(rand::thread_rng().gen_range(3.0..10.0)));
    //     }
    // }
}

fn generate_world(seed: u32, width: u16, height: u16, scale: f64, max_elevation: f32) -> Mesh {
    assert!(width % 2 == 0);
    assert!(height % 2 == 0);
    assert!(max_elevation > 0.0);

    let mut vertices = vec![];
    let mut indices = vec![];

    let perlin = noise::Perlin::new(seed);
    let generate_vertex = |x: u16, y: u16| {
        [
            x as f32,
            perlin.get([
                scale * (x as f64 / width as f64),
                scale * (y as f64 / height as f64),
            ]) as f32
                * max_elevation,
            y as f32,
        ]
    };

    let mut x = 0;
    let mut y = 0;
    loop {
        vertices.extend([
            generate_vertex(x, y),
            generate_vertex(x, y + 1),
            generate_vertex(x + 1, y + 1),
            generate_vertex(x + 1, y),
        ]);
        indices.extend([
            vertices.len() as u32 - 4, // 0
            vertices.len() as u32 - 3, // 1
            vertices.len() as u32 - 1, // 3
            vertices.len() as u32 - 1, // 3
            vertices.len() as u32 - 3, // 1
            vertices.len() as u32 - 2, // 2
        ]);

        if x + 2 < width {
            x += 1;
        } else if y + 2 < height {
            x = 0;
            y += 1;
        } else {
            break;
        }
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; vertices.len()])
    .with_inserted_indices(Indices::U32(indices))
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_duplicated_vertices()
    .with_computed_normals()
}
