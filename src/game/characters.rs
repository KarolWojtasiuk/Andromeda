use bevy::prelude::*;
use bevy::utils::HashMap;
use derive_more::derive::{Display, FromStr};

use crate::engine::camera::GameCameraTarget;
use crate::engine::character::npc::Npc;
use crate::engine::character::player::Player;
use crate::engine::prototype::{PrototypeBundle, PrototypeRegistry};

#[derive(FromStr, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameCharacterId {
    Player,
    Enemy,
}

pub fn create_character_registry(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> PrototypeRegistry<GameCharacterId> {
    let mut map = HashMap::new();

    map.insert(
        GameCharacterId::Player,
        Box::new((
            Player,
            GameCameraTarget,
            Mesh3d(meshes.add(Capsule3d::new(0.5, 1.0))),
            MeshMaterial3d(materials.add(Color::linear_rgb(1.0, 0.8, 0.0))),
        )) as Box<dyn PrototypeBundle<GameCharacterId>>,
    );

    map.insert(
        GameCharacterId::Enemy,
        Box::new((
            Npc::default(),
            Mesh3d(meshes.add(Capsule3d::new(0.5, 1.0))),
            MeshMaterial3d(materials.add(Color::linear_rgb(1.0, 0.0, 0.0))),
        )) as Box<dyn PrototypeBundle<GameCharacterId>>,
    );

    PrototypeRegistry::new(map)
}
