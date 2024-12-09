use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui;

use super::character::player::Player;
use super::character::{Character, Health, Speed};
use super::item::storage::{DropItemCommand, InsertItemCommand, ItemStorage};
use super::item::{Item, ItemDescription, ItemValue};

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
    filter_query: Query<(), Or<(With<Character>, With<ItemStorage>, With<Item>)>>,
) {
    for click in click_events.read() {
        if filter_query.get(click.target).is_ok() {
            selected_entity.0 = Some(click.target);
        }
    }
}

fn draw_ui(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut selected_entity: ResMut<SelectedDebugEntity>,
    cores: Query<(Option<&Name>, Option<&Parent>)>,
    spatials: Query<(&Transform, &GlobalTransform)>,
    characters: Query<(&Health, &Speed), With<Character>>,
    item_storages: Query<&ItemStorage>,
    items: Query<(&ItemDescription, &ItemValue, Option<&Parent>), With<Item>>,
    player: Single<Entity, With<Player>>,
) {
    let Some(entity) = selected_entity.0 else {
        return;
    };

    grid_window("Core", contexts.ctx_mut(), |ui| {
        grid_row(ui, "Id", |ui| {
            ui.label(entity.to_string());
        });

        if let Ok((name, parent)) = cores.get(entity) {
            grid_row(ui, "Name", |ui| {
                ui.label(name.map_or("", |n| n.as_str()));
            });
            grid_row(ui, "Parent", |ui| {
                ui.horizontal(|ui| {
                    ui.label(parent.map_or("".to_string(), |p| p.get().to_string()));

                    if let Some(parent) = parent {
                        if ui.button("Select").clicked() {
                            selected_entity.0 = Some(parent.get());
                        }
                    }
                });
            });
        };

        grid_row(ui, "Actions", |ui| {
            if ui.button("Unselect").clicked() {
                selected_entity.0 = None;
            }
            if ui.button("Despawn").clicked() {
                selected_entity.0 = None;
                commands.queue(DespawnRecursive { warn: true, entity });
            }
        });
    });

    if let Ok((trasform, global_transform)) = spatials.get(entity) {
        grid_window("Spatial", contexts.ctx_mut(), |ui| {
            grid_row(ui, "Translation", |ui| {
                ui.label(trasform.translation.to_string());
            });
            grid_row(ui, "Rotation", |ui| {
                ui.label(trasform.rotation.to_string());
            });
            grid_row(ui, "Scale", |ui| {
                ui.label(trasform.scale.to_string());
            });

            let global_transform = global_transform.compute_transform();
            grid_row(ui, "Global Translation", |ui| {
                ui.label(global_transform.translation.to_string());
            });
            grid_row(ui, "Global Rotation", |ui| {
                ui.label(global_transform.rotation.to_string());
            });
            grid_row(ui, "Global Scale", |ui| {
                ui.label(global_transform.scale.to_string());
            });
        });
    }

    if let Ok((health, speed)) = characters.get(entity) {
        grid_window("Character", contexts.ctx_mut(), |ui| {
            grid_row(ui, "Health", |ui| {
                ui.label(format!("{}/{}", health.current, health.max));
            });
            grid_row(ui, "Speed", |ui| {
                ui.label(speed.0.to_string());
            });
        });
    };

    if let Ok(storage) = item_storages.get(entity) {
        grid_window("Item Storage", contexts.ctx_mut(), |ui| {
            for (item, core) in storage.items().iter().map(|i| (i, cores.get(*i))) {
                grid_row(ui, &item.to_string(), |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            core.map_or("<UNKNOWN>", |c: (Option<&Name>, Option<&Parent>)| {
                                c.0.map_or("", |n| n.as_str())
                            }),
                        );
                        if ui.button("Drop").clicked() {
                            commands.queue(DropItemCommand {
                                storage: entity,
                                item: *item,
                            });
                        }
                        if ui.button("Select").clicked() {
                            selected_entity.0 = Some(*item);
                        }
                    });
                });
            }
        });
    };

    if let Ok((description, value, parent)) = items.get(entity) {
        grid_window("Item", contexts.ctx_mut(), |ui| {
            grid_row(ui, "Description", |ui| {
                ui.label(description.0.as_str());
            });
            grid_row(ui, "Value", |ui| {
                ui.label(value.0.to_string());
            });
            grid_row(ui, "Actions", |ui| {
                let is_world_item = spatials.contains(entity);

                ui.add_enabled_ui(is_world_item, |ui| {
                    if ui.button("Insert").clicked() {
                        commands.queue(InsertItemCommand {
                            storage: *player,
                            item: entity,
                        });
                    }
                });
                ui.add_enabled_ui(!is_world_item, |ui| {
                    if ui.button("Drop").clicked() {
                        commands.queue(DropItemCommand {
                            storage: parent.unwrap().get(),
                            item: entity,
                        });
                    }
                });
            });
        });
    };
}

fn grid_window(
    name: &'static str,
    context: &mut egui::Context,
    content: impl FnOnce(&mut egui::Ui),
) {
    egui::Window::new(name)
        .default_open(false)
        .show(context, |ui| {
            egui::Grid::new(name).striped(true).show(ui, content)
        });
}

fn grid_row(ui: &mut egui::Ui, label: &str, value: impl FnOnce(&mut egui::Ui)) {
    ui.label(label);
    value(ui);
    ui.end_row();
}
