use bevy::prelude::*;
use chunk::{init_chunks};
use crate::settings::Settings;
use crate::world::coordinates::ChunkPosition;
use crate::world::chunk::{ChunkManager, update_chunk_mesh};

pub mod chunk;
pub mod block;
pub mod chunk_mesh;
pub mod coordinates;

pub fn init_world(builder: &mut AppBuilder, settings: &Settings) {
    builder.add_resource(ChunkManager::new(
            ChunkPosition::new(0, 1, 0),
            2,
            4,
        ))
        .add_startup_system(init_chunks.system())
        .add_system(update_chunk_mesh.system());

}