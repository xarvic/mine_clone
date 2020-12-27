use crate::world::coordinates::{ChunkPosition, CHUNK_SIZE};
use crate::world::chunk::ChunkData;
use crate::world::block_inner::{AIR, GRASS, DIRT, STONE};
use utils::{create_perlin_noise, write_perlin_noise};

pub fn generate_chunk(position: ChunkPosition) -> ChunkData {
    if position.y != 0 {
        ChunkData::filled(AIR)
    } else {

        let mut noise = create_perlin_noise(264958643553465476, position.x * CHUNK_SIZE, position.z * CHUNK_SIZE, 64, 0.0..=8.0);
        write_perlin_noise(&mut noise, 264958643553465476, position.x * CHUNK_SIZE, position.z * CHUNK_SIZE, 16, 0.0..=2.0);
        write_perlin_noise(&mut noise, 264958643553465476, position.x * CHUNK_SIZE, position.z * CHUNK_SIZE, 4, 0.0..=0.8);
        write_perlin_noise(&mut noise, 264958643553465476, position.x * CHUNK_SIZE, position.z * CHUNK_SIZE, 2, 0.0..=0.2);

        let mut chunk = ChunkData::filled(AIR);
        for (position , block) in chunk.iter_mut() {
            if noise[position.x as usize][position.z as usize] >= position.y as f32 - 0.5 {
                *block = if position.y == 15 {
                    GRASS
                } else if position.y > 9 {
                    DIRT
                } else {
                    STONE
                };
            }
        }
        chunk
    }
}