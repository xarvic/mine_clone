use std::ops::Range;
use bevy::asset::Handle;
use bevy::render::mesh::Mesh;
use std::fmt::Debug;
use std::ops::BitOr;
use std::fmt::Formatter;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlockInfo(u8);

pub const EMPTY: BlockInfo = BlockInfo(0);
pub const POWERED: BlockInfo = BlockInfo(1);
pub const BLOCK_MESH: BlockInfo = BlockInfo(1 << 1);

impl BlockInfo {
    pub fn contains(&self, other: BlockInfo) -> bool {
        (self.0 & other.0) == other.0
    }
    pub fn contains_any(&self, other: BlockInfo) -> bool {
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
        if self.contains(POWERED) {
            if found {
                f.write_str(" | ")?;
            }
            found = true;
            f.write_str("POWERED")?;
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
pub struct Block {
    pub btype: u16,
    pub data: u8,
    pub info: BlockInfo,
}

pub static AIR: Block = Block{btype: 0, data: 0, info: EMPTY};
pub static GROUND: Block = Block{btype: 1, data: 0, info: BLOCK_MESH };

struct TexCoords;

struct Faces([Range<usize>;6]);

enum Look<'a> {
    Block{faces: &'a Faces, translucent: bool},
    Mesh{meshes: &'a [(Handle<Mesh>, TexCoords)], translucent: bool},
}

trait BlockRepr {
    fn look(&self) -> Look;
}