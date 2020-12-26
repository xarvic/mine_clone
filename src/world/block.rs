use crate::world::chunk::Chunk;
use crate::world::coordinates::BlockVector;

enum BoxedBlock {
    Simple{id: u16, data: u8},
    Custom{behaviour: Box<dyn Block>, data: u8},
}

trait Block {

}

struct Slot {
    current: *mut dyn Block,
    chunk: *mut Chunk,
    position: BlockVector,
}

impl Slot {
    pub fn new(&mut self, chunk: &mut Chunk, position: BlockVector) -> Self {

    }
    pub fn replace(&mut self, new: &dyn Block) {

    }
}