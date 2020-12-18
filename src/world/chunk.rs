use crate::world::block::{Block, AIR, GROUND};
use crate::world::chunk_mesh::create_chunk_mesh;

use std::mem::replace;
use std::time::Instant;
use std::ops::{Index, IndexMut};
use itertools::__std_iter::repeat;
use std::collections::HashMap;

use bevy::prelude::*;
use crate::world::coordinates::{ChunkPosition, BlockVector, CHUNK_SIZE, BlockPosition, MAX_CHILD};
use std::collections::HashSet;
use std::ops::{Deref, DerefMut};


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
    pub fn lowest(&self) -> BlockPosition {
        self.position + BlockVector::new(0, 0, 0)
    }
    pub fn highest(&self) -> BlockPosition {
        self.position + BlockVector::new(MAX_CHILD, MAX_CHILD, MAX_CHILD)
    }
}

#[derive(Clone)]
pub struct ChunkData {
    pub blocks: [[[Block; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
}

impl ChunkData {
    pub fn new(data: [[[Block; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]) -> Self {
        Self{
            blocks: data,
        }
    }
    pub fn filled(block: Block) -> Self {
        Self::new([[[block; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize])
    }
    pub fn get(&self, pos: BlockVector) -> Option<&Block> {
        if pos.fits() {
            unsafe {
                Some(self.get_unchecked(pos))
            }
        } else {
            None
        }
    }

    pub unsafe fn get_unchecked(&self, pos: BlockVector) -> &Block {
        self.blocks.get_unchecked(pos.x as usize)
            .get_unchecked(pos.y as usize)
            .get_unchecked(pos.z as usize)
    }
    pub fn get_mut(&mut self, pos: BlockVector) -> Option<&mut Block> {
        if pos.fits() {
            unsafe {
                Some(self.get_unchecked_mut(pos))
            }
        } else {
            None
        }
    }
    pub unsafe fn get_unchecked_mut<'a>(&'a mut self, pos: BlockVector) -> &'a mut Block {
        self.blocks.get_unchecked_mut(pos.x as usize)
            .get_unchecked_mut(pos.y as usize)
            .get_unchecked_mut(pos.z as usize)
    }
    pub fn clear(&mut self, pos: BlockVector) -> Block {
        replace(&mut self.get_mut(pos).unwrap(), AIR)
    }
    pub fn iter(&self) -> impl Iterator<Item = (BlockVector, &Block)> {
        self.blocks.iter().enumerate()
            .flat_map(|(x, items)|items.iter().enumerate().zip(repeat(x)))
            .flat_map(|((y, items), x)|items.iter().enumerate().zip(repeat((x, y))))
            .map(|((z, block), (x, y))|(BlockVector::new(x as i64, y as i64, z as i64), block))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (BlockVector, &mut Block)> {
        self.blocks.iter_mut().enumerate()
            .flat_map(|(x, items)|items.iter_mut().enumerate().zip(repeat(x)))
            .flat_map(|((y, items), x)|items.iter_mut().enumerate().zip(repeat((x, y))))
            .map(|((z, block), (x, y))|(BlockVector::new(x as i64, y as i64, z as i64), block))
    }
}

impl<T: Into<BlockVector>> Index<T> for ChunkData {
    type Output = Block;

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
    if position.y > 0 {
        ChunkData::filled(AIR)
    } else {
        ChunkData::filled(GROUND)
    }
}

pub struct ChunkManager {
    chunks: HashMap<ChunkPosition, Entity>,
    player_chunk: ChunkPosition,
    chunk_loading_distance: f32,
    chunk_discard_distance: f32,
    texture_atlas: Option<Handle<StandardMaterial>>,
    chunk_rerender: HashSet<ChunkPosition>,
}

impl ChunkManager {
    pub fn new(current_position: ChunkPosition, chunk_loading_distance: u32, chunk_discard_distance: u32) -> Self {
        Self {
            chunks: HashMap::new(),
            player_chunk: current_position,
            chunk_loading_distance: chunk_loading_distance as f32,
            chunk_discard_distance: chunk_discard_distance as f32,
            texture_atlas: None,
            chunk_rerender: HashSet::new(),
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

        self.load_chunk(commands, &mut meshes, ChunkPosition::new(0, 0, 0), true);
        self.load_chunk(commands, &mut meshes, ChunkPosition::new(1, 0, 0), true);
        self.load_chunk(commands, &mut meshes, ChunkPosition::new(0, 0, 1), true);
        self.load_chunk(commands, &mut meshes, ChunkPosition::new(1, 0, 1), true);
    }
    fn load_chunk(&mut self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, chunk_position: ChunkPosition, rendered: bool) {
        let chunk_data = chunk_loader(chunk_position);

        let chunk = Chunk {
            position: chunk_position,
            data: chunk_data,

            x_positive: self.chunks.get(&chunk_position.with_x( 1)).cloned(),
            x_negative: self.chunks.get(&chunk_position.with_x(-1)).cloned(),
            y_positive: self.chunks.get(&chunk_position.with_y( 1)).cloned(),
            y_negative: self.chunks.get(&chunk_position.with_y(-1)).cloned(),
            z_positive: self.chunks.get(&chunk_position.with_z( 1)).cloned(),
            z_negative: self.chunks.get(&chunk_position.with_z(-1)).cloned(),
        };

        let entity = if rendered {
            let start = Instant::now();
            let mesh = meshes.add(create_chunk_mesh(&chunk));
            println!("created mesh in {} sec. !", start.elapsed().as_secs_f32());

            let handle = self.texture_atlas.clone().unwrap();

            println!("spawn cube mesh!");
            commands
                .spawn(PbrBundle {
                    mesh,
                    material: handle,
                    ..Default::default()
                })
                .with(chunk);
            commands.current_entity().unwrap()//We just inserted one, this should be ok
        } else {

            commands.spawn((chunk,));
            commands.current_entity().unwrap()
        };

        self.chunks.insert(chunk_position, entity);
    }
    pub fn update(&mut self, commands: &mut Commands, meshes: ResMut<Assets<Mesh>>, new_position: ChunkPosition) {

    }
    pub fn get<'a>(&self, position: BlockPosition, query: &'a Query<(&Chunk,)>) -> Option<&'a Block> {
        let (block, chunk) = position.local();
        unsafe {Some(query.get_component::<Chunk>(*self.chunks.get(&chunk)?).ok()?.data.get_unchecked(block))}
    }
    pub fn get_with_mut<'a>(&self, position: BlockPosition, query: &'a mut Query<(&mut Chunk,)>) -> Option<&'a Block> {
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
    pub fn set(&mut self, position: BlockPosition, block: Block, query: &mut Query<(&mut Chunk,)>) {
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
    type Target = Block;

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

pub fn update_chunk_mesh(
    mut meshes: ResMut<Assets<Mesh>>,
    mut manager: ResMut<ChunkManager>,
    mut query: Query<(&Chunk, &mut Handle<Mesh>)>
) {
    for position in manager.chunk_rerender.iter() {
        if let Some((chunk, mut handle)) = manager.chunks
            .get(position)
            .and_then(|chunk|query.get_mut(*chunk).ok())
        {
            meshes.remove(handle.clone());
            let mesh = create_chunk_mesh(chunk);
            *handle = meshes.add(mesh);
            println!("update chunk mesh!");
        }
    }
    manager.chunk_rerender.clear();
}