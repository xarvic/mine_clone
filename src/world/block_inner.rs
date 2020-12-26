use std::ops::{Range, Index};
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

pub const AIR: BlockInner = BlockInner{btype: 0, data: 0, info: EMPTY};
pub const DIRT: BlockInner = BlockInner{btype: 2, data: 0, info: BLOCK_MESH};
pub const GRASS: BlockInner = BlockInner{btype: 3, data: 0, info: BLOCK_MESH};
pub const STONE: BlockInner = BlockInner{btype: 1, data: 0, info: BLOCK_MESH};
pub const WOOD: BlockInner = BlockInner{btype: 4, data: 0, info: BLOCK_MESH};
pub const LOG: BlockInner = BlockInner{btype: 5, data: 0, info: BLOCK_MESH};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BlockInner {
    pub btype: u16,
    pub data: u8,
    pub info: BlockInfo,
}

struct Faces([Range<usize>;6]);

#[repr(C)]
pub enum Side {
    Top = 0,
    Front = 1,
    Back = 2,
    Left = 3,
    Right = 4,
    Bottom = 5,
}

#[derive(Copy, Clone)]
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