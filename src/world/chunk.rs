use crate::world::block::Block;

struct ChunkPosition {
    x: u64,
    y: u64,
    z: u64,
}

impl ChunkPosition {

}

struct Chunk{
    position: ChunkPosition,
    data: [[[Block; 16]; 16]; 16],
}

impl Chunk{
    pub fn new(data: [[[Block; 16]; 16]; 16], position: ChunkPosition) -> Self {
        Self{
            position,
            data,
        }
    }
    pub fn filled(block: Block, position: impl Into<ChunkPosition>) -> Self {
        Self::new([[[block; 16]; 16]; 16], position.into())
    }

}