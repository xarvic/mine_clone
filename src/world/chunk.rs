use crate::world::block_inner::{BlockInner, AIR, GRASS, DIRT, STONE};
use crate::world::chunk_mesh::create_chunk_mesh;

use std::mem::replace;
use std::ops::{Index, IndexMut};
use itertools::__std_iter::repeat;
use std::collections::HashMap;

use bevy::prelude::*;
use crate::world::coordinates::{ChunkPosition, BlockVector, CHUNK_SIZE, BlockPosition, MAX_CHILD};
use std::collections::HashSet;
use std::ops::{Deref, DerefMut};
use crate::player::player::PlayerMovement;
use itertools::Itertools;

use utils::{MapData, create_perlin_noise};

pub struct Chunk {
    pub position: ChunkPosition,
    pub data: ChunkData,

    //Adjacent chunks
    pub x_positive: Option<Entity>,
    pub x_negative: Option<Entity>,
    pub y_positive: Option<Entity>,
    pub y_negative: Option<Entity>,
    pub z_positive: Option<Entity>,
    pub z_negative: Option<Entity>,
}

impl Chunk {

}

#[derive(Clone)]
pub struct ChunkData {
    pub blocks: [[[BlockInner; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
}

impl ChunkData {
    pub fn new(data: [[[BlockInner; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]) -> Self {
        Self{
            blocks: data,
        }
    }
    pub fn filled(block: BlockInner) -> Self {
        Self::new([[[block; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize])
    }
    pub fn get(&self, pos: BlockVector) -> Option<&BlockInner> {
        if pos.fits() {
            unsafe {
                Some(self.get_unchecked(pos))
            }
        } else {
            None
        }
    }

    pub unsafe fn get_unchecked(&self, pos: BlockVector) -> &BlockInner {
        self.blocks.get_unchecked(pos.x as usize)
            .get_unchecked(pos.y as usize)
            .get_unchecked(pos.z as usize)
    }
    pub fn get_mut(&mut self, pos: BlockVector) -> Option<&mut BlockInner> {
        if pos.fits() {
            unsafe {
                Some(self.get_unchecked_mut(pos))
            }
        } else {
            None
        }
    }
    pub unsafe fn get_unchecked_mut<'a>(&'a mut self, pos: BlockVector) -> &'a mut BlockInner {
        self.blocks.get_unchecked_mut(pos.x as usize)
            .get_unchecked_mut(pos.y as usize)
            .get_unchecked_mut(pos.z as usize)
    }
    pub fn clear(&mut self, pos: BlockVector) -> BlockInner {
        replace(&mut self.get_mut(pos).unwrap(), AIR)
    }
    pub fn iter(&self) -> impl Iterator<Item = (BlockVector, &BlockInner)> {
        self.blocks.iter().enumerate()
            .flat_map(|(x, items)|items.iter().enumerate().zip(repeat(x)))
            .flat_map(|((y, items), x)|items.iter().enumerate().zip(repeat((x, y))))
            .map(|((z, block), (x, y))|(BlockVector::new(x as i64, y as i64, z as i64), block))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (BlockVector, &mut BlockInner)> {
        self.blocks.iter_mut().enumerate()
            .flat_map(|(x, items)|items.iter_mut().enumerate().zip(repeat(x)))
            .flat_map(|((y, items), x)|items.iter_mut().enumerate().zip(repeat((x, y))))
            .map(|((z, block), (x, y))|(BlockVector::new(x as i64, y as i64, z as i64), block))
    }
}

impl<T: Into<BlockVector>> Index<T> for ChunkData {
    type Output = BlockInner;

    fn index(&self, index: T) -> &Self::Output {
        let pos = index.into();
        assert!(pos.fits(), "invalid Blockposition");
        unsafe{
            self.get_unchecked(pos)
        }
    }
}

impl<T: Into<BlockVector>> IndexMut<T> for ChunkData {
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        let pos = index.into();
        assert!(pos.fits(), "invalid Blockposition");
        unsafe{
            self.get_unchecked_mut(pos)
        }
    }
}

fn chunk_loader(position: ChunkPosition) -> ChunkData {
    if position.y != 0 {
        ChunkData::filled(AIR)
    } else {
        let mut map_data = MapData::default();

        let noise = create_perlin_noise(0, position.x * CHUNK_SIZE, position.z * CHUNK_SIZE, 8);

        let mut chunk = ChunkData::filled(AIR);
        for (position , block) in chunk.iter_mut() {
            if noise[position.x as usize][position.z as usize] >= position.y as f32 + 0.5 {
                *block = if position.y == 15 {
                    GRASS
                } else if position.y > 9 {
                    DIRT
                } else {
                    STONE
                };
            }
        }
        chunk
    }
}

pub struct ChunkManager {
    chunks: HashMap<ChunkPosition, Entity>,
    player_chunk: ChunkPosition,
    chunk_loading_distance: f32,
    chunk_discard_distance: f32,
    texture_atlas: Option<Handle<StandardMaterial>>,
    chunk_rerender: HashSet<ChunkPosition>,
    current_meshes: isize,
}

impl ChunkManager {
    pub fn new(current_position: ChunkPosition, chunk_loading_distance: f32, chunk_discard_distance: f32) -> Self {
        Self {
            chunks: HashMap::new(),
            player_chunk: current_position,
            chunk_loading_distance: chunk_loading_distance as f32,
            chunk_discard_distance: chunk_discard_distance as f32,
            texture_atlas: None,
            chunk_rerender: HashSet::new(),
            current_meshes: 0,
        }
    }
    fn init(&mut self, commands: &mut Commands,
            server: Res<AssetServer>,
            mut materials: ResMut<Assets<StandardMaterial>>,
            mut meshes: ResMut<Assets<Mesh>>,
    ) {
        println!("init registry!");

        let texture: Handle<Texture> = server.load("textures.png");
        let material = StandardMaterial{
            albedo: Default::default(),
            albedo_texture: Some(texture),
            shaded: false
        };

        let material_handle = materials.add(material);

        self.texture_atlas = Some(material_handle);
    }
    fn load_chunk(&mut self, commands: &mut Commands, chunk_position: ChunkPosition) -> Entity {
        let chunk_data = chunk_loader(chunk_position);

        let chunk = Chunk {
            position: chunk_position,
            data: chunk_data,

            /*x_positive: self.chunks.get(&chunk_position.with_x( 1)).cloned(),
            x_negative: self.chunks.get(&chunk_position.with_x(-1)).cloned(),
            y_positive: self.chunks.get(&chunk_position.with_y( 1)).cloned(),
            y_negative: self.chunks.get(&chunk_position.with_y(-1)).cloned(),
            z_positive: self.chunks.get(&chunk_position.with_z( 1)).cloned(),
            z_negative: self.chunks.get(&chunk_position.with_z(-1)).cloned(),*/
            x_positive: None,
            x_negative: None,
            y_positive: None,
            y_negative: None,
            z_positive: None,
            z_negative: None
        };

        commands
            .spawn(PbrBundle{
                material: self.texture_atlas.as_ref().unwrap().clone(),
                transform: Transform::from_translation(chunk_position.center()),
                ..PbrBundle::default()
            })
            .with(chunk);
        let entity = commands.current_entity().unwrap();
        self.chunks.insert(chunk_position, entity);

        entity
    }
    pub fn update(&mut self, commands: &mut Commands, meshes: ResMut<Assets<Mesh>>, new_position: ChunkPosition) {

    }
    pub fn get<'a>(&self, position: BlockPosition, query: &'a Query<(&Chunk,)>) -> Option<&'a BlockInner> {
        let (block, chunk) = position.local();
        unsafe {Some(query.get_component::<Chunk>(*self.chunks.get(&chunk)?).ok()?.data.get_unchecked(block))}
    }
    pub fn get_with_mut<'a>(&self, position: BlockPosition, query: &'a mut Query<(&mut Chunk,)>) -> Option<&'a BlockInner> {
        let (block, chunk) = position.local();
        unsafe {Some(query.get_component::<Chunk>(*self.chunks.get(&chunk)?).ok()?.data.get_unchecked(block))}
    }
    /// the caller must ensure that blockupdates are handled correctly!
    fn get_mut<'a>(&self, position: BlockPosition, query: &'a mut Query<(&mut Chunk,)>) -> Option<MutBlock<'a>> {
        let (block, chunk) = position.local();
        let res = query.get_mut(*self.chunks.get(&chunk)?);
        Some(MutBlock {
            inner: res.ok()?.0,
            position: block,
        })
    }
    pub fn set(&mut self, position: BlockPosition, block: BlockInner, query: &mut Query<(&mut Chunk,)>) {
        if let Some(mut block_ref) = self.get_mut(position, query) {
            *block_ref = block;

            let (block, chunk) = position.local();
            self.chunk_rerender.insert(chunk);
            if block.x == 0 {
                self.chunk_rerender.insert(position.with_x(-1).chunk());
            }
            if block.x == MAX_CHILD {
                self.chunk_rerender.insert(position.with_x(1).chunk());
            }
            if block.y == 0 {
                self.chunk_rerender.insert(position.with_y(-1).chunk());
            }
            if block.y == MAX_CHILD {
                self.chunk_rerender.insert(position.with_y(1).chunk());
            }
            if block.z == 0 {
                self.chunk_rerender.insert(position.with_z(-1).chunk());
            }
            if block.z == MAX_CHILD {
                self.chunk_rerender.insert(position.with_z(1).chunk());
            }
        }
        //self.block_updates.extend(position.adjacent());
    }
    /*pub fn get_present_blocks<'a>(&'a self, positions: GridSnap, query: &'a Query<'a, (&'a Chunk,)>) -> Blocks<'a> {
        Blocks {
            chunks: self,
            gid_snap: positions,
            last_chunk: None,
            query,
        }
    }*/
}
/*
struct Blocks<'a> {
    chunks: &'a ChunkManager,
    gid_snap: GridSnap,
    last_chunk: Option<(*Chunk, ChunkPosition)>,
    query: &'a Query<'a, (&'a Chunk,)>,
}

impl<'a> Iterator for Blocks<'a> {
    type Item = (BlockPosition, Option<&'a Block>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((mut last, chunk_position)) = self.last_chunk.as_mut() {
            let (local, new_chunk_position) = self.gid_snap.next().unwrap();
            if new_chunk_position != chunk_position {
                *last = self.query.get(self.chunks.chunks.get(new_chunk_position))?;
            }

            Some(())
        } else {
            None
        }
    }
}*/

pub struct MutBlock<'a> {
    inner: Mut<'a, Chunk>,
    position: BlockVector,
}

impl<'a> Deref for MutBlock<'a> {
    type Target = BlockInner;

    fn deref(&self) -> &Self::Target {
        unsafe {&self.inner.data.get_unchecked(self.position)}
    }
}

impl<'a> DerefMut for MutBlock<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {self.inner.deref_mut().data.get_unchecked_mut(self.position)}
    }
}

pub fn init_chunks(commands: &mut Commands,
                   resources: Res<AssetServer>,
                   mut registry: ResMut<ChunkManager>,
                   mut textures: ResMut<Assets<StandardMaterial>>,
                   mut meshes: ResMut<Assets<Mesh>>,
                   ) {
    registry.init(commands, resources, textures, meshes);
}

pub fn update_chunk_scope(
    commands: &mut Commands,
    mut manager: ResMut<ChunkManager>,
    mut chunks: Query<(&mut Chunk,)>,
    player: Query<(&Transform, &PlayerMovement)>,
) {
    for (transform, options) in player.iter() {
        manager.player_chunk = ChunkPosition::from(transform.translation);

        let load_dist = manager.chunk_loading_distance as i64;

        let mut load = Vec::new();

        let mut load_dist_square = manager.chunk_loading_distance * CHUNK_SIZE as f32;
        load_dist_square *= load_dist_square;

            (-load_dist..=load_dist)
            .cartesian_product(-load_dist..=load_dist)
            .cartesian_product(-load_dist..=load_dist)
            .map(|((x, y), z)| {
                manager.player_chunk.with_x(x).with_y(y).with_z(z)
            }).for_each(|position| {
            if position.center().distance_squared(transform.translation) < load_dist_square && !manager.chunks.contains_key(&position) {
                load.push(position);
            }
        });

        for position in load.iter() {
            manager.load_chunk(commands, *position);
        }

        let unload_dist_square = manager.chunk_discard_distance *
            manager.chunk_discard_distance *
            CHUNK_SIZE as f32 *
            CHUNK_SIZE as f32;

        for (mut chunk,) in chunks.iter_mut() {

            if chunk.position.center().distance_squared(transform.translation) >= unload_dist_square {
                commands.despawn(manager.chunks.remove(&chunk.position).unwrap());
            } else {
                let mut changed = false;
                let mut complete = true;

                if chunk.x_negative.is_none() {
                    if let Some(adjacent) = manager.chunks
                        .get(&chunk.position.with_x(-1))
                    {
                        chunk.x_negative = Some(*adjacent);
                        changed = true;
                    } else {
                        complete = false;
                    }
                }
                if chunk.x_positive.is_none() {
                    if let Some(adjacent) = manager.chunks
                        .get(&chunk.position.with_x(1))
                    {
                        chunk.x_positive = Some(*adjacent);
                        changed = true;
                    } else {
                        complete = false;
                    }
                }
                if chunk.y_negative.is_none() {
                    if let Some(adjacent) = manager.chunks
                        .get(&chunk.position.with_y(-1))
                    {
                        chunk.y_negative = Some(*adjacent);
                        changed = true;
                    } else {
                        complete = false;
                    }
                }
                if chunk.y_positive.is_none() {
                    if let Some(adjacent) = manager.chunks
                        .get(&chunk.position.with_y(1))
                    {
                        chunk.y_positive = Some(*adjacent);
                        changed = true;
                    } else {
                        complete = false;
                    }
                }
                if chunk.z_negative.is_none() {
                    if let Some(adjacent) = manager.chunks
                        .get(&chunk.position.with_z(-1))
                    {
                        chunk.z_negative = Some(*adjacent);
                        changed = true;
                    }
                }
                if chunk.z_positive.is_none() {
                    if let Some(adjacent) = manager.chunks
                        .get(&chunk.position.with_z(1))
                    {
                        chunk.z_positive = Some(*adjacent);
                        changed = true;
                    } else {
                        complete = false;
                    }
                }

                if changed && complete {
                    manager.chunk_rerender.insert(chunk.position);
                }
            }
        }
    }
}

pub fn update_chunk_mesh(
    mut meshes: ResMut<Assets<Mesh>>,
    mut manager: ResMut<ChunkManager>,
    mut chunks: Query<(&Chunk, &mut Handle<Mesh>)>,
    adjacent: Query<(&Chunk,)>,
) {
    let mut change = 0;
    for position in manager.chunk_rerender.iter() {
        if let Some(entity) = manager.chunks
            .get(position) {
            if let Ok((chunk, mut handle)) = chunks.get_mut(*entity) {
                //This is ok, we are only accsessing further Chunks immutably
                if handle.is_strong() {
                    meshes.remove(handle.clone());
                    change -= 1;
                }
                let mesh = create_chunk_mesh(chunk, &adjacent);
                if let Some(mesh) = mesh {
                    // Some -> Some just update the mesh
                    *handle = meshes.add(mesh);
                    change += 1;
                } else {
                    // Some -> None remove the mesh
                    *handle = Handle::default();
                }
            }
        } else {
            //Error the component map contains an invalid entity
            eprintln!("\033[34mTried to update a non exsiting chunk!\033[0m");
        }
    }
    manager.current_meshes += change;
    manager.chunk_rerender.clear();
}