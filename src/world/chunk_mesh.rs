use bevy::render::mesh::{Mesh, Indices, VertexAttributeValues};
use crate::world::chunk::Chunk;
use bevy::render::pipeline::PrimitiveTopology;
use bevy::prelude::*;
use crate::world::block_inner::{BLOCK_MESH, TEXTURES, BlockLook, Side};
use crate::world::coordinates::BlockVector;

#[inline(always)]
fn create_face(verticies: &mut Vec<[f32; 3]>, indices: &mut Vec<u16>, normals: &mut Vec<[f32; 3]>, uvs: &mut Vec<[f32; 2]>,
               normal: Vec3,
               start: Vec3, ax1: Vec3, ax2: Vec3,
               uv_resolution: u32, uv_index: u32) {
    let next_index = verticies.len() as u16;

    let x = (uv_index % uv_resolution) as f32;
    let y = (uv_index / uv_resolution) as f32;

    let step = 1.0 / (uv_resolution as f32);



    //  0    1
    //   +--+
    //   | /|
    //   |/ |
    //   +--+
    //  2    3

    //0
    verticies.push(start.into());
    normals.push(normal.into());
    uvs.push([step * x, step * y]);

    //1
    verticies.push((start + ax1).into());
    normals.push(normal.into());
    uvs.push([step * (x + 1.0), step * y]);

    //2
    verticies.push((start + ax2).into());
    normals.push(normal.into());
    uvs.push([step * x, step * (y + 1.0)]);

    //3
    verticies.push((start + ax1 + ax2).into());
    normals.push(normal.into());
    uvs.push([step * (x + 1.0), step * (y + 1.0)]);

    // ------Indices----------
    indices.push(next_index);
    indices.push(next_index + 2);
    indices.push(next_index + 1);
    indices.push(next_index + 1);
    indices.push(next_index + 2);
    indices.push(next_index + 3);
}

pub fn create_chunk_mesh(chunk: &Chunk, query: &Query<(&Chunk, &mut Handle<Mesh>)>) -> Option<Mesh> {
    let mut verticies = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();
    let mut uvs = Vec::new();

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

    for (position, block) in chunk_data.iter() {
        if let BlockLook::Faces{ref textures } = TEXTURES[block.btype as usize] {

            let global_position = chunk.position + position;
            let lower= global_position.lower_corner();
            let heigher = global_position.higher_corner();

            //println!("build block: {:?}", position);

            //Up
            if check_block_face(position.with_y(1), y_positive) {
                create_face(&mut verticies, &mut indices, &mut normals, &mut uvs,
                            Vec3::new(0.0, 1.0, 0.0),
                            heigher, Vec3::new(-1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0),
                            16, textures[Side::Top]
                );
            }
            //side 1
            if check_block_face(position.with_x(1), x_positive) {
                create_face(&mut verticies, &mut indices, &mut normals, &mut uvs,
                            Vec3::new(1.0, 0.0, 0.0),
                            Vec3::new(heigher.x, heigher.y, heigher.z), Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, -1.0, 0.0),
                            16, textures[Side::Front]
                );
            }
            //side 2
            if check_block_face(position.with_z(1), z_positive) {
                create_face(&mut verticies, &mut indices, &mut normals, &mut uvs,
                            Vec3::new(0.0, 0.0, 1.0),
                            Vec3::new(lower.x, heigher.y, heigher.z), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, -1.0, 0.0),
                            16, textures[Side::Left]
                );
            }
            //Down
            if check_block_face(position.with_y(-1), y_negative) {
                create_face(&mut verticies, &mut indices, &mut normals, &mut uvs,
                            Vec3::new(0.0, -1.0, 0.0),
                            lower, Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0),
                            16, textures[Side::Bottom]
                );
            }
            //side -1
            if check_block_face(position.with_x(-1), x_negative) {
                create_face(&mut verticies, &mut indices, &mut normals, &mut uvs,
                            Vec3::new(0.0, 0.0, 1.0),
                            Vec3::new(lower.x, heigher.y, lower.z), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, -1.0, 0.0),
                            16, textures[Side::Back]
                );
            }
            //side -2
            if check_block_face(position.with_z(-1), z_negative) {
                create_face(&mut verticies, &mut indices, &mut normals, &mut uvs,
                            Vec3::new(0.0, 0.0, 1.0),
                            Vec3::new(heigher.x, heigher.y, lower.z), Vec3::new(-1.0, 0.0, 0.0), Vec3::new(0.0, -1.0, 0.0),
                            16, textures[Side::Right]
                );
            }
        }
    }

    if verticies.is_empty() {
        None
    } else {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, VertexAttributeValues::Float3(verticies));
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::Float3(normals));
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float2(uvs));
        mesh.set_indices(Some(Indices::U16(indices)));

        Some(mesh)
    }
}