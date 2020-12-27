use crate::content::provider::{Provider, ChunkUpdate};
use crate::world::chunk::ChunkData;
use crate::world::coordinates::ChunkPosition;
use crate::content::world_generation::generate_chunk;
use rand::RngCore;

/// In memory is a singleplayer provider which doesnt save anything!
pub struct InMemory {
    seed: u64,
}

impl InMemory {
    pub fn new() -> Self {
        InMemory {
            seed: rand::thread_rng().next_u64()
        }
    }
    pub fn with_seed(seed: u64) -> Self {
        InMemory {
            seed,
        }
    }
}

impl Provider for InMemory {
    fn load_chunk(&mut self, position: ChunkPosition) -> ChunkData {
        generate_chunk(position)
    }

    fn get_chunk_update(&mut self) -> Option<ChunkUpdate> {
        //No one else can update the world!
        None
    }

    fn apply_chunk_update(&mut self, _update: ChunkUpdate) {
        println!("send update!");
        //We dont safe anything: do nothing
    }
}