use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct Settings{
    pub render_settings: RenderSettings,
    pub game_settings: GameSettings,
}

#[derive(Clone, Hash, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Hash, Debug, Serialize, Deserialize)]
pub struct GameSettings {

}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {

        }
    }
}