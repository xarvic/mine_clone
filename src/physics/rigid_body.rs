use std::num::NonZeroU64;
use bevy::math::Vec3;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc::channel;
use crate::world::chunk::{ChunkManager, Chunk};
use bevy::ecs::Query;
use bevy::prelude::Transform;
use crate::world::block_types::{StaticBlocks, BlockFeel};
use crate::physics::collider::AAQuader;
use std::mem::replace;
use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct RigidBodyHandle(NonZeroU64);

pub struct RigidBody {
    position: Vec3,
    velocity: Vec3,
    next_force: Vec3,
    flying: bool,
    inv_mass: f32,
    handle: RigidBodyHandle,

    collider: AAQuader,
}

impl RigidBody {
    pub fn new(position: Vec3, inv_mass: f32, flying: bool, collider: AAQuader) -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);

        let value = COUNTER.fetch_add(1, Ordering::SeqCst);

        RigidBody {
            position,
            velocity: Default::default(),
            next_force: Default::default(),
            flying,
            inv_mass: 0.0,
            handle: RigidBodyHandle(unsafe { NonZeroU64::new_unchecked(value) }),
            collider,
        }
    }
    pub fn apply_force(&mut self) {
        //Dampening
        //TODO: specify ground and air friction
        if self.flying {
            self.velocity *= 0.8;
        } else {
            self.velocity = (self.velocity * 0.75) + Vec3::new(0.0, -0.1, 0.0);
        }
        //Apply force
        let force = replace(&mut self.next_force, Vec3::zero());
        self.velocity += force;
        self.position += self.velocity;
    }
    pub fn position(&self) -> Vec3 {
        self.position
    }
    pub fn velocity(&self) -> Vec3 {
        self.velocity
    }
    pub fn handle(&self) -> RigidBodyHandle {
        self.handle
    }
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }
    pub fn add_force(&mut self, force: Vec3) {
        self.next_force += force;
    }
}

pub enum PhysicsCommand {
}

pub enum PhysicsEvent {

}

pub struct PhysicsEngine {
    mapping: HashMap<RigidBodyHandle, usize>,
    rigid_bodies: Vec<RigidBody>,

    connection: Mutex<(Receiver<PhysicsCommand>, Sender<PhysicsEvent>)>,
}

impl PhysicsEngine {
    pub fn new() -> (PhysicsEngine, Sender<PhysicsCommand>, Receiver<PhysicsEvent>) {
        let (command_sender, command_reciever) = channel();
        let (event_sender, event_reciever) = channel();

        (PhysicsEngine::create(command_reciever, event_sender), command_sender, event_reciever)
    }
    pub fn create(input: Receiver<PhysicsCommand>, output: Sender<PhysicsEvent>) -> PhysicsEngine {
        PhysicsEngine {
            mapping: HashMap::new(),
            rigid_bodies: Vec::new(),

            connection: Mutex::new((input, output)),
        }
    }
    /// recieves physic commands, updates the rigidbodies, sends physic events
    ///
    /// The world state meight be inconsistent if the physic events are intercepted before update_entities
    /// was called!
    pub fn step(&mut self, world: &ChunkManager, static_blocks: &StaticBlocks, chunks: Query<(&Chunk,)>) {
        //TODO: interation with moving enitities

        //Apply force and speed
        //Interaction moving entities and world
        for rigid_body in self.rigid_bodies.iter_mut() {
            rigid_body.apply_force();

            let mut collider = rigid_body.collider.translated(rigid_body.position);

            for position in collider.contained() {
                if let Some(block) = world.get(position, &chunks) {
                    match static_blocks[block.btype as usize].1 {
                        BlockFeel::Empty => {}
                        BlockFeel::ColliderSet(colliders) => {
                            println!("check colliders!");
                            for block_collider in colliders.iter().map(|c|c.translated(position.lower_corner())) {
                                let result = collider.impact_volume(block_collider);
                                if result.x != 0.0 && result.y != 0.0 && result.z != 0.0 {
                                    println!(" - impact: {}", result);
                                    restrict_motion(result, rigid_body);
                                    collider = rigid_body.collider.translated(rigid_body.position);
                                }
                            }
                        }
                        BlockFeel::Custom => {}
                    }
                }
            }
        }
    }

    pub fn update_entities(&mut self, mut objects: Query<(&RigidBodyHandle, &mut Transform)>) {
        for (handle, mut transform) in objects.iter_mut() {
            if let Some(rigid_body) = self.get_mut(*handle) {
                transform.translation = rigid_body.position();
            }
        }
    }
    pub fn create_rigid_body(&mut self, position: Vec3, inv_mass: f32, is_flying: bool, collider: AAQuader) -> RigidBodyHandle {
        let rigid_body = RigidBody::new(position, inv_mass, is_flying, collider);
        let handle = rigid_body.handle();
        self.rigid_bodies.push(rigid_body);
        self.mapping.insert(handle, self.rigid_bodies.len() - 1);
        handle
    }
    pub fn remove_rigid_body(&mut self, handle: RigidBodyHandle) {
        if let Some(index) = self.mapping.remove(&handle) {
            self.rigid_bodies.swap_remove(index);
            if let Some(body) = self.rigid_bodies.get(index) {
                self.mapping.insert(body.handle(), index);
            }
        };
    }
    pub fn get_mut(&mut self, handle: RigidBodyHandle) -> Option<&mut RigidBody> {
        let index = self.mapping.get(&handle).cloned();
        index.and_then(move|index|self.rigid_bodies.get_mut(index))
    }
}

fn restrict_motion(impact_volume: Vec3, rigid_body: &mut RigidBody) {
    let tm = (impact_volume * rigid_body.velocity).abs();

    println!("restrict {}", tm);

    let impact_translation;

    if tm.x > tm.y && tm.x > tm.z {
        impact_translation = Vec3::new(impact_volume.x, 0.0, 0.0);
    } else if tm.y >= tm.z {
        impact_translation = Vec3::new(0.0, impact_volume.y, 0.0);
    } else {
        impact_translation = Vec3::new(0.0, 0.0, impact_volume.z);
    }
    rigid_body.position += impact_translation;
    let impact_normal = impact_translation.normalize();
    rigid_body.velocity += impact_normal * (impact_normal.dot(rigid_body.velocity)).max(0.0)
}