use std::num::NonZeroU64;
use bevy::math::Vec3;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc::channel;
use crate::world::chunk::{ChunkManager, Chunk};
use bevy::ecs::Query;
use bevy::prelude::Transform;

struct RigidBodyComp {
}

struct RigidBodyHandle(NonZeroU64);

struct RigidBody {
    position: Vec3,
    velocity: Vec3,
    use_gravity: bool,
    inv_mass: f32,

}

enum PhysicsCommand {

}

enum PhysicsEvent {

}

struct PhysicsEngine {
    rigid_bodies: Vec<RigidBody>,

    input: Receiver<PhysicsCommand>,
    output: Sender<PhysicsEvent>,
}

impl PhysicsEngine {
    pub fn new() -> (PhysicsEngine, Sender<PhysicsCommand>, Receiver<PhysicsEvent>) {
        let (command_sender, command_reciever) = channel();
        let (event_sender, event_reciever) = channel();

        (PhysicsEngine::create(command_reciever, event_sender), command_sender, event_reciever)
    }
    pub fn create(input: Receiver<PhysicsCommand>, output: Sender<PhysicsEvent>) -> PhysicsEngine {
        PhysicsEngine {
            rigid_bodies: Vec::new(),

            input,
            output,
        }
    }
    /// recieves physic commands, updates the rigidbodies, sends physic events
    ///
    /// The world state meight be inconsistent if the physic events are intercepted before update_entities
    /// was called!
    pub fn step(&mut self, world: &ChunkManager, chunks: Query<(&Chunk,)>) {
        unimplemented!()
    }

    pub fn update_entities(&mut self, objects: Query<(&RigidBodyComp, &mut Transform)>) {
        unimplemented!()
    }
}