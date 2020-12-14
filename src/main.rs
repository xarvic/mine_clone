#![allow(unused_variables)]
#![allow(dead_code)]

use crate::engine::load_engine;
use crate::world::block::FULL;

mod engine;
mod renderer;
mod world;
mod player;
mod controller;
mod util;

fn run() {
    let engine = load_engine();

    engine.run();
}

fn main() {
    run()
}
