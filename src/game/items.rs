use bevy::prelude::*;
use bevy::utils::HashMap;
use derive_more::derive::{Display, FromStr};

use crate::engine::item::{Item, ItemDescription};
use crate::engine::prototype::{PrototypeBundle, PrototypeRegistry};

#[derive(FromStr, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameItemId {
    Chestplate,
    LongSword,
}

pub fn create_item_registry(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> PrototypeRegistry<GameItemId> {
    let mut map = HashMap::new();

    map.insert(
        GameItemId::Chestplate,
        Box::new((
            Item,
            Name::new("Chestplate"),
            ItemDescription("Heavy steel chestplate".to_string()),
            Mesh3d(meshes.add(Cuboid::new(0.5, 0.1, 0.5))),
            MeshMaterial3d(materials.add(Color::linear_rgb(0.5, 0.5, 1.0))),
        )) as Box<dyn PrototypeBundle<GameItemId>>,
    );

    map.insert(
        GameItemId::LongSword,
        Box::new((
            Item,
            Name::new("Sword"),
            ItemDescription("Long steel sword".to_string()),
            Mesh3d(meshes.add(Cuboid::new(0.4, 0.1, 1.25))),
            MeshMaterial3d(materials.add(Color::linear_rgb(0.3, 0.3, 0.3))),
        )) as Box<dyn PrototypeBundle<GameItemId>>,
    );

    PrototypeRegistry::new(map)
}
