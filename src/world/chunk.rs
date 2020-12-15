use crate::world::block::{Block, AIR, GROUND};
use crate::world::chunk_mesh::create_chunk_mesh;

use std::mem::replace;
use std::time::Instant;
use std::ops::{Add, Index, IndexMut};
use itertools::__std_iter::repeat;
use std::collections::HashMap;

use bevy::prelude::*;


#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ChunkPosition {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl ChunkPosition {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        ChunkPosition{
            x,
            y,
            z,
        }
    }
    pub fn x(&self, x: i64) -> Self {
        let mut new = *self;
        new.x += x;
        new
    }
    pub fn y(&self, y: i64) -> Self {
        let mut new = *self;
        new.y += y;
        new
    }
    pub fn z(&self, z: i64) -> Self {
        let mut new = *self;
        new.z += z;
        new
    }
}

impl From<Vec3> for ChunkPosition {
    fn from(vec: Vec3) -> Self {
        ChunkPosition::new(vec.x() as i64 >> CHUNK_SIZE_EXP,
                           vec.y() as i64 >> CHUNK_SIZE_EXP,
                           vec.z() as i64 >> CHUNK_SIZE_EXP)
    }
}


/// A position relative to a chunk
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct BlockPosition {
    pub(crate) x: i64,
    pub(crate) y: i64,
    pub(crate) z: i64,
}

const CHUNK_SIZE_EXP: u32 = 4;
//Dont change auto generated!
const CHUNK_SIZE: i64 = 1 << CHUNK_SIZE_EXP;

impl BlockPosition {
    //Dont change auto generated!
    const BLOCK_BITS: i64 = CHUNK_SIZE - 1;
    //Dont change auto generated!
    const CHUNK_BITS: i64 = !Self::BLOCK_BITS;

    pub fn new(x: i64, y: i64, z: i64) -> Self {
        BlockPosition{
            x,
            y,
            z,
        }
    }
    pub fn fits(&self) -> bool {
        self.x & Self::CHUNK_BITS == 0 && self.y & Self::CHUNK_BITS == 0 && self.z & Self::CHUNK_BITS == 0
    }
    pub fn x(&self, x: i64) -> BlockPosition {
        let mut new = *self;
        new.x += x;
        new
    }
    pub fn y(&self, y: i64) -> BlockPosition {
        let mut new = *self;
        new.y += y;
        new
    }
    pub fn z(&self, z: i64) -> BlockPosition {
        let mut new = *self;
        new.z += z;
        new
    }

}

impl From<(i64, i64, i64)> for BlockPosition {
    fn from(value: (i64, i64, i64)) -> Self {
        BlockPosition::new(value.0, value.1, value.2)
    }
}

impl Add<BlockPosition> for ChunkPosition {
    type Output = Vec3;

    fn add(self, rhs: BlockPosition) -> Self::Output {
        Vec3::new((self.x << CHUNK_SIZE_EXP) as f32 + rhs.x as f32,
                  (self.y << CHUNK_SIZE_EXP) as f32 + rhs.y as f32,
                  (self.z << CHUNK_SIZE_EXP) as f32 + rhs.z as f32)
    }
}

pub struct Chunk {
    pub position: ChunkPosition,
    pub data: ChunkData,

    //Adjacent chunks
    pub x_positive: Option<Entity>,
    pub x_negative: Option<Entity>,
    pub y_positive: Option<Entity>,
    pub y_negative: Option<Entity>,
    pub z_positive: Option<Entity>,
    pub z_negative: Option<Entity>,
}

#[derive(Clone)]
pub struct ChunkData {
    pub blocks: [[[Block; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
}

impl ChunkData {
    pub fn new(data: [[[Block; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]) -> Self {
        Self{
            blocks: data,
        }
    }
    pub fn filled(block: Block) -> Self {
        Self::new([[[block; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize])
    }
    pub fn get(&self, pos: BlockPosition) -> Option<&Block> {
        if pos.fits() {
            unsafe {
                Some(self.get_unchecked(pos))
            }
        } else {
            None
        }
    }
    pub unsafe fn get_unchecked(&self, pos: BlockPosition) -> &Block {
        self.blocks.get_unchecked(pos.x as usize)
            .get_unchecked(pos.y as usize)
            .get_unchecked(pos.z as usize)
    }
    pub fn get_mut(&mut self, pos: BlockPosition) -> Option<&mut Block> {
        if pos.fits() {
            unsafe {
                Some(self.get_unchecked_mut(pos))
            }
        } else {
            None
        }
    }
    pub unsafe fn get_unchecked_mut(&mut self, pos: BlockPosition) -> &mut Block {
        self.blocks.get_unchecked_mut(pos.x as usize)
            .get_unchecked_mut(pos.y as usize)
            .get_unchecked_mut(pos.z as usize)
    }
    pub fn clear(&mut self, pos: BlockPosition) -> Block {
        replace(&mut self.get_mut(pos).unwrap(), AIR)
    }
    pub fn iter(&self) -> impl Iterator<Item = (BlockPosition, &Block)> {
        self.blocks.iter().enumerate()
            .flat_map(|(x, items)|items.iter().enumerate().zip(repeat(x)))
            .flat_map(|((y, items), x)|items.iter().enumerate().zip(repeat((x, y))))
            .map(|((z, block), (x, y))|(BlockPosition::new(x as i64, y as i64, z as i64), block))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (BlockPosition, &mut Block)> {
        self.blocks.iter_mut().enumerate()
            .flat_map(|(x, items)|items.iter_mut().enumerate().zip(repeat(x)))
            .flat_map(|((y, items), x)|items.iter_mut().enumerate().zip(repeat((x, y))))
            .map(|((z, block), (x, y))|(BlockPosition::new(x as i64, y as i64, z as i64), block))
    }
}

impl<T: Into<BlockPosition>> Index<T> for ChunkData {
    type Output = Block;

    fn index(&self, index: T) -> &Self::Output {
        let pos = index.into();
        assert!(pos.fits(), "invalid Blockposition");
        unsafe{
            self.get_unchecked(pos)
        }
    }
}

impl<T: Into<BlockPosition>> IndexMut<T> for ChunkData {
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        let pos = index.into();
        assert!(pos.fits(), "invalid Blockposition");
        unsafe{
            self.get_unchecked_mut(pos)
        }
    }
}

fn chunk_loader(position: ChunkPosition) -> ChunkData {
    if position.y > 0 {
        ChunkData::filled(AIR)
    } else {
        ChunkData::filled(GROUND)
    }
}

pub struct ChunkRegistry {
    chunks: HashMap<ChunkPosition, Entity>,
    player_chunk: ChunkPosition,
    chunk_loading_distance: f32,
    chunk_discard_distance: f32,
    texture_atlas: Option<Handle<StandardMaterial>>,
}

impl ChunkRegistry {
    pub fn new(current_position: ChunkPosition, chunk_loading_distance: u32, chunk_discard_distance: u32) -> Self {
        Self {
            chunks: HashMap::new(),
            player_chunk: current_position,
            chunk_loading_distance: chunk_loading_distance as f32,
            chunk_discard_distance: chunk_discard_distance as f32,
            texture_atlas: None,
        }
    }
    fn init(&mut self, commands: &mut Commands,
            server: Res<AssetServer>,
            mut materials: ResMut<Assets<StandardMaterial>>,
            mut meshes: ResMut<Assets<Mesh>>,
    ) {
        println!("init registry!");

        let texture: Handle<Texture> = server.load("textures.png");
        let material = StandardMaterial{
            albedo: Default::default(),
            albedo_texture: Some(texture),
            shaded: false
        };

        let material_handle = materials.add(material);

        self.texture_atlas = Some(material_handle);

        self.load_chunk(commands, &mut meshes, ChunkPosition::new(0, 0, 0), true);
        self.load_chunk(commands, &mut meshes, ChunkPosition::new(1, 0, 0), true);
        self.load_chunk(commands, &mut meshes, ChunkPosition::new(0, 0, 1), true);
        self.load_chunk(commands, &mut meshes, ChunkPosition::new(1, 0, 1), true);
    }
    fn load_chunk(&mut self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, chunk_position: ChunkPosition, rendered: bool) {
        let chunk_data = chunk_loader(chunk_position);

        let chunk = Chunk {
            position: chunk_position,
            data: chunk_data,

            x_positive: self.chunks.get(&chunk_position.x( 1)).cloned(),
            x_negative: self.chunks.get(&chunk_position.x(-1)).cloned(),
            y_positive: self.chunks.get(&chunk_position.y( 1)).cloned(),
            y_negative: self.chunks.get(&chunk_position.y(-1)).cloned(),
            z_positive: self.chunks.get(&chunk_position.z( 1)).cloned(),
            z_negative: self.chunks.get(&chunk_position.z(-1)).cloned(),
        };

        let entity = if rendered {
            let start = Instant::now();
            let mesh = meshes.add(create_chunk_mesh(&chunk));
            println!("created mesh in {} sec. !", start.elapsed().as_secs_f32());

            let handle = self.texture_atlas.clone().unwrap();

            println!("spawn cube mesh!");
            commands
                .spawn(PbrComponents {
                    mesh,
                    material: handle,
                    ..Default::default()
                })
                .with(chunk);
            commands.current_entity().unwrap()//We just inserted one, this should be ok
        } else {

            commands.spawn((chunk,));
            commands.current_entity().unwrap()
        };

        self.chunks.insert(chunk_position, entity);
    }
    pub fn update(&mut self, commands: &mut Commands, meshes: ResMut<Assets<Mesh>>, new_position: ChunkPosition) {

    }
}

//TODO: mut 'commands: Commands' only works on 0.3 (change to 'commands: &mut Commands' otherwise)!
pub fn init_chunks(mut commands: Commands,
                   mut registry: ResMut<ChunkRegistry>,
                   resources: Res<AssetServer>,
                   mut textures: ResMut<Assets<StandardMaterial>>,
                   mut meshes: ResMut<Assets<Mesh>>) {
    registry.init(&mut commands, resources, textures, meshes);
}

//TODO: mut 'commands: Commands' only works on 0.3 (change to 'commands: &mut Commands' otherwise)!
pub fn update_chunks(mut commands: Commands,
                     mut registry: ResMut<ChunkRegistry>,
                     mut meshes: ResMut<Assets<Mesh>>) {
    registry.update(&mut commands, meshes, ChunkPosition::new(0, 1, 0));
}