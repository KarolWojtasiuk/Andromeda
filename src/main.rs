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
    commands.spawn(Camera2d);
}
