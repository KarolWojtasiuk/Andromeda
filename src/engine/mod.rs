mod ui;

pub mod camera;
pub mod character;
pub mod input;
pub mod item;

use bevy::diagnostic::{
    EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
};
use bevy::prelude::*;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::GameCameraPlugin;
use character::CharacterPlugin;
use clap::{ArgAction, Parser};
use input::GameInputPlugin;
use item::ItemPlugin;
use ui::GameUiPlugin;

pub fn create_app(info: GameInfo) -> App {
    let args = EngineArgs::parse();

    let mut app = App::new();
    app.insert_resource(info);
    app.add_plugins(DefaultPlugins.set(bevy::window::WindowPlugin {
        primary_window: Some(Window {
            title: info.name.to_string(),
            ..default()
        }),
        ..default()
    }));

    app.add_plugins((
        GameInputPlugin,
        GameCameraPlugin,
        CharacterPlugin,
        ItemPlugin,
        GameUiPlugin,
    ));

    if args.show_game_version_overlay {
        app.add_systems(Startup, spawn_info_overlay);
    }

    if args.enable_inspector {
        app.add_plugins((
            DefaultInspectorConfigPlugin,
            WorldInspectorPlugin::default(),
        ));
    }

    if args.enable_diagnostics {
        app.add_plugins((
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
        ));
    }

    app
}

#[derive(Parser)]
#[command(version)]
struct EngineArgs {
    #[arg(
        short = 'v',
        long = "version-overlay",
        help = "Show game version overlay",
        action = ArgAction::Set,
        default_value_t = true,
    )]
    pub show_game_version_overlay: bool,
    #[arg(
        short = 'i',
        long = "inspector",
        help = "Enable world inspector",
        default_value_t = false
    )]
    pub enable_inspector: bool,
    #[arg(
        short = 'l',
        long = "diagnostics-logger",
        help = "Enable diagnostics logging",
        default_value_t = false
    )]
    pub enable_diagnostics: bool,
}

#[derive(Resource, Clone, Copy)]
pub struct GameInfo {
    pub name: &'static str,
    pub version: Option<&'static str>,
}

fn spawn_info_overlay(mut commands: Commands, info: Res<GameInfo>) {
    let game_info = info
        .version
        .map_or(info.name.to_string(), |v| format!("{} {}", info.name, v));

    commands.spawn((Text(game_info), Node {
        position_type: PositionType::Absolute,
        bottom: Val::Px(4.0),
        right: Val::Px(4.0),
        ..default()
    }));
}
