use bevy::prelude::*;
use chunk::{init_chunks};
use crate::settings::Settings;
use crate::world::coordinates::ChunkPosition;
use crate::world::chunk::{ChunkManager, update_chunk_mesh, update_chunk_scope};
use bevy::prelude::stage::POST_UPDATE;

pub mod chunk;
pub mod block_inner;
pub mod chunk_mesh;
pub mod coordinates;
mod chunk_manager;
pub mod block;

pub fn init_world(builder: &mut AppBuilder, settings: &Settings) {
    builder.add_resource(ChunkManager::new(
            ChunkPosition::new(0, 1, 0),
            settings.game_settings.load_distance,
            settings.game_settings.unload_distance,
        ))
        .add_startup_system(init_chunks.system())
        .add_system_to_stage(POST_UPDATE, update_chunk_mesh.system())
        .add_system(update_chunk_scope.system());
}