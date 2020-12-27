use bevy::render::mesh::{Mesh, Indices};
use crate::world::chunk::Chunk;
use bevy::render::pipeline::PrimitiveTopology;
use bevy::prelude::*;
use crate::world::block_inner::{Side, BLOCK_MESH};
use crate::world::coordinates::BlockVector;
use crate::world::block_types::{BlockLook, StaticBlocks};

pub const EMPTY: VisibleDirection = VisibleDirection(0);
pub const Y_POS: VisibleDirection = VisibleDirection(1);
pub const X_POS: VisibleDirection = VisibleDirection(2);
pub const Z_POS: VisibleDirection = VisibleDirection(4);
pub const X_NEG: VisibleDirection = VisibleDirection(8);
pub const Z_NEG: VisibleDirection = VisibleDirection(16);
pub const Y_NEG: VisibleDirection = VisibleDirection(32);

pub struct VisibleDirection (u8);

impl VisibleDirection {
    pub fn rotate_y_clockwise(self) -> Self {
        unimplemented!()
    }
    pub fn rotate_z_clockwise(self) -> Self {
        unimplemented!()
    }
    pub fn any(self, other: Self) -> bool {
        unimplemented!()
    }
}

pub trait ChunkMesh: Sized {
    type Builder: ChunkMeshBuilder;

    fn from_builder(builder: Self::Builder) -> Option<Self>;
}

impl ChunkMesh for Mesh {
    type Builder = BevyChunkMeshBuilder;

    fn from_builder(builder: Self::Builder) -> Option<Self> {
        if builder.indices.len() != 0 {
            let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

            mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, builder.verticies);
            mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, builder.normals);
            mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, builder.uvs);
            mesh.set_indices(Some(Indices::U16(builder.indices)));

            Some(mesh)
        } else {
            None
        }
    }
}

pub trait ChunkMeshBuilder {
    fn empty() -> Self;
    fn add_face(&mut self, face: Face, position: Vec3);
}

pub struct Face {
    normal: Vec3,
    start: Vec3,
    ax1: Vec3,
    ax2: Vec3,
    uv_resolution: u32,
    uv_index: u32,
}

impl Face{
    pub fn new(normal: Vec3,
               start: Vec3, ax1: Vec3, ax2: Vec3,
               uv_resolution: u32, uv_index: u32) -> Self {
        Face {
            normal,
            start,
            ax1,
            ax2,
            uv_resolution,
            uv_index,
        }
    }
}

pub struct BevyChunkMeshBuilder {
    verticies: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    pub(crate) indices: Vec<u16>,
}

impl ChunkMeshBuilder for BevyChunkMeshBuilder {
    fn empty() -> Self {
        BevyChunkMeshBuilder {
            verticies: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            indices: Vec::new(),
        }
    }

    fn add_face(&mut self, face: Face, position: Vec3) {
        let next_index = self.verticies.len() as u16;

        let x = (face.uv_index % face.uv_resolution) as f32;
        let y = (face.uv_index / face.uv_resolution) as f32;

        let step = 1.0 / (face.uv_resolution as f32);

        //  0    1
        //   +--+
        //   | /|
        //   |/ |
        //   +--+
        //  2    3

        //0
        self.verticies.push((face.start + position).into());
        self.normals.push(face.normal.into());
        self.uvs.push([step * x, step * y]);

        //1
        self.verticies.push((face.start + face.ax1 + position).into());
        self.normals.push(face.normal.into());
        self.uvs.push([step * (x + 1.0), step * y]);

        //2
        self.verticies.push((face.start + face.ax2 + position).into());
        self.normals.push(face.normal.into());
        self.uvs.push([step * x, step * (y + 1.0)]);

        //3
        self.verticies.push((face.start + face.ax1 + face.ax2 + position).into());
        self.normals.push(face.normal.into());
        self.uvs.push([step * (x + 1.0), step * (y + 1.0)]);

        // ------Indices----------
        self.indices.push(next_index);
        self.indices.push(next_index + 2);
        self.indices.push(next_index + 1);
        self.indices.push(next_index + 1);
        self.indices.push(next_index + 2);
        self.indices.push(next_index + 3);
    }
}

/// creates a mesh representing the solid blocks of a given chunk
/// The coordinates are relative to chunk.position.center()
pub fn create_chunk_mesh<M: ChunkMeshBuilder>(chunk: &Chunk, query: &Query<(&Chunk,)>, blocks: &StaticBlocks, mesh_builder: &mut M) {
    let chunk_data = &chunk.data;

    let x_positive = chunk.x_positive.and_then(|entity|Some(query.get_component(entity).ok()?));
    let x_negative = chunk.x_negative.and_then(|entity|Some(query.get_component(entity).ok()?));
    let y_positive = chunk.y_positive.and_then(|entity|Some(query.get_component(entity).ok()?));
    let y_negative = chunk.y_negative.and_then(|entity|Some(query.get_component(entity).ok()?));
    let z_positive = chunk.z_positive.and_then(|entity|Some(query.get_component(entity).ok()?));
    let z_negative = chunk.z_negative.and_then(|entity|Some(query.get_component(entity).ok()?));

    let check_block_face = |position: BlockVector, adjacent: Option<&Chunk>| -> bool {
        unsafe {
            if position.fits() {
                !chunk_data.get_unchecked(position).info.contains(BLOCK_MESH)
            } else {
                adjacent.map(|chunk| chunk.data.get_unchecked(position.chunk_relative()))
                    .map_or(true, |block|!block.info.contains(BLOCK_MESH))
            }
        }
    };

    let center = chunk.position.center();

    for (position, block) in chunk_data.iter() {
        let (look, _, personality) = &blocks[block.btype as usize];
        match look {
            BlockLook::Empty => {}
            BlockLook::DynamicBlockMesh => {
                unimplemented!()
            }
            BlockLook::CustomMesh => {
                unimplemented!()
            }
            BlockLook::Cube{ref textures } => {
                let global_position = chunk.position + position;
                let lower = global_position.lower_corner() - center;

                //println!("build block: {:?}", position);

                //Up
                if check_block_face(position.with_y(1), y_positive) {
                    mesh_builder.add_face(Face::new(
                        Vec3::new(0.0, 1.0, 0.0),
                        Vec3::new(1.0, 1.0, 1.0),
                        Vec3::new(-1.0, 0.0, 0.0),
                        Vec3::new(0.0, 0.0, -1.0),
                        16, textures[Side::Top]
                    ), lower);
                }
                //side 1
                if check_block_face(position.with_x(1), x_positive) {
                    mesh_builder.add_face(Face::new(
                        Vec3::new(1.0, 0.0, 0.0),
                        Vec3::new(1.0, 1.0, 1.0),
                        Vec3::new(0.0, 0.0, -1.0),
                        Vec3::new(0.0, -1.0, 0.0),
                        16, textures[Side::Front]
                    ), lower);
                }
                //side 2
                if check_block_face(position.with_z(1), z_positive) {
                    mesh_builder.add_face(Face::new(
                        Vec3::new(0.0, 0.0, 1.0),
                        Vec3::new(0.0, 1.0, 1.0),
                        Vec3::new(1.0, 0.0, 0.0),
                        Vec3::new(0.0, -1.0, 0.0),
                        16, textures[Side::Left]
                    ), lower);
                }
                //Down
                if check_block_face(position.with_y(-1), y_negative) {
                    mesh_builder.add_face(Face::new(
                        Vec3::new(0.0, -1.0, 0.0),
                        Vec3::zero(),
                        Vec3::new(0.0, 0.0, 1.0),
                        Vec3::new(1.0, 0.0, 0.0),
                        16, textures[Side::Bottom]
                    ), lower);
                }
                //side -1
                if check_block_face(position.with_x(-1), x_negative) {
                    mesh_builder.add_face(Face::new(
                        Vec3::new(-1.0, 0.0, 0.0),
                        Vec3::new(0.0, 1.0, 0.0),
                        Vec3::new(0.0, 0.0, 1.0),
                        Vec3::new(0.0, -1.0, 0.0),
                        16, textures[Side::Back]
                    ), lower);
                }
                //side -2
                if check_block_face(position.with_z(-1), z_negative) {
                    mesh_builder.add_face(Face::new(
                        Vec3::new(0.0, 0.0, -1.0),
                        Vec3::new(1.0, 1.0, 0.0),
                        Vec3::new(-1.0, 0.0, 0.0),
                        Vec3::new(0.0, -1.0, 0.0),
                        16, textures[Side::Right]
                    ), lower);
                }
            }
        }
    }
}