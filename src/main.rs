#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

#![feature(with_options, const_fn)]

use crate::engine::load_engine;
use anyhow::Result;
use crate::settings::Settings;
use std::io::{BufReader, BufWriter};
use std::fs::File;

#[macro_use] extern crate impl_ops;

pub mod engine;
pub mod renderer;
pub mod world;
pub mod player;
pub mod controller;
pub mod util;
pub mod settings;
pub mod physics;
pub mod entities;

fn load_settings() -> Result<Settings> {
    Ok(match serde_json::from_reader(BufReader::new(File::open("./settings.json")?)) {
        Ok(settings) => settings,
        Err(error) => {
            eprintln!("could`nt load settings: {}", error.to_string());
            let settings = Settings::default();

            serde_json::to_writer_pretty(
                BufWriter::new(File::with_options()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open("./settings.json")?),
                &settings,
            )?;

            settings
        }
    })
}

fn main() -> Result<()>{
    let settings = load_settings()?;

    let engine = load_engine(settings);

    engine.run();

    Ok(())
}
