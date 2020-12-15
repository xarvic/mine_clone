use crate::settings::Settings;
use bevy::DefaultPlugins;
use crate::player::init_player;
use crate::world::init_world;
use crate::renderer::init_rendering;
use bevy::app::App;

pub fn load_engine(settings: Settings) -> App {

    let mut builder = App::build();
    builder.add_plugins(DefaultPlugins);


    //Add rendering Systems
    init_rendering(&mut builder, &settings);
    //Add World
    init_world(&mut builder, &settings);
    //Add Player
    init_player(&mut builder, &settings);

    builder.app
}