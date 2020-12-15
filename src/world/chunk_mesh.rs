use bevy::render::mesh::{Mesh, Indices};
use crate::world::chunk::Chunk;
use bevy::render::pipeline::PrimitiveTopology;
use bevy::prelude::*;
use crate::world::block::BLOCK_MESH;

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

pub fn create_chunk_mesh(chunk: &Chunk) -> Mesh {
    let mut verticies = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();
    let mut uvs = Vec::new();

    let chunk_data = &chunk.data;

    for (position, block) in chunk_data.iter() {
        if block.info.contains(BLOCK_MESH) {
            let lower = chunk.position + position;
            let heigher = lower + Vec3::new(1.0, 1.0, 1.0);

            //println!("build block: {:?}", position);

            //Up
            if chunk_data.get(position.y(1)).map_or(true, |block|!block.info.contains(BLOCK_MESH)) {
                create_face(&mut verticies, &mut indices, &mut normals, &mut uvs,
                            Vec3::new(0.0, 1.0, 0.0),
                            heigher, Vec3::new(-1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0),
                            16, 0
                );
            }
            //side 1
            if chunk_data.get(position.x(1)).map_or(true, |block|!block.info.contains(BLOCK_MESH)) {
                create_face(&mut verticies, &mut indices, &mut normals, &mut uvs,
                            Vec3::new(1.0, 0.0, 0.0),
                            Vec3::new(heigher.x, heigher.y, heigher.z), Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, -1.0, 0.0),
                            16, 3
                );
            }
            //side 2
            if chunk_data.get(position.z(1)).map_or(true, |block|!block.info.contains(BLOCK_MESH)) {
                create_face(&mut verticies, &mut indices, &mut normals, &mut uvs,
                            Vec3::new(0.0, 0.0, 1.0),
                            Vec3::new(lower.x, heigher.y, heigher.z), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, -1.0, 0.0),
                            16, 3
                );
            }
            //Down
            if chunk_data.get(position.y(-1)).map_or(true, |block|!block.info.contains(BLOCK_MESH)) {
                create_face(&mut verticies, &mut indices, &mut normals, &mut uvs,
                            Vec3::new(0.0, -1.0, 0.0),
                            lower, Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0),
                            16, 2
                );
            }
            //side -1
            if chunk_data.get(position.x(-1)).map_or(true, |block|!block.info.contains(BLOCK_MESH)) {
                create_face(&mut verticies, &mut indices, &mut normals, &mut uvs,
                            Vec3::new(0.0, 0.0, 1.0),
                            Vec3::new(lower.x, heigher.y, lower.z), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, -1.0, 0.0),
                            16, 3
                );
            }
            //side -2
            if chunk_data.get(position.z(-1)).map_or(true, |block|!block.info.contains(BLOCK_MESH)) {
                create_face(&mut verticies, &mut indices, &mut normals, &mut uvs,
                            Vec3::new(0.0, 0.0, 1.0),
                            Vec3::new(heigher.x, heigher.y, lower.z), Vec3::new(-1.0, 0.0, 0.0), Vec3::new(0.0, -1.0, 0.0),
                            16, 3
                );
            }
        }
    }
    println!("created mesh with {} faces ({} vertices)!", indices.len() / 3, verticies.len());

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, verticies);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U16(indices)));

    mesh
}

