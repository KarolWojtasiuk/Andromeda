#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::prelude::*;

mod engine;
mod game;

fn main() -> AppExit {
    game::run()
}
