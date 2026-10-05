use avian2d::PhysicsPlugins;
use bevy::{
    asset::AssetMetaCheck,
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};

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
        .add_systems(Update, screenshot_on_spacebar)
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

fn screenshot_on_spacebar(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut counter: Local<u32>,
) {
    if input.just_pressed(KeyCode::KeyW) {
        let path = format!("./screenshot-{}.png", *counter);
        *counter += 1;
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
    }
}
