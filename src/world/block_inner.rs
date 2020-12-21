use std::ops::{Range, Index};
use bevy::asset::Handle;
use bevy::render::mesh::Mesh;
use std::fmt::Debug;
use std::ops::BitOr;
use std::fmt::Formatter;
use std::ops::IndexMut;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlockInfo(u8);

pub const EMPTY: BlockInfo = BlockInfo(0);
pub const POWERED: BlockInfo = BlockInfo(1);
pub const BLOCK_MESH: BlockInfo = BlockInfo(1 << 1);

impl BlockInfo {
    pub fn contains(self, other: BlockInfo) -> bool {
        (self.0 & other.0) == other.0
    }
    pub fn contains_any(self, other: BlockInfo) -> bool {
        (self.0 & other.0) != 0
    }
}

impl Debug for BlockInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut found = false;
        if self.contains(POWERED) {
            found = true;
            f.write_str("POWERED")?;
        }
        if self.contains(BLOCK_MESH) {
            if found {
                f.write_str(" | ")?;
            }
            found = true;
            f.write_str("BLOCK_MESH")?;
        }

        if !found {
            f.write_str("EMPTY")
        } else {
            Ok(())
        }
    }
}

impl BitOr for BlockInfo {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        BlockInfo(self.0 | rhs.0)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BlockInner {
    pub btype: u16,
    pub data: u8,
    pub info: BlockInfo,
}

pub static AIR: BlockInner = BlockInner {btype: 0, data: 0, info: EMPTY};
pub static GRASS: BlockInner = BlockInner {btype: 1, data: 0, info: BLOCK_MESH };
pub static DIRT: BlockInner = BlockInner {btype: 2, data: 0, info: BLOCK_MESH };
pub static STONE: BlockInner = BlockInner {btype: 3, data: 0, info: BLOCK_MESH };
pub static LOG: BlockInner = BlockInner {btype: 3, data: 0, info: BLOCK_MESH };

pub static TEXTURES: &'static [BlockLook] = &[
    BlockLook::Empty,
    BlockLook::top_side_bottom(0, 3, 2),
    BlockLook::uniform(2),
    BlockLook::uniform(1),
    BlockLook::top_side_bottom(21, 20, 21),
];

struct TexCoords;

struct Faces([Range<usize>;6]);

enum Look<'a> {
    Block{faces: &'a Faces, translucent: bool},
    Mesh{meshes: &'a [(Handle<Mesh>, TexCoords)], translucent: bool},
}

#[repr(C)]
pub enum Side {
    Top = 0,
    Front = 1,
    Back = 2,
    Left = 3,
    Right = 4,
    Bottom = 5,
}

pub struct Sides<T> {
    values: [T; 6],
}

impl<T> Sides<T> {
    pub const fn filled(value: T) -> Self where T: Copy {
        Sides {
            values: [value; 6]
        }
    }
    pub const fn new(values: [T; 6]) -> Self {
        Sides{
            values
        }
    }
    pub fn inner(self) -> [T; 6] {
        self.values
    }
}

impl<T: Default> Default for Sides<T> {
    fn default() -> Self {
        Self::new([T::default(), T::default(), T::default(), T::default(), T::default(), T::default()])
    }
}

impl<T> Index<Side> for Sides<T> {
    type Output = T;

    fn index(&self, index: Side) -> &Self::Output {
        unsafe {
            self.values.get_unchecked(index as usize)
        }
    }
}

impl<T> IndexMut<Side> for Sides<T> {
    fn index_mut(&mut self, index: Side) -> &mut Self::Output {
        unsafe {
            self.values.get_unchecked_mut(index as usize)
        }
    }
}

struct BlockMeta {
    pub look: BlockLook,
}

pub enum BlockLook {
    Empty,
    Faces{textures: Sides<u32>},
}

impl BlockLook {
    pub const fn uniform(texture: u32) -> Self {
        BlockLook::Faces {textures: Sides::filled(texture)}
    }
    pub const fn top_side_bottom(top: u32, side: u32, bottom: u32) -> Self {
        BlockLook::Faces {textures: Sides::new([top, side, side, side, side, bottom])}
    }
}

/// The block and feel of a static block (only described by its id and meta fields)
trait BlockPersonality {
    fn info(&self, data: u8) -> BlockInfo;

    fn block_tick(&self);
}