use bevy::prelude::*;
use player::{PlayerMovement, camera_movement_system, mouse_motion_system, MouseState};
use crate::settings::Settings;
use crate::util::{print_fps, FPS};
use bevy_rapier3d::rapier::dynamics::{RigidBodyBuilder, MassProperties};
use bevy_rapier3d::rapier::geometry::ColliderBuilder;

pub mod player;

pub fn init_player(builder: &mut AppBuilder, settings: &Settings) {
    builder.add_system(camera_movement_system.system())
        .add_system(mouse_motion_system.system())
        .add_system(print_fps.system())
        .add_startup_system(setup.system());
}

/// set up a simple scene with a "parent" cube and a "child" cube
//TODO: mut 'commands: Commands' only works on 0.3 (change to 'commands: &mut Commands' otherwise)!
fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    textures: ResMut<Assets<Texture>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {

    let rigid_body = RigidBodyBuilder::new_dynamic().mass(0.0, true);
    let collider = ColliderBuilder::capsule_y(1.0, 1.0);

    commands
        .insert_resource(MouseState::default())
        .insert_resource(FPS::default())
        // light
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(-2.0, 8.0, -1.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(5.0, 10.0, 10.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .with_bundle((
            PlayerMovement::new(),
            rigid_body,
            collider,
        ));
}

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