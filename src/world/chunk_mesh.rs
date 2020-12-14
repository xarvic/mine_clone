use bevy::ecs::{ResMut, Query};
use bevy::asset::{Assets, Handle};
use bevy::render::mesh::{Mesh, Indices};
use crate::world::chunk::Chunk;
use bevy::render::pipeline::PrimitiveTopology;
use bevy::prelude::*;
use crate::world::block::FULL;

#[inline(always)]
fn create_face(verticies: &mut Vec<[f32; 3]>, indices: &mut Vec<u16>, normals: &mut Vec<[f32; 3]>, uvs: &mut Vec<[f32; 2]>,
               normal: Vec3,
               start: Vec3, ax1: Vec3, ax2: Vec3,
               uv_start: Vec2, uv1: Vec2, uv2: Vec2) {
    let next_index = verticies.len() as u16;

    //  0    1
    //   +--+
    //   | /|
    //   |/ |
    //   +--+
    //  2    3

    //0
    verticies.push(start.into());
    normals.push(normal.into());
    uvs.push(uv_start.into());

    //1
    verticies.push((start + ax1).into());
    normals.push(normal.into());
    uvs.push((uv_start + uv1).into());

    //2
    verticies.push((start + ax2).into());
    normals.push(normal.into());
    uvs.push((uv_start + uv2).into());

    //3
    verticies.push((start + ax1 + ax2).into());
    normals.push(normal.into());
    uvs.push((uv_start + uv1 + uv2).into());

    // ------Indices----------
    indices.push(next_index);
    indices.push(next_index + 2);
    indices.push(next_index + 1);
    indices.push(next_index + 1);
    indices.push(next_index + 2);
    indices.push(next_index + 3);
}

pub fn create_chunk_mesh(chunk_data: &Chunk) -> Mesh {
    let mut verticies = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();
    let mut uvs = Vec::new();


    for (position, block) in chunk_data.iter() {
        if block.info.contains(FULL) {
            let lower = chunk_data.position + position;
            let heigher = lower + Vec3::new(1.0, 1.0, 1.0);

            //println!("build block: {:?}", position);

            //Up
            if chunk_data.get(position.y(1)).map_or(true, |block|!block.info.contains(FULL)) {
                create_face(&mut verticies, &mut indices, &mut normals, &mut uvs,
                            Vec3::new(0.0, 1.0, 0.0),
                            heigher, Vec3::new(-1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0),
                            Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)
                );
            }
            //side 1
            if chunk_data.get(position.x(1)).map_or(true, |block|!block.info.contains(FULL)) {
                create_face(&mut verticies, &mut indices, &mut normals, &mut uvs,
                            Vec3::new(1.0, 0.0, 0.0),
                            heigher, Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, -1.0, 0.0),
                            Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)
                );
            }
            //side 2
            if chunk_data.get(position.z(1)).map_or(true, |block|!block.info.contains(FULL)) {
                create_face(&mut verticies, &mut indices, &mut normals, &mut uvs,
                            Vec3::new(0.0, 0.0, 1.0),
                            heigher, Vec3::new(0.0, -1.0, 0.0), Vec3::new(-1.0, 0.0, 0.0),
                            Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)
                );
            }
            //Down
            if chunk_data.get(position.y(-1)).map_or(true, |block|!block.info.contains(FULL)) {
                create_face(&mut verticies, &mut indices, &mut normals, &mut uvs,
                            Vec3::new(0.0, -1.0, 0.0),
                            lower, Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0),
                            Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)
                );
            }
            //side -1
            if chunk_data.get(position.x(-1)).map_or(true, |block|!block.info.contains(FULL)) {
                create_face(&mut verticies, &mut indices, &mut normals, &mut uvs,
                            Vec3::new(-1.0, 0.0, 0.0),
                            lower,  Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0),
                            Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)
                );
            }
            //side -2
            if chunk_data.get(position.z(-1)).map_or(true, |block|!block.info.contains(FULL)) {
                create_face(&mut verticies, &mut indices, &mut normals, &mut uvs,
                            Vec3::new(0.0, 0.0, -1.0),
                            lower, Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0),
                            Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)
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

pub fn insert_rendered_chunk(mut commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>, chunk: Chunk) {
    let mesh = meshes.add(create_chunk_mesh(&chunk));

    let material = materials.add(StandardMaterial {
        albedo: Color::rgb(1.0, 0.7, 0.6),
        ..Default::default()
    });

    commands
        .spawn(PbrBundle {
            mesh,
            material: material.clone(),
            ..Default::default()
        })
        .with(chunk);
}

pub fn chunk_mesh_manager(mut meshes: ResMut<Assets<Mesh>>, query: Query<(&Handle<Mesh>, &Chunk)>) {
    for (handle, chunk) in query.iter() {

    }

}