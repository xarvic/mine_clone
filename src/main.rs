#![allow(unused_variables)]
#![allow(dead_code)]

use crate::engine::load_engine;

mod engine;
mod renderer;
mod world;
mod player;
mod controller;

fn main() {
    let engine = load_engine();

    engine.run();
}
