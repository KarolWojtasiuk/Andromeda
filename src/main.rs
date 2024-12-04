#![allow(clippy::type_complexity)]

use engine::camera::{GameCamera, GameCameraTarget};
use engine::player::Player;
use engine::prelude::*;
use engine::{GameInfo, create_app};

mod engine;

fn main() {
    let mut app = create_app(GameInfo {
        name: env!("CARGO_PKG_NAME"),
        version: Some(env!("CARGO_PKG_VERSION")),
    });

    app.add_systems(Startup, setup);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(GameCamera);
    commands.spawn((Player, GameCameraTarget, Sprite {
        color: Color::linear_rgb(1.0, 0.75, 0.0),
        custom_size: Some(Vec2::new(50.0, 50.0)),
        ..default()
    }));

    commands.spawn((Transform::from_xyz(-100.0, 0.0, 0.0), Sprite {
        color: Color::linear_rgb(1.0, 0.0, 0.0),
        custom_size: Some(Vec2::new(25.0, 25.0)),
        ..default()
    }));
    commands.spawn((Transform::from_xyz(50.0, 250.0, 0.0), Sprite {
        color: Color::linear_rgb(1.0, 0.0, 0.0),
        custom_size: Some(Vec2::new(25.0, 25.0)),
        ..default()
    }));
}
