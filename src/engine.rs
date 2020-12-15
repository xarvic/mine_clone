use bevy::prelude::*;
use crate::renderer::rendering;
use crate::player::player::{PlayerMovement, camera_movement_system, mouse_motion_system, MouseState};
use crate::util::{FPS, print_fps};
use crate::world::chunk::{ChunkRegistry, ChunkPosition, init_chunks, update_chunks};

/// this component indicates what entities should rotate
struct Rotator;

/// rotates the parent, which will result in the child also rotating
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotator>>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_x(3.0 * time.delta_seconds());
    }
}

/// set up a simple scene with a "parent" cube and a "child" cube
fn setup(
    commands: &mut Commands,
    meshes: ResMut<Assets<Mesh>>,
    textures: ResMut<Assets<Texture>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {

    commands
        .insert_resource(MouseState::default())
        .insert_resource(FPS::default())
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(-2.0, 8.0, -1.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(5.0, 10.0, 10.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        }).with(PlayerMovement::new());
}



pub fn load_engine() -> App {
    let mut builder = App::build();

    builder.add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_system(camera_movement_system)
        .add_system(mouse_motion_system)
        .add_system(print_fps)
        .add_startup_system(setup)
        .add_resource(ChunkRegistry::new(
            ChunkPosition::new(0, 1, 0),
            2,
            4,
        ))
        .add_startup_system(init_chunks)
        .add_system(update_chunks);

        //.add_system(rotator_system);

    //Add rendering Systems
    rendering(&mut builder);

    builder.app
}