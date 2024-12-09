pub mod npc;
pub mod player;

use bevy::prelude::*;
use npc::NpcPlugin;
use player::PlayerPlugin;
use smart_default::SmartDefault;

use super::item::storage::ItemStorage;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Character>();
        app.register_type::<Health>();
        app.register_type::<Speed>();
        app.add_plugins((PlayerPlugin, NpcPlugin));
    }
}

#[derive(Component, Default, Reflect, Debug)]
#[require(Transform, Name(|| Name::new("Character")), Health, Speed, ItemStorage)]
pub struct Character;

#[derive(Component, SmartDefault, Reflect, Debug)]
pub struct Health {
    #[default(100)]
    pub current: u16,
    #[default(100)]
    pub max: u16,
}

#[derive(Component, SmartDefault, Reflect, Debug)]
pub struct Speed(#[default(5.0)] pub f32);
