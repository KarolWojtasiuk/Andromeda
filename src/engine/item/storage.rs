use bevy::prelude::*;

pub struct ItemStoragePlugin;

impl Plugin for ItemStoragePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ItemStorage>();
    }
}

#[derive(Component, Default, Reflect, Debug)]
pub struct ItemStorage(pub(self) Vec<Entity>);

impl ItemStorage {
    pub fn items(&self) -> &[Entity] {
        &self.0
    }
}

pub struct InsertItemCommand {
    pub storage: Entity,
    pub item: Entity,
}

impl Command for InsertItemCommand {
    fn apply(self, world: &mut World) {
        debug_assert!(
            world
                .entity_mut(self.storage)
                .get_mut::<ItemStorage>()
                .is_some_and(|s| !s.0.contains(&self.item))
        );

        world
            .entity_mut(self.storage)
            .get_mut::<ItemStorage>()
            .unwrap()
            .0
            .push(self.item);
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
        debug_assert!(
            world
                .entity_mut(self.item)
                .get::<Parent>()
                .is_some_and(|p| p.get() == self.storage)
        );
        debug_assert!(
            world
                .entity_mut(self.storage)
                .get_mut::<ItemStorage>()
                .is_some_and(|s| s.0.contains(&self.item))
        );

        world
            .entity_mut(self.storage)
            .get_mut::<ItemStorage>()
            .unwrap()
            .0
            .retain(|e| *e != self.item);

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
