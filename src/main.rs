#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

#![feature(with_options, const_fn, const_fn_floating_point_arithmetic, const_panic)]

use crate::engine::load_engine;
use anyhow::Result;
use crate::settings::Settings;
use std::io::{BufReader, BufWriter};
use std::fs::File;
use crate::physics::collider::AAQuader;
use bevy::math::Vec3;

#[macro_use] extern crate impl_ops;

pub mod engine;
pub mod renderer;
pub mod world;
pub mod player;
pub mod controller;
pub mod util;
pub mod settings;
pub mod entities;
pub mod content;
pub mod physics;

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
    /*let settings = load_settings()?;

    let engine = load_engine(settings);

    engine.run();
*/

    let box1 = AAQuader::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 2.0, 2.0));
    let box2 = AAQuader::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(3.0, 3.0, 3.0));

    println!("impact: {}", box1.impact_volume(box2));

    Ok(())
}
