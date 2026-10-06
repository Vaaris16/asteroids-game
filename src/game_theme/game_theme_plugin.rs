use bevy::prelude::*;

use crate::{
    GameState,
    game_theme::{
        game_theme::GameTheme,
        theme_button::{
            cleanup_theme_button, set_theme_icon, spawn_theme_button, theme_button_interactions,
        },
        theme_colors::{ThemeColors, apply_theme_to_new_ui, set_changed_theme},
    },
};

pub struct GameThemePlugin;

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
struct GameThemeSet;

impl Plugin for GameThemePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            GameThemeSet.run_if(in_state(GameState::SplashScreen)),
        );
        app.add_systems(OnEnter(GameState::SplashScreen), spawn_theme_button)
            .add_systems(OnExit(GameState::SplashScreen), cleanup_theme_button)
            .add_systems(Update, theme_button_interactions.in_set(GameThemeSet))
            .add_systems(
                Update,
                (set_changed_theme, apply_theme_to_new_ui, set_theme_icon),
            )
            .insert_resource(ThemeColors::theme_colors(&GameTheme::default()))
            .init_resource::<GameTheme>();
    }
}
