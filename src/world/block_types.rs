use bevy::prelude::*;

use crate::world::block_inner::{Sides, BlockInfo, EMPTY, BLOCK_MESH};
use crate::world::chunk_mesh::{VisibleDirection, Face};
use crate::physics::collider::AAQuader;


pub type StaticBlocks = [(BlockLook, BlockFeel, Box<dyn BlockPersonality + Send + Sync>)];

pub type StaticBlocksRes = Vec<(BlockLook, BlockFeel, Box<dyn BlockPersonality + Send + Sync>)>;

pub fn get_block_types() -> StaticBlocksRes {
    let mut block_types: StaticBlocksRes = vec![
        (BlockLook::Empty, BlockFeel::Empty, Box::new(Air)),
        (BlockLook::Empty, BlockFeel::Empty, Box::new(Cube::uniform("stone", 1))),
        (BlockLook::Empty, BlockFeel::Empty, Box::new(Cube::uniform("dirt", 2))),
        (BlockLook::Empty, BlockFeel::Empty, Box::new(Cube::top_side_bottom("grass", 0, 3, 2))),
        (BlockLook::Empty, BlockFeel::Empty, Box::new(Cube::top_side_bottom("log", 21, 20, 21))),
        (BlockLook::Empty, BlockFeel::Empty, Box::new(Cube::uniform("wood", 4))),
    ];

    block_types.iter_mut().for_each(|(look, feel, block)|{
        *look = block.get_block_look();
        *feel = block.get_feel();
    });

    block_types
}

pub enum BlockLook {
    Empty,
    DynamicBlockMesh,
    CustomMesh,
    Cube{textures: Sides<u32>},
}

pub enum BlockFeel {
    Empty,
    ColliderSet(&'static [AAQuader]),
    Custom,
}

/// The look and feel of a static block (only described by its id and meta fields)
pub trait BlockPersonality {
    /// The look type for caching, so we can optimise for the most common types (Cube and Empty)
    /// Every other type can determin the structure
    fn get_block_look(&self) -> BlockLook;
    fn name(&self) -> &'static str;

    fn info(&self, data: u8) -> BlockInfo;
    fn get_faces(&self, data: u8) -> &[(Face, VisibleDirection)];
    fn get_mesh(&self, data: u8) -> Option<(Handle<Mesh>, Handle<StandardMaterial>)>;

    fn get_feel(&self) -> BlockFeel;
    fn get_collider(&self, data: u8) -> &[AAQuader];

}

pub struct Air;

impl BlockPersonality for Air {
    fn get_block_look(&self) -> BlockLook {
        BlockLook::Empty
    }

    fn name(&self) -> &'static str {
        "air"
    }

    fn info(&self, data: u8) -> BlockInfo {
        EMPTY
    }

    fn get_faces(&self, data: u8) -> &[(Face, VisibleDirection)] {
        &[]
    }

    fn get_mesh(&self, data: u8) -> Option<(Handle<Mesh>, Handle<StandardMaterial>)> {
        None
    }

    fn get_feel(&self) -> BlockFeel {
        BlockFeel::Empty
    }

    fn get_collider(&self, data: u8) -> &[AAQuader] {
        &[]
    }
}



pub struct Cube{
    textures: Sides<u32>,
    name: &'static str,
}

impl Cube {
    pub const fn uniform(name: &'static str, texture: u32) -> Self {
        Self {name, textures: Sides::filled(texture)}
    }
    pub const fn top_side_bottom(name: &'static str, top: u32, side: u32, bottom: u32) -> Self {
        Self {name, textures: Sides::new([top, side, side, side, side, bottom])}
    }
    pub fn new(name: &'static str, textures: Sides<u32>) -> Self {
        Self {name, textures}
    }
}

static QUADER_COLLIDERS: &'static [AAQuader] = &[AAQuader::new(Vec3{x:0.0, y:0.0, z:0.0}, Vec3{x:1.0, y:1.0, z:1.0})];

impl BlockPersonality for Cube{
    fn get_block_look(&self) -> BlockLook {
        BlockLook::Cube { textures: self.textures }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn info(&self, data: u8) -> BlockInfo {
        BLOCK_MESH
    }

    fn get_faces(&self, data: u8) -> &[(Face, VisibleDirection)] {
        unimplemented!()
    }

    fn get_mesh(&self, data: u8) -> Option<(Handle<Mesh>, Handle<StandardMaterial>)> {
        None
    }

    fn get_feel(&self) -> BlockFeel {

        BlockFeel::ColliderSet(QUADER_COLLIDERS)
    }

    fn get_collider(&self, data: u8) -> &[AAQuader] {
        QUADER_COLLIDERS
    }
}