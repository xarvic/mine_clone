use bevy::math::Vec3;
use std::ops::{self, Add, Sub, Neg, AddAssign, SubAssign};
use std::fmt::Display;
use std::fmt::Formatter;

/// A relative position of a block, (mostly relative to a chunk)
/// This is useful to iterate over a chunk, independent from its position
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct BlockVector {
    pub(crate) x: i64,
    pub(crate) y: i64,
    pub(crate) z: i64,
}

impl Display for BlockVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("<")?;
        self.x.fmt(f)?;
        f.write_str(" | ")?;
        self.y.fmt(f)?;
        f.write_str(" | ")?;
        self.z.fmt(f)?;
        f.write_str(">")
    }
}

pub const CHUNK_SIZE_EXP: u32 = 4;
//Dont change auto generated!
pub const CHUNK_SIZE: i64 = 1 << CHUNK_SIZE_EXP;

//Dont change auto generated!
pub const BLOCK_BITS: i64 = CHUNK_SIZE - 1;
//Dont change auto generated!
pub const CHUNK_BITS: i64 = !BLOCK_BITS;

pub const MAX_CHILD: i64 = BLOCK_BITS;

pub static ADJACENT_POSITIONS: [BlockVector; 6] = [
    BlockVector::new(1, 0, 0),
    BlockVector::new(0, 1, 0),
    BlockVector::new(0, 0, 1),
    BlockVector::new(-1, 0, 0),
    BlockVector::new(0, -1, 0),
    BlockVector::new(0, 0, -1),
];

impl BlockVector {

    pub const fn new(x: i64, y: i64, z: i64) -> Self {
        BlockVector {
            x,
            y,
            z,
        }
    }
    pub const fn fits(&self) -> bool {
        self.x & CHUNK_BITS == 0 && self.y & CHUNK_BITS == 0 && self.z & CHUNK_BITS == 0
    }
    /// adds the given value the the x coordinate of this vector
    pub const fn with_x(&self, x: i64) -> BlockVector {
        let mut new = *self;
        new.x += x;
        new
    }
    /// adds the given value the the y coordinate of this vector
    pub const fn with_y(&self, y: i64) -> BlockVector {
        let mut new = *self;
        new.y += y;
        new
    }
    /// adds the given value the the z coordinate of this vector
    pub const fn with_z(&self, z: i64) -> BlockVector {
        let mut new = *self;
        new.z += z;
        new
    }
    ///
    pub const fn global(&self) -> BlockPosition {
        BlockPosition(*self)
    }

    pub const fn chunk_relative(&self) -> BlockVector {
        BlockVector::new(self.x & BLOCK_BITS, self.y & BLOCK_BITS, self.z & BLOCK_BITS)
    }

    pub fn distance(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, self.y as f32)
    }

    pub fn adjacent(&self) -> impl Iterator<Item=BlockVector> {
        let this = *self;
        ADJACENT_POSITIONS.iter().map(move|ad|ad + this)
    }
}

impl_op_ex!(+ |a: &BlockVector, b: &BlockVector| -> BlockVector {
    BlockVector::new(a.x + b.x, a.y + b.y, a.z + b.z)
});

impl AddAssign<BlockVector> for BlockVector {
    fn add_assign(&mut self, rhs: BlockVector) {
        *self = *self + rhs;
    }
}

impl AddAssign<&BlockVector> for BlockVector {
    fn add_assign(&mut self, rhs: &BlockVector) {
        *self = *self + rhs;
    }
}

impl_op_ex!(- |a: &BlockVector, b: &BlockVector| -> BlockVector {
    BlockVector::new(a.x - b.x, a.y - b.y, a.z - b.z)
});

impl SubAssign<BlockVector> for BlockVector {
    fn sub_assign(&mut self, rhs: BlockVector) {
        *self = *self - rhs;
    }
}

impl SubAssign<&BlockVector> for BlockVector {
    fn sub_assign(&mut self, rhs: &BlockVector) {
        *self = *self - rhs;
    }
}

impl_op_ex_commutative!(* |a: &BlockVector, b: &i64| -> BlockVector {
    BlockVector::new(a.x * b, a.y * b, a.z * b)
});

impl Neg for BlockVector {
    type Output = BlockVector;

    fn neg(self) -> Self {
        self * -1
    }
}

///The absolute position of a Block
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct BlockPosition (BlockVector);

impl Display for BlockPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl BlockPosition {
    pub const fn new(x: i64, y: i64, z: i64) -> Self {
        Self(BlockVector::new(x, y, z))
    }
    pub const fn zero() -> Self {
        Self::new(0, 0, 0)
    }
    pub fn from_vector(position: Vec3) -> Self {
        Self::new(position.x.floor() as i64, position.y.floor() as i64, position.z.floor() as i64)
    }

    pub const fn x(&self) -> i64 {
        self.0.x
    }
    pub const fn y(&self) -> i64 {
        self.0.x
    }
    pub const fn z(&self) -> i64 {
        self.0.x
    }
    ///adds the given value the the x coordinate of this vector
    pub const fn with_x(&self, x: i64) -> BlockPosition {
        Self(self.0.with_x(x))
    }
    ///adds the given value the the y coordinate of this vector
    pub const fn with_y(&self, y: i64) -> BlockPosition {
        Self(self.0.with_y(y))
    }
    ///adds the given value the the z coordinate of this vector
    pub const fn with_z(&self, z: i64) -> BlockPosition {
        Self(self.0.with_z(z))
    }

    ///the center of the block
    pub fn block_center(&self) -> Vec3 {
        Vec3::new(self.0.x as f32 + 0.5, self.0.y as f32 + 0.5, self.0.z as f32 + 0.5)
    }
    ///lower corner (each value is 0.5 lower than Self::block_center)
    pub fn lower_corner(&self) -> Vec3 {
        Vec3::new(self.0.x as f32, self.0.y as f32, self.0.z as f32)
    }
    ///higher corner (each value is 0.5 higher than Self::block_center)
    pub fn higher_corner(&self) -> Vec3 {
        Vec3::new(self.0.x as f32 + 1.0, self.0.y as f32 + 1.0, self.0.z as f32 + 1.0)
    }
    ///returns the chunk this block belongs to!
    pub const fn chunk(&self) -> ChunkPosition {
        ChunkPosition::new(self.0.x >> CHUNK_SIZE_EXP, self.0.y >> CHUNK_SIZE_EXP, self.0.z >> CHUNK_SIZE_EXP)
    }
    pub fn adjacent(&self) -> impl Iterator<Item = BlockPosition> {
        self.0.adjacent().map(|vec|Self(vec))
    }
    pub const fn chunk_relative(&self) -> BlockVector {
        self.0.chunk_relative()
    }
    pub const fn local(&self) -> (BlockVector, ChunkPosition) {
        (self.chunk_relative(), self.chunk())
    }
}

impl_op_ex_commutative!(+ |a: &BlockVector, b: &BlockPosition| -> BlockPosition {
    BlockPosition(a + b.0)
});

impl AddAssign<BlockVector> for BlockPosition {
    fn add_assign(&mut self, rhs: BlockVector) {
        *self = *self + rhs;
    }
}

impl AddAssign<&BlockVector> for BlockPosition {
    fn add_assign(&mut self, rhs: &BlockVector) {
        *self = *self + rhs;
    }
}

impl_op_ex_commutative!(- |a: &BlockVector, b: &BlockPosition| -> BlockPosition {
    BlockPosition(a - b.0)
});

impl SubAssign<BlockVector> for BlockPosition {
    fn sub_assign(&mut self, rhs: BlockVector) {
        *self = *self - rhs;
    }
}

impl SubAssign<&BlockVector> for BlockPosition {
    fn sub_assign(&mut self, rhs: &BlockVector) {
        *self = *self - rhs;
    }
}

impl_op_ex!(- |a: &BlockPosition, b: &BlockPosition| -> BlockVector {
    a.0 - b.0
});


#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ChunkPosition {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Display for ChunkPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        self.x.fmt(f)?;
        f.write_str(" | ")?;
        self.y.fmt(f)?;
        f.write_str(" | ")?;
        self.z.fmt(f)?;
        f.write_str("]")
    }
}

impl ChunkPosition {
    pub const fn new(x: i64, y: i64, z: i64) -> Self {
        ChunkPosition{
            x,
            y,
            z,
        }
    }
    pub const fn with_x(&self, x: i64) -> Self {
        let mut new = *self;
        new.x += x;
        new
    }
    pub const fn with_y(&self, y: i64) -> Self {
        let mut new = *self;
        new.y += y;
        new
    }
    pub const fn with_z(&self, z: i64) -> Self {
        let mut new = *self;
        new.z += z;
        new
    }
}

impl From<Vec3> for ChunkPosition {
    fn from(vec: Vec3) -> Self {
        ChunkPosition::new(vec.x as i64 >> CHUNK_SIZE_EXP,
                           vec.y as i64 >> CHUNK_SIZE_EXP,
                           vec.z as i64 >> CHUNK_SIZE_EXP)
    }
}

impl Add<BlockVector> for ChunkPosition {
    type Output = BlockPosition;

    fn add(self, rhs: BlockVector) -> Self::Output {
        BlockPosition::new(
            (self.x << CHUNK_SIZE_EXP) + rhs.x,
            (self.y << CHUNK_SIZE_EXP) + rhs.y,
            (self.z << CHUNK_SIZE_EXP) + rhs.z
        )
    }
}

impl Add<ChunkPosition> for BlockVector {
    type Output = BlockPosition;

    fn add(self, rhs: ChunkPosition) -> Self::Output {
        rhs.add(self)
    }
}

impl Sub<ChunkPosition> for BlockPosition {
    type Output = BlockVector;

    fn sub(self, rhs: ChunkPosition) -> Self::Output {
        BlockVector::new(
            self.x() - (rhs.x << CHUNK_SIZE_EXP),
            self.y() - (rhs.y << CHUNK_SIZE_EXP),
            self.z() - (rhs.z << CHUNK_SIZE_EXP),
        )
    }
}