use crate::world::coordinates::{ChunkPosition, BlockPosition};
use crate::world::chunk::ChunkData;
use crate::world::block_inner::BlockInner;

pub enum ChunkUpdate {
    BlockUpdate(BlockPosition, BlockInner),
}

impl ChunkUpdate {
    pub fn position(&self) -> ChunkPosition {
        match self {
            Self::BlockUpdate(position, _) => {position.chunk()}
        }
    }
}

/// Provider loads the world and entities. It also updates the data on changes and manages the owner
/// client relation.
///
/// A single-player world or a server-client might be a provider for the game data.
pub trait Provider {
    fn load_chunk(&mut self, position: ChunkPosition) -> ChunkData;
    fn get_chunk_update(&mut self) -> Option<ChunkUpdate>;
    fn apply_chunk_update(&mut self, update: ChunkUpdate);
}