use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{EguiContexts, EguiPlugin};
use bevy_inspector_egui::egui::{self, Align2, Color32, Vec2};

use super::character::player::Player;
use super::character::{Character, Health};
use super::input::GameplayInput;
use super::item::storage::{DropItemCommand, ItemStorage};
use super::item::{Item, ItemDescription};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EguiPlugin, MeshPickingPlugin));

        app.register_type::<FocusedEntity>();
        app.register_type::<InventoryState>();
        app.init_resource::<FocusedEntity>();
        app.init_resource::<InventoryState>();
        app.add_systems(Update, (select_entity, draw_focus_ui, draw_inventory));
    }
}

#[derive(Resource, Default, Reflect, Debug)]
#[reflect(Resource)]
pub struct FocusedEntity(pub Option<Entity>);

fn select_entity(
    mut over_events: EventReader<Pointer<Over>>,
    mut out_events: EventReader<Pointer<Out>>,
    mut focused_entity: ResMut<FocusedEntity>,
    filter_query: Query<(), Or<(With<Character>, With<ItemStorage>, With<Item>)>>,
) {
    for out in out_events.read() {
        if focused_entity.0.is_some_and(|e| e == out.target) {
            focused_entity.0 = None;
        }
    }

    for over in over_events.read() {
        if filter_query.get(over.target).is_ok() {
            focused_entity.0 = Some(over.target);
        }
    }
}

fn draw_focus_ui(
    mut contexts: EguiContexts,
    focused_entity: Res<FocusedEntity>,
    characters: Query<(&Name, &Health), (With<Character>, Without<Player>)>,
    items: Query<(&Name, &ItemDescription), With<Item>>,
) {
    let Some(entity) = focused_entity.0 else {
        return;
    };

    if let Ok((name, health)) = characters.get(entity) {
        egui::Window::new("Character")
            .anchor(Align2::CENTER_TOP, Vec2::new(0.0, 5.0))
            .min_size(Vec2::ZERO)
            .title_bar(false)
            .movable(false)
            .resizable(false)
            .show(contexts.ctx_mut(), |ui| {
                ui.add(
                    egui::ProgressBar::new(health.current as f32 / health.max as f32)
                        .fill(Color32::RED)
                        .text(format!(
                            "{} ({}/{})",
                            name.as_str(),
                            health.current,
                            health.max
                        )),
                )
            });
    }

    if let Ok((name, description)) = items.get(entity) {
        egui::Window::new("Item")
            .anchor(Align2::CENTER_TOP, Vec2::new(0.0, 5.0))
            .min_size(Vec2::ZERO)
            .title_bar(false)
            .movable(false)
            .resizable(false)
            .show(contexts.ctx_mut(), |ui| {
                ui.heading(name.as_str());
                ui.label(&description.0);
            });
    }
}

#[derive(Resource, Default, Reflect, Debug)]
#[reflect(Resource)]
pub struct InventoryState(pub bool);

fn draw_inventory(
    mut state: ResMut<InventoryState>,
    mut contexts: EguiContexts,
    mut commands: Commands,
    input: Res<GameplayInput>,
    inventory: Single<(Entity, &ItemStorage), With<Player>>,
    items: Query<&Name, With<Item>>,
) {
    if input.toggle_inventory {
        state.0 = !state.0;
    }

    if !state.0 {
        return;
    }

    egui::Window::new("Inventory")
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .collapsible(false)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            for item in inventory.1.items() {
                let name = items.get(*item).unwrap();
                if ui.button(name.as_str()).clicked() {
                    commands.queue(DropItemCommand {
                        storage: inventory.0,
                        item: *item,
                    });
                }
            }
        });
}
