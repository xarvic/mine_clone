use bevy::prelude::*;
use crate::renderer::rendering;
use crate::player::player::{PlayerMovement, camera_movement_system, mouse_motion_system, MouseState};
use crate::util::{FPS, print_fps};
use crate::world::chunk::{Chunk, ChunkPosition};
use crate::world::block::{GROUND, AIR};
use crate::world::chunk_mesh::insert_rendered_chunk;

/// this component indicates what entities should rotate
struct Rotator;

/// rotates the parent, which will result in the child also rotating
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotator>>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_x(3.0 * time.delta_seconds());
    }
}

fn insert_test_cubes(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 2.0 }));
    let cube_material_handle = materials.add(StandardMaterial {
        albedo: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });

    commands
        .spawn(PbrBundle {
            mesh: cube_handle.clone(),
            material: cube_material_handle.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        })
        .with(Rotator)
        .with_children(|parent| {
            // child cube
            parent.spawn(PbrBundle {
                mesh: cube_handle,
                material: cube_material_handle,
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 3.0)),
                ..Default::default()
            });
        });
}

fn one_chunk(mut commands: &mut Commands,
             mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<StandardMaterial>>) {

    let mut chunk = Chunk::filled(GROUND, ChunkPosition::new(0, 0, 0));

    for (pos, block) in chunk.iter_mut() {
        if (pos.x + pos.y) % 2 == 0 && pos.z == 0 {
            *block = AIR;
        }
    }

    insert_rendered_chunk(commands,
                          &mut meshes,
                          &mut materials,
                          chunk.clone(),
    );

    chunk.position = ChunkPosition::new(1, 1, 0);

    insert_rendered_chunk(commands,
                          &mut meshes,
                          &mut materials,
                          chunk,
    );

}

/// set up a simple scene with a "parent" cube and a "child" cube
fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    one_chunk(commands, meshes, materials);

    commands
        .insert_resource(MouseState::default())
        .insert_resource(FPS::default())
        // parent cube

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
        .add_startup_system(one_chunk)
        .add_startup_system(setup);
        //.add_system(rotator_system);

    //Add rendering Systems
    rendering(&mut builder);

    builder.app
}