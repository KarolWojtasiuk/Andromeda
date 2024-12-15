use std::fmt::Display;
use std::hash::Hash;
use std::str::FromStr;

use bevy::prelude::*;
use bevy::utils::HashMap;
use derive_more::derive::Constructor;

pub trait PrototypeId:
    FromStr + Display + Clone + Copy + Eq + Hash + Sync + Send + 'static
{
}

impl<T: FromStr + Display + Clone + Copy + Eq + Hash + Sync + Send + 'static> PrototypeId for T {}

#[derive(Component)]
pub struct PrototypeInstance<T: PrototypeId>(T);

impl<T: PrototypeId> PrototypeInstance<T> {
    pub fn id(&self) -> T {
        self.0
    }
}

#[derive(Resource, Constructor)]
pub struct PrototypeRegistry<T: PrototypeId>(HashMap<T, Box<dyn PrototypeBundle<T>>>);

impl<T: PrototypeId> PrototypeRegistry<T> {
    pub fn spawn(&self, id: T, commands: &mut Commands) -> Entity {
        let prototype = self
            .0
            .get(&id)
            .unwrap_or_else(|| panic!("Prototype with id '{}' does not exist in registry", id));
        prototype.spawn(id, commands)
    }

    pub fn spawn_at(&self, id: T, transform: Transform, commands: &mut Commands) -> Entity {
        let entity = self.spawn(id, commands);
        commands.entity(entity).insert(transform);
        entity
    }
}

pub trait PrototypeBundle<Id: PrototypeId>: Send + Sync {
    fn spawn(&self, id: Id, commands: &mut Commands) -> Entity;
}

impl<T: Bundle + Clone, Id: PrototypeId> PrototypeBundle<Id> for T {
    fn spawn(&self, id: Id, commands: &mut Commands) -> Entity {
        commands.spawn((PrototypeInstance(id), self.clone())).id()
    }
}
