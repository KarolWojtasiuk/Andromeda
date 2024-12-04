pub mod prelude {
    pub use bevy::prelude::*;
}

pub mod camera;
pub mod input;
pub mod player;

use camera::GameCameraPlugin;
use clap::{ArgAction, Parser};
use input::GameInputPlugin;
use player::PlayerPlugin;
use prelude::*;

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

    app.add_plugins((GameInputPlugin, GameCameraPlugin, PlayerPlugin));

    if args.show_game_info_overlay {
        app.add_systems(Startup, spawn_info_overlay);
    }

    app
}

#[derive(Parser)]
#[command(version)]
struct EngineArgs {
    #[arg(
        short = 'i',
        long = "info-overlay",
        help = "Show game info overlay",
        action = ArgAction::Set,
        default_value_t = true,
    )]
    pub show_game_info_overlay: bool,
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
