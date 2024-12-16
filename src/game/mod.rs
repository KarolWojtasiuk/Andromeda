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

    const SIZE: UVec2 = UVec2::new(1000, 1000);
    const CELL_SIZE: f32 = 1.5;
    commands.spawn((
        Transform::from_xyz(
            -(CELL_SIZE * SIZE.x as f32 / 2.0),
            -1.0,
            -(CELL_SIZE * SIZE.y as f32 / 2.0),
        ),
        Mesh3d(meshes.add(generate_world(
            2137,
            SIZE,
            CELL_SIZE,
            2.5,
            Vec2::new(123.4, 432.1),
            100.0,
        ))),
        MeshMaterial3d(materials.add(Color::linear_rgb(0.3, 0.1, 0.6))),
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

fn generate_world(
    seed: u32,
    size: UVec2,
    cell_size: f32,
    noise_scale: f32,
    noise_offset: Vec2,
    height_scale: f32,
) -> Mesh {
    assert!(size.x % 2 == 0);
    assert!(size.y % 2 == 0);
    assert!(height_scale > 0.0);

    let mut vertices = vec![];
    let mut indices = vec![];

    let perlin = noise::Perlin::new(seed);
    let generate_vertex = |x: u32, y: u32| {
        [
            x as f32 * cell_size,
            perlin.get([
                cell_size as f64
                    * noise_scale as f64
                    * (noise_offset.x as f64 + x as f64 / size.x as f64),
                cell_size as f64
                    * noise_scale as f64
                    * (noise_offset.y as f64 + y as f64 / size.y as f64),
            ]) as f32
                * height_scale,
            y as f32 * cell_size,
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
            vertices.len() as u32 - 4,
            vertices.len() as u32 - 3,
            vertices.len() as u32 - 1,
            vertices.len() as u32 - 1,
            vertices.len() as u32 - 3,
            vertices.len() as u32 - 2,
        ]);

        if x + 2 < size.x {
            x += 1;
        } else if y + 2 < size.y {
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
