#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

#![feature(with_options)]

use crate::engine::load_engine;
use anyhow::Result;
use crate::settings::Settings;
use std::io::{BufReader, BufWriter};
use std::fs::File;

mod engine;
mod renderer;
mod world;
mod player;
mod controller;
mod util;
mod settings;
mod physics;

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
