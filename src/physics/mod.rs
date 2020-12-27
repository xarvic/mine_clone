use bevy::app::AppBuilder;
use crate::settings::Settings;
use crate::physics::rigid_body::{PhysicsEngine, RigidBodyHandle};
use bevy::ecs::{ResMut, Query, Res};
use crate::world::chunk::{Chunk, ChunkManager};
use bevy::prelude::{Transform, IntoSystem};
use crate::world::block_types::StaticBlocksRes;

pub mod rigid_body;
pub mod ray;
pub mod collider;

pub fn init_physics(builder: &mut AppBuilder, settings: &Settings) {
    let (engine, _sender, _reviever) = PhysicsEngine::new();
    builder.add_resource(engine);
    builder.add_system(update_physics.system());
}

fn update_physics(
    mut physic_engine: ResMut<PhysicsEngine>,
    chunk_manager: Res<ChunkManager>,
    static_blocks: Res<StaticBlocksRes>,
    chunks: Query<(&Chunk,)>,
    enities: Query<(&RigidBodyHandle, &mut Transform)>
) {
    physic_engine.step(&chunk_manager, &static_blocks, chunks);
    physic_engine.update_entities(enities);
}