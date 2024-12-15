use bevy::prelude::*;

use crate::engine::item::Item;

pub struct ItemStoragePlugin;

impl Plugin for ItemStoragePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ItemStorage>();
    }
}

#[derive(Component, Default, Reflect, Debug)]
pub struct ItemStorage;

pub struct ItemStorageItems<'a> {
    items: Query<'a, 'a, (Entity, &'a Parent), (With<Item>, Without<Transform>)>,
}

impl ItemStorageItems<'_> {
    #[allow(dead_code)]
    pub fn get(&self, entity: Entity) -> Vec<Entity> {
        self.items
            .iter()
            .filter(|(_, p)| p.get() == entity)
            .map(|(e, _)| e)
            .collect()
    }
}

pub struct InsertItemCommand {
    pub storage: Entity,
    pub item: Entity,
}

impl Command for InsertItemCommand {
    fn apply(self, world: &mut World) {
        assert!(world.entity(self.item).contains::<Item>());

        world
            .entity_mut(self.item)
            .remove::<(Transform, GlobalTransform)>()
            .set_parent(self.storage);
    }
}

pub struct DropItemCommand {
    pub storage: Entity,
    pub item: Entity,
}

impl Command for DropItemCommand {
    fn apply(self, world: &mut World) {
        assert!(world.entity(self.item).contains::<Item>());
        assert!(
            world
                .entity(self.item)
                .get::<Parent>()
                .is_some_and(|p| p.get() == self.storage)
        );

        let transform = world
            .entity(self.storage)
            .get::<GlobalTransform>()
            .unwrap()
            .compute_transform();
        world
            .entity_mut(self.item)
            .insert(transform)
            .remove_parent();
    }
}
