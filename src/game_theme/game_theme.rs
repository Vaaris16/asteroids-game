use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub enum GameTheme {
    Retro,
    #[default]
    Cosmic,
}

impl GameTheme {
    pub fn toggle_game_theme(&self) -> GameTheme {
        match self {
            GameTheme::Retro => GameTheme::Cosmic,
            GameTheme::Cosmic => GameTheme::Retro,
        }
    }
}
