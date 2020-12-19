use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Settings{
    pub render_settings: RenderSettings,
    pub game_settings: GameSettings,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RenderSettings {
    pub msaa_samples: u32
}

impl Default for RenderSettings {
    fn default() -> Self {
        RenderSettings {
            msaa_samples: 4,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameSettings {
    pub print_fps: bool,
    pub print_position: bool,
    pub load_distance: f32,
    pub unload_distance: f32,

}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            print_position: false,
            load_distance: 2.0,
            print_fps: false,
            unload_distance: 4.0,
        }
    }
}