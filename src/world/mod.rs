use bevy::prelude::*;
use chunk::{ChunkRegistry, ChunkPosition, init_chunks, update_chunks};
use crate::settings::Settings;

pub mod chunk;
pub mod block;
pub mod chunk_mesh;

pub fn init_world(builder: &mut AppBuilder, settings: &Settings) {
    builder.add_resource(ChunkRegistry::new(
            ChunkPosition::new(0, 1, 0),
            2,
            4,
        ))
        .add_startup_system(init_chunks.system())
        .add_system(update_chunks.system());
}