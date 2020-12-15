use bevy::app::AppBuilder;
use crate::settings::Settings;
use bevy::render::render_graph::base::Msaa;

pub fn init_rendering(builder: &mut AppBuilder, settings: &Settings) {
    builder.add_resource(Msaa { samples: settings.render_settings.msaa_samples });
}