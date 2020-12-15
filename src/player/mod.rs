use bevy::prelude::*;
use crate::player::player::PlayerMovement;

pub mod player;

#[derive(Bundle)]
struct PlayerBundle {
    transform: Transform,
    movement: PlayerMovement,
}

impl PlayerBundle {
    pub fn new(transform: Transform) -> Self {
        PlayerBundle {
            transform,
            movement: PlayerMovement::new(),
        }
    }
}

struct PlayerState {
    position: Vec3,
    movement: Vec3,
}