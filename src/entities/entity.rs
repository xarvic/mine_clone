use bevy::math::Vec3;
use crate::physics::Quader;

/// An movable object which interacts with others and the static chunks
///
pub struct Object {
    velocity: Vec3,
    next_acceleration: Vec3,
    collider: Quader,
    flying: bool,
}

impl Object {
    pub fn new(collider: Quader, is_flying: bool) -> Self {
        Object {
            velocity: Vec3::zero(),
            next_acceleration: Vec3::zero(),
            collider,
            flying: is_flying,
        }
    }
    pub fn apply_force(&mut self, force: Vec3) {
        self.next_acceleration += force;
    }
    pub fn set_flying(&mut self, flying: bool) {
        self.flying = flying;
    }
    pub fn is_flying(&self) -> bool {
        self.flying
    }
    pub fn velocity(&self) -> Vec3 {
        self.velocity
    }
    pub(crate) fn calc_velocity(&mut self) {
        let friction = 0.9;
        self.velocity = self.velocity * friction + self.next_acceleration;
        if !self.flying {
            self.velocity.y -= 5.0;
        }
        self.next_acceleration = Vec3::zero();
    }
    pub fn get_collider(&self) -> Quader {
        self.collider
    }
}