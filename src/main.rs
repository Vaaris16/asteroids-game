use avian2d::PhysicsPlugins;
use bevy::{asset::AssetMetaCheck, prelude::*};

use crate::{
    core::core_plugin::CorePlugin, game::game_plugin::GamePlugin,
    game_theme::game_theme_plugin::GameThemePlugin, retry::retry_plugin::RetryPlugin,
    splashscreen::splash_screen_plugin::SplashScreenPlugin,
};

mod core;
mod game;
mod game_theme;
mod retry;
mod splashscreen;

pub const BACKGROUND_COLOR: Color = Color::BLACK;

pub const TEXT_COLOR: Color = Color::WHITE;
pub const BORDER_COLOR: Color = Color::WHITE;

#[derive(Default, States, Hash, Eq, Debug, PartialEq, Clone)]
pub enum GameState {
    #[default]
    SplashScreen,
    Game,
    Retry,
}

fn main() {
    App::new()
        .add_systems(Update, mute_all_audio)
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        fit_canvas_to_parent: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..Default::default()
                }),
            PhysicsPlugins::default(),
            CorePlugin,
            GamePlugin,
            SplashScreenPlugin,
            RetryPlugin,
            GameThemePlugin,
        ))
        .init_state::<GameState>()
        .run();
}

fn mute_all_audio(audio_sinks: Query<&mut AudioSink>) {
    for mut sink in audio_sinks {
        sink.mute(); // Or sink.set_volume(0.0);
    }
}
