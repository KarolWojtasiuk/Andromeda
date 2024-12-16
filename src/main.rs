#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![feature(iter_array_chunks)]

use bevy::prelude::*;

mod engine;
mod game;

fn main() -> AppExit {
    game::run()
}
