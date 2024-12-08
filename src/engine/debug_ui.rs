use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui::{self, Response};

use super::character::{Character, Health, Speed};
use super::item::Item;
use super::item::storage::ItemStorage;

pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MeshPickingPlugin);

        app.register_type::<SelectedDebugEntity>();
        app.init_resource::<SelectedDebugEntity>();
        app.add_systems(Update, (select_entity, draw_ui));
    }
}

#[derive(Resource, Default, Reflect, Debug)]
#[reflect(Resource)]
pub struct SelectedDebugEntity(pub Option<Entity>);

fn select_entity(
    mut click_events: EventReader<Pointer<Click>>,
    mut selected_entity: ResMut<SelectedDebugEntity>,
    filter_query: Query<(), Or<(With<Character>, With<ItemStorage>)>>,
) {
    for click in click_events.read() {
        if filter_query.get(click.target).is_ok() {
            selected_entity.0 = Some(click.target);
        }
    }
}

fn draw_ui(
    mut contexts: EguiContexts,
    selected_entity: Res<SelectedDebugEntity>,
    core: Query<(&Name, &Transform, &GlobalTransform)>,
    character: Query<(&Health, &Speed), With<Character>>,
    item_storages: Query<&ItemStorage>,
    items: Query<(Entity, &Name), With<Item>>,
) {
    let Some(entity) = selected_entity.0 else {
        return;
    };

    if let Ok((name, trasform, global_transform)) = core.get(entity) {
        grid_window("Core", contexts.ctx_mut(), |ui| {
            grid_row(ui, "Id", |ui| ui.label(entity.to_string()));
            grid_row(ui, "Name", |ui| ui.label(name.to_string()));
            grid_row(ui, "Translation", |ui| {
                ui.label(trasform.translation.to_string())
            });
            grid_row(ui, "Rotation", |ui| ui.label(trasform.rotation.to_string()));
            grid_row(ui, "Scale", |ui| ui.label(trasform.scale.to_string()));

            let global_transform = global_transform.compute_transform();
            grid_row(ui, "Global Translation", |ui| {
                ui.label(global_transform.translation.to_string())
            });
            grid_row(ui, "Global Rotation", |ui| {
                ui.label(global_transform.rotation.to_string())
            });
            grid_row(ui, "Global Scale", |ui| {
                ui.label(global_transform.scale.to_string())
            });
        });

        if let Ok((health, speed)) = character.get(entity) {
            grid_window("Character", contexts.ctx_mut(), |ui| {
                grid_row(ui, "Health", |ui| {
                    ui.label(format!("{}/{}", health.current, health.max))
                });
                grid_row(ui, "Speed", |ui| ui.label(speed.0.to_string()));
            });
        };

        if let Ok(storage) = item_storages.get(entity) {
            grid_window("Item Storage", contexts.ctx_mut(), |ui| {
                for (item, name) in storage
                    .items()
                    .iter()
                    .map(|i| (i, items.get(*i).map(|i| i.1)))
                {
                    grid_row(ui, &item.to_string(), |ui| {
                        ui.label(name.map_or("<Unknown>", |n| n.as_str()))
                    });
                }
            });
        };
    };
}

fn grid_window(
    name: &'static str,
    context: &mut egui::Context,
    content: impl FnOnce(&mut egui::Ui),
) {
    egui::Window::new(name).show(context, |ui| {
        egui::Grid::new(name).striped(true).show(ui, content)
    });
}

fn grid_row(ui: &mut egui::Ui, label: &str, value: impl FnOnce(&mut egui::Ui) -> Response) {
    ui.label(label);
    value(ui);
    ui.end_row();
}
