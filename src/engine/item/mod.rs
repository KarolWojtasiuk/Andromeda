pub mod storage;

use bevy::prelude::*;
use storage::ItemStoragePlugin;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Item>();
        app.register_type::<ItemDescription>();
        app.register_type::<ItemValue>();
        app.add_plugins(ItemStoragePlugin);

        #[cfg(debug_assertions)]
        app.add_systems(Update, validate_item_assumption);
    }
}

/*
Game items can exist in two forms
- World item which can be picked by character
- Item inside storage (chest or character inventory)

To differentiate these forms we need some assumption based on whether the entity has Parent and Transform components
| Transform | Parent |                   Result                    |
|-----------|--------|---------------------------------------------|
|✅         |✅      |World item inside some parent, can be picked |
|✅         |❌      |World item without parent, can be picked too |
|❌         |✅      |Item inside some storage, can be dropped     |
|❌         |❌      |Idk what is this, treat this as invalid state|
*/

#[cfg(debug_assertions)]
fn validate_item_assumption(
    items: Populated<(Entity, &Name), (With<Item>, Without<Transform>, Without<Parent>)>,
    mut commands: Commands,
) {
    for (entity, name) in items.iter() {
        error!(
            "Item '{}' ({}) is in invalid state, entity components will be logged below.",
            name, entity
        );
        commands.entity(entity).log_components();
    }
}

#[derive(Component, Default, Reflect, Debug)]
#[require(Name(|| Name::new("Item")), ItemDescription, ItemValue)]
pub struct Item;

#[derive(Component, Default, Reflect, Debug)]
pub struct ItemDescription(pub String);

#[derive(Component, Default, Reflect, Debug)]
pub struct ItemValue(pub u16);
