use crate::settings::Settings;
use bevy::DefaultPlugins;
use crate::player::init_player;
use crate::world::init_world;
use crate::renderer::init_rendering;
use bevy::app::App;
use bevy::render::render_graph::base::Msaa;
use bevy_rapier3d::physics::RapierPhysicsPlugin;

pub fn load_engine(settings: Settings) -> App {

    let mut builder = App::build();
    //MSaa hast to be the first resource
    builder.add_resource(Msaa { samples: settings.render_settings.msaa_samples });
    builder.add_plugins(DefaultPlugins);
    builder.add_plugin(RapierPhysicsPlugin);
    builder.add_startup_system(setup_physics.system());

    //Add Player
    init_player(&mut builder, &settings);
    //Add World
    init_world(&mut builder, &settings);
    //Add rendering Systems
    init_rendering(&mut builder, &settings);

    builder.app
}