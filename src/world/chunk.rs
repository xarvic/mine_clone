use crate::world::block::{Block, AIR};
use std::mem::replace;
use std::ops::{Add, Index, IndexMut};
use bevy::prelude::Vec3;
use itertools::__std_iter::repeat;

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
}



/// A position relative to a chunk
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct BlockPosition {
    pub(crate) x: i64,
    pub(crate) y: i64,
    pub(crate) z: i64,
}

impl BlockPosition {
    const BLOCK_BITS: i64 = 15;
    const CHUNK_BITS: i64 = !15;

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
        Vec3::new((self.x * 16) as f32 + rhs.x as f32, (self.y * 16) as f32 + rhs.y as f32, (self.z * 16) as f32 + rhs.z as f32)
    }
}

#[derive(Clone)]
pub struct Chunk{
    pub position: ChunkPosition,
    pub data: [[[Block; 16]; 16]; 16],
    pub mesh_update: bool,
}

impl Chunk{
    pub fn new(data: [[[Block; 16]; 16]; 16], position: ChunkPosition) -> Self {
        Self{
            position,
            data,
            mesh_update: true,
        }
    }
    pub fn filled(block: Block, position: impl Into<ChunkPosition>) -> Self {
        Self::new([[[block; 16]; 16]; 16], position.into())
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
        self.data.get_unchecked(pos.x as usize)
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
        self.data.get_unchecked_mut(pos.x as usize)
            .get_unchecked_mut(pos.y as usize)
            .get_unchecked_mut(pos.z as usize)
    }
    pub fn clear(&mut self, pos: BlockPosition) -> Block {
        replace(&mut self.get_mut(pos).unwrap(), AIR)
    }
    pub fn iter(&self) -> impl Iterator<Item = (BlockPosition, &Block)> {
        self.data.iter().enumerate()
            .flat_map(|(x, items)|items.iter().enumerate().zip(repeat(x)))
            .flat_map(|((y, items), x)|items.iter().enumerate().zip(repeat((x, y))))
            .map(|((z, block), (x, y))|(BlockPosition::new(x as i64, y as i64, z as i64), block))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (BlockPosition, &mut Block)> {
        self.data.iter_mut().enumerate()
            .flat_map(|(x, items)|items.iter_mut().enumerate().zip(repeat(x)))
            .flat_map(|((y, items), x)|items.iter_mut().enumerate().zip(repeat((x, y))))
            .map(|((z, block), (x, y))|(BlockPosition::new(x as i64, y as i64, z as i64), block))
    }
}

impl<T: Into<BlockPosition>> Index<T> for Chunk {
    type Output = Block;

    fn index(&self, index: T) -> &Self::Output {
        let pos = index.into();
        assert!(pos.fits(), "invalid Blockposition");
        unsafe{
            self.get_unchecked(pos)
        }
    }
}

impl<T: Into<BlockPosition>> IndexMut<T> for Chunk {
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        let pos = index.into();
        assert!(pos.fits(), "invalid Blockposition");
        unsafe{
            self.get_unchecked_mut(pos)
        }
    }
}