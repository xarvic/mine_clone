#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

use crate::engine::load_engine;

mod engine;
mod renderer;
mod world;
mod player;
mod controller;
mod util;

fn main() {
    let engine = load_engine();

    engine.run();
}
