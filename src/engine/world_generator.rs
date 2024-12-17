use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

pub fn generate_world(
    size: UVec2,
    cell_size: f32,
    height_noise: impl Fn(f32, f32) -> f32,
    color_noise: impl Fn(f32, f32) -> Color,
) -> Mesh {
    assert!(size.x % 2 == 0);
    assert!(size.y % 2 == 0);

    let mut vertices = vec![];
    let mut indices = vec![];
    let mut colors = vec![];

    let generate_vertex = |x: u32, y: u32| {
        [
            x as f32 * cell_size,
            height_noise(
                ((x as f32 - size.x as f32 / 2.0) / size.x as f32 / 2.0) * cell_size,
                ((y as f32 - size.y as f32 / 2.0) / size.y as f32 / 2.0) * cell_size,
            ),
            y as f32 * cell_size,
        ]
    };
    let generate_color = |x: u32, y: u32| {
        color_noise(
            (x as f32 - size.x as f32 / 2.0) / size.x as f32 / 2.0,
            (y as f32 - size.y as f32 / 2.0) / size.y as f32 / 2.0,
        )
        .to_linear()
        .to_f32_array()
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
        colors.extend([
            generate_color(x, y),
            generate_color(x, y + 1),
            generate_color(x + 1, y + 1),
            generate_color(x + 1, y),
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
    .with_inserted_indices(Indices::U32(indices))
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
    .with_duplicated_vertices()
    .with_computed_normals()
}
